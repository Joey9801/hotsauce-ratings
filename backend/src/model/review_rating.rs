//! SeaORM Entity. Generated by sea-orm-codegen 0.8.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "review_rating")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub rating_id: i32,
    pub review_id: i32,
    pub rating_axis_id: i32,
    pub rating: Decimal,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::rating_axis::Entity",
        from = "Column::RatingAxisId",
        to = "super::rating_axis::Column::RatingAxisId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    RatingAxis,
    #[sea_orm(
        belongs_to = "super::review::Entity",
        from = "Column::ReviewId",
        to = "super::review::Column::ReviewId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Review,
}

impl Related<super::rating_axis::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RatingAxis.def()
    }
}

impl Related<super::review::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Review.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
