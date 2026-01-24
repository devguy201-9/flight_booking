use crate::domain::address::entity::{Address, AddressTypeDomain};
use crate::infrastructure::persistence::seaorm::entities::address::{
    ActiveModel, AddressTypeDb, Model,
};
use sea_orm::ActiveValue::{NotSet, Set};

/// Domain AddressTypeDomain -> ORM AddressTypeDb
impl From<&AddressTypeDomain> for AddressTypeDb {
    fn from(value: &AddressTypeDomain) -> Self {
        match value {
            AddressTypeDomain::Home => AddressTypeDb::HOME,
            AddressTypeDomain::Billing => AddressTypeDb::BILLING,
            AddressTypeDomain::Contact => AddressTypeDb::CONTACT,
            AddressTypeDomain::Other => AddressTypeDb::OTHER,
        }
    }
}

/// ORM AddressTypeDb -> Domain AddressTypeDomain
impl From<AddressTypeDb> for AddressTypeDomain {
    fn from(value: AddressTypeDb) -> Self {
        match value {
            AddressTypeDb::HOME => AddressTypeDomain::Home,
            AddressTypeDb::BILLING => AddressTypeDomain::Billing,
            AddressTypeDb::CONTACT => AddressTypeDomain::Contact,
            AddressTypeDb::OTHER => AddressTypeDomain::Other,
        }
    }
}

pub struct AddressMapper;

impl AddressMapper {
    /// SeaORM Model -> Domain Entity
    pub fn model_to_domain(model: Model) -> Address {
        Address {
            id: model.id,
            user_id: model.user_id,
            title: model.title,
            address_line_1: model.address_line_1,
            address_line_2: model.address_line_2,
            country: model.country,
            city: model.city,
            is_default: model.is_default,
            r#type: model.r#type.into(),
            recipient_name: model.recipient_name,
            postal_code: model.postal_code,
            phone_number: model.phone_number,
            is_deleted: model.is_deleted,
        }
    }

    /// Domain Entity -> SeaORM ActiveModel (for INSERT)
    ///
    /// Usecase: create new address in DB.
    /// - ignore domain.id (domain usually id=0 or unused)
    pub fn domain_to_active_model_for_create(entity: &Address) -> ActiveModel {
        ActiveModel {
            id: NotSet,
            user_id: Set(entity.user_id),
            title: Set(entity.title.clone()),
            address_line_1: Set(entity.address_line_1.clone()),
            address_line_2: Set(entity.address_line_2.clone()),
            country: Set(entity.country.clone()),
            city: Set(entity.city.clone()),
            is_default: Set(entity.is_default),
            r#type: Set((&entity.r#type).into()),
            recipient_name: Set(entity.recipient_name.clone()),
            postal_code: Set(entity.postal_code.clone()),
            phone_number: Set(entity.phone_number.clone()),
            is_deleted: Set(entity.is_deleted),

            ..Default::default()
        }
    }

    /// Domain Entity -> SeaORM ActiveModel (for UPDATE)
    ///
    /// Usecase: update existing address
    pub fn domain_to_active_model_for_update(entity: &Address) -> ActiveModel {
        ActiveModel {
            id: Set(entity.id),
            user_id: Set(entity.user_id),
            title: Set(entity.title.clone()),
            address_line_1: Set(entity.address_line_1.clone()),
            address_line_2: Set(entity.address_line_2.clone()),
            country: Set(entity.country.clone()),
            city: Set(entity.city.clone()),
            is_default: Set(entity.is_default),
            r#type: Set((&entity.r#type).into()),
            recipient_name: Set(entity.recipient_name.clone()),
            postal_code: Set(entity.postal_code.clone()),
            phone_number: Set(entity.phone_number.clone()),
            is_deleted: Set(entity.is_deleted),

            ..Default::default()
        }
    }
}
