use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "staff")]
#[graphql(complex)]
#[graphql(name = "Staff")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub staff_id: i32,
    pub first_name: String,
    pub last_name: String,
    pub address_id: i32,
    pub reports_to_id: Option<i32>,
    pub picture: Option<Vec<u8>>,
    pub email: Option<String>,
    pub store_id: i32,
    pub active: i8,
    pub username: String,
    pub password: Option<String>,
    pub last_update: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::address::Entity",
        from = "Column::AddressId",
        to = "super::address::Column::AddressId",
        on_update = "Cascade",
        on_delete = "Restrict"
    )]
    Address,
    #[sea_orm(
        belongs_to = "super::store::Entity",
        from = "Column::StoreId",
        to = "super::store::Column::StoreId",
        on_update = "Cascade",
        on_delete = "Restrict"
    )]
    Store,
    #[sea_orm(
        belongs_to = "Entity",
        from = "Column::ReportsToId",
        to = "Column::StaffId",
        on_update = "Restrict",
        on_delete = "Restrict"
    )]
    SelfRef,
    #[sea_orm(has_many = "super::payment::Entity")]
    Payment,
    #[sea_orm(has_many = "super::rental::Entity")]
    Rental,
}

impl Related<super::address::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Address.def()
    }
}

impl Related<super::store::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Store.def()
    }
}

impl Related<super::payment::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Payment.def()
    }
}

impl Related<super::rental::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Rental.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
