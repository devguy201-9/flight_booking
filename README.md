# Flight Booking Backend (Rust)

Backend service for airline-style booking flows, implemented with Clean Architecture + Domain-Driven Design.

This project now includes full wiring for:
- `user`, `auth`, `address`
- `airport`, `flight`, `booking`, `passenger`, `checkin`, `boarding_pass`

---

## Highlights

- Rust `edition = 2024`
- Axum HTTP API with OpenAPI generation (`utoipa`)
- SeaORM repositories (PostgreSQL)
- Redis cache-aside in application services
- Kafka event publishers for all business domains
- Request-context based authorization with admin/user checks
- Optimistic locking on mutable aggregates (`flight`, `booking`, `passenger`, `checkin`)

---

## Project Structure

```text
src/
├── domain/          # Entities, value rules, domain errors, repository interfaces, domain events
├── application/     # Commands, views, mappers, use-case services + service interfaces
├── presentation/    # Request/serializer DTOs, HTTP error mapping
├── api/             # Controller endpoints + route registration
├── infrastructure/  # SeaORM repos, Kafka publishers, Redis/JWT/bootstrap/runtime
└── core/            # AppState, request context, shared response/config contracts
```

Dependency direction:

`api/presentation -> application -> domain <- infrastructure`

---

## Implemented Domain Modules

### Base modules
- `user`: register/verify/login/profile/admin management
- `auth`: login, refresh, logout
- `address`: user addresses with audit-aware persistence

### Flight booking modules
- `airport`: create/update/get/list/deactivate (soft delete)
- `flight`: create/update/search/get/cancel with flight status transitions
- `booking`: create/confirm/cancel/get/list/update payment status
- `passenger`: add/update/remove/list passenger by booking rules
- `checkin`: create/update/cancel/list checkins, checkin window validation
- `boarding_pass`: issue/get/list boarding passes, immutable after issue

---

## API Route Map

Public:
- `/v1/server/*`
- `/api/v1/auth/*` (login, refresh)
- `/api/v1/users/*` (register, verify email)

Protected:
- `/api/v1/auth/*`
- `/api/v1/users/*`
- `/api/v1/addresses/*`
- `/api/v1/airports/*`
- `/api/v1/flights/*`
- `/api/v1/bookings/*`
- `/api/v1/passengers/*`
- `/api/v1/checkins/*`
- `/api/v1/boarding-passes/*`

Swagger UI:
- `http://localhost:<PORT>/swagger-ui`
- OpenAPI JSON: `http://localhost:<PORT>/api-docs/openapi.json`

---

## AppState Wiring (Current)

`AppState` includes:
- Infrastructure handles: `db`, `deploy_mode`, `ctx_provider`, `gateway_registry`
- Service handles: `user_service`, `auth_service`, `address_service`, `airport_service`, `flight_service`, `booking_service`, `passenger_service`, `checkin_service`, `boarding_pass_service`

Bootstrap flow (`AppStateBuilder`):
1. Build DB/Redis/Kafka/context/gateway resources
2. Build cache + repositories + token/password adapters
3. Build Kafka event publishers
4. Build all application services
5. Assemble `AppState`

---

## Event Publishing

Kafka publishers are implemented for:
- `user`, `address`
- `airport`, `flight`, `booking`, `passenger`, `checkin`, `boarding_pass`

Each module uses typed event structs in `src/domain/<module>/events/` and trait-driven publishers in `src/application/common/event_publisher.rs`.

---

## Data + Persistence

- ORM: SeaORM
- Migrations: `migration/`
- Core tables:
  - `users`
  - `addresses`
  - `airports`
  - `flights`
  - `bookings`
  - `passengers`
  - `checkins`
  - `boarding_passes`

---

## Prerequisites

- Rust (stable toolchain)
- Docker + Docker Compose
- PostgreSQL, Redis, Kafka (via docker compose is easiest)

For Windows builds, C/C++ toolchain and CMake are required (for some native crate dependencies).

---

## Configuration

Create `.env` (example values):

```env
DATABASE_URL=postgres://postgres:postgres@localhost:5432/flight_db
REDIS_URL=redis://localhost:6379
KAFKA_BROKERS=localhost:9092
JWT_SECRET=replace_me
```

Do not commit real credentials.

---

## Run Locally

```bash
# 1) Start infra services
docker-compose up -d postgres redis kafka

# 2) Run migrations
cargo run -p migration

# 3) Start server
cargo run --bin flight-booking
```

Windows note (if native crypto build fails):

```powershell
$env:AWS_LC_SYS_C_STD = "11"
$env:AWS_LC_SYS_NO_ASM = "1"
cargo check
```

---

## Useful Commands

```bash
# format + lint (if configured in your environment)
cargo fmt
cargo clippy

# compile check
cargo check

# run app
cargo run --bin flight-booking
```

---

## Current Status

- Full route registration for all 6 booking modules is done.
- `AppState` and bootstrap wiring are updated for all new services/repositories/publishers.
- Kafka publisher implementations exist for every module.
- `cargo check` passes in current environment (with warnings).

---

## Known Gaps

- No full automated test suite yet (unit/integration/e2e).
- Some warnings still exist (unused imports/variables, deprecated API use).
- Kafka consumers/processors are not implemented yet (publishers only).
- Pagination/filtering strategy is basic in some list endpoints.
