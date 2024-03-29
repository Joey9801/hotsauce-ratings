use sea_orm_migration::{prelude::*, sea_query::Table};

use crate::m001_create_user_tables::User;
use crate::m002_create_sauce_tables::Sauce;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "003_create_ratings_tables"
    }
}

#[derive(Iden)]
pub enum RatingAxis {
    Table,
    Id,
    Name,
    MinValue,
    MaxValue,
    MinValueDesc,
    MaxValueDesc,
}

#[derive(Iden)]
pub enum Review {
    Table,
    Id,
    Sauce,
    User,
    Timestamp,
    Text,
}

#[derive(Iden)]
pub enum ReviewRating {
    Table,
    Id,
    Review,
    RatingAxis,
    Rating,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(RatingAxis::Table)
                    .col(
                        ColumnDef::new(RatingAxis::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(RatingAxis::Name).string().not_null())
                    .col(ColumnDef::new(RatingAxis::MinValue).float().not_null())
                    .col(ColumnDef::new(RatingAxis::MaxValue).float().not_null())
                    .col(ColumnDef::new(RatingAxis::MinValueDesc).string().not_null())
                    .col(ColumnDef::new(RatingAxis::MaxValueDesc).string().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Review::Table)
                    .col(
                        ColumnDef::new(Review::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Review::Sauce).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Review::Table, Review::Sauce)
                            .to(Sauce::Table, Sauce::Id),
                    )
                    .col(ColumnDef::new(Review::User).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Review::Table, Review::User)
                            .to(User::Table, User::Id),
                    )
                    .col(ColumnDef::new(Review::Timestamp).timestamp().not_null())
                    .col(ColumnDef::new(Review::Text).string())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(ReviewRating::Table)
                    .col(
                        ColumnDef::new(ReviewRating::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ReviewRating::Review).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(ReviewRating::Table, ReviewRating::Review)
                            .to(Review::Table, Review::Id),
                    )
                    .col(
                        ColumnDef::new(ReviewRating::RatingAxis)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(ReviewRating::Table, ReviewRating::RatingAxis)
                            .to(RatingAxis::Table, RatingAxis::Id),
                    )
                    .col(ColumnDef::new(ReviewRating::Rating).float().not_null())
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ReviewRating::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Review::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(RatingAxis::Table).to_owned())
            .await?;

        Ok(())
    }
}
