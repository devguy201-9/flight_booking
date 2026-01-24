use axum::{
    Json,
    body::Body,
    extract::State,
    http::{Request, Response},
};

use crate::{
    core::{app_state::AppState, response::common::EntityResponse, user_context::UserContext},
    infrastructure::{
        config::service_registry::ServiceConfig,
        gateway::proxy::{ProxyClient, check_service_health},
    },
    presentation::{
        gateway::dto::{GatewayHealth, ServiceHealth},
        http::error::{AppResult, HttpError},
    },
};

use log::info;

#[utoipa::path(
    get,
    path = "/gateway/health",
    tag = "Gateway",
    responses(
        (status = 200, description = "Gateway health status", body = EntityResponse<GatewayHealth>)
    )
)]
pub async fn gateway_health_check(
    State(state): State<AppState>,
) -> AppResult<Json<EntityResponse<GatewayHealth>>> {
    let services = state.gateway_registry.list_all().await;
    let client = reqwest::Client::new();

    let mut service_healths = Vec::new();
    let mut all_healthy = true;

    for service in services {
        let healthy = check_service_health(
            &client,
            &service.base_url,
            service.health_check_path.as_deref(),
        )
        .await;

        if !healthy {
            all_healthy = false;
        }

        service_healths.push(ServiceHealth {
            name: service.name.clone(),
            base_url: service.base_url.clone(),
            healthy,
        });
    }

    let status = if all_healthy { "healthy" } else { "degraded" };

    Ok(Json(EntityResponse {
        message: "Gateway health check".to_string(),
        data: Some(GatewayHealth {
            status: status.to_string(),
            services: service_healths,
        }),
        total: 1,
    }))
}

#[utoipa::path(
    get,
    path = "/gateway/services",
    tag = "Gateway",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "List of services", body = EntityResponse<Vec<ServiceConfig>>),
        (status = 401, description = "Unauthorized"),
    )
)]
pub async fn list_services(
    State(state): State<AppState>,
    ctx: UserContext,
) -> AppResult<Json<EntityResponse<Vec<ServiceConfig>>>> {
    info!("User {} listing gateway services", ctx.user_id);

    let services = state.gateway_registry.list_all().await;

    Ok(Json(EntityResponse {
        message: "Services retrieved successfully".to_string(),
        total: services.len() as i64,
        data: Some(services),
    }))
}

/// Core proxy function
async fn proxy_to_service(
    service_name: &str,
    state: &AppState,
    ctx: Option<&UserContext>,
    request: Request<Body>,
) -> AppResult<Response<Body>> {
    let service_config = state
        .gateway_registry
        .get(service_name)
        .await
        .ok_or_else(|| HttpError::EntityNotFound {
            detail: format!("Service '{}' not found", service_name),
        })?;

    if service_config.require_auth && ctx.is_none() {
        return Err(HttpError::Unauthorized);
    }

    // infra proxy client => TechnicalError -> HttpError bằng From
    let proxy_client = ProxyClient::new(service_config.timeout_secs)?;

    let resp = proxy_client
        .forward_request(
            &service_config,
            request,
            ctx.as_ref().map(|c| c.user_id),
            ctx.as_ref().map(|c| c.session_id.clone().to_string()),
        )
        .await?;

    Ok(resp)
}

// --------------
// Proxy endpoints
// --------------

pub async fn proxy_to_product_service(
    State(state): State<AppState>,
    ctx: UserContext,
    req: axum::extract::Request,
) -> AppResult<Response<Body>> {
    proxy_to_service("product-service", &state, Some(&ctx), req).await
}

pub async fn proxy_to_order_service(
    State(state): State<AppState>,
    ctx: UserContext,
    req: axum::extract::Request,
) -> AppResult<Response<Body>> {
    proxy_to_service("product-service", &state, Some(&ctx), req).await
}

pub async fn proxy_to_inventory_service(
    State(state): State<AppState>,
    ctx: UserContext,
    req: axum::extract::Request,
) -> AppResult<Response<Body>> {
    proxy_to_service("inventory-service", &state, Some(&ctx), req).await
}

pub async fn proxy_to_notification_service(
    State(state): State<AppState>,
    ctx: UserContext,
    req: axum::extract::Request,
) -> AppResult<Response<Body>> {
    proxy_to_service("notification-service", &state, Some(&ctx), req).await
}
