use sea_orm_migration::{manager, prelude::*, sea_query::Table};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220623_000001_create_tables"
    }
}

#[derive(Iden)]
pub enum User {
    Table,
    Id,
    Username,
}

#[derive(Iden)]
pub enum UsedNonce {
    Table,
    Nonce,
    User,
    UsedAt,
}

#[derive(Iden)]
pub enum UserGoogleLogin {
    Table,
    User,
    GoogleId,
}

#[derive(Iden)]
pub enum Manufacturer {
    Table,
    Id,
    Name,
}

#[derive(Iden)]
pub enum Sauce {
    Table,
    Id,
    Name,
    Manufacturer,
}

#[derive(Iden)]
pub enum RatingAxis {
    Table,
    Id,
    Name,
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
                    .table(User::Table)
                    .col(
                        ColumnDef::new(User::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(User::Username)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(UsedNonce::Table)
                    .col(ColumnDef::new(UsedNonce::Nonce).string().not_null())
                    .col(ColumnDef::new(UsedNonce::User).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(UsedNonce::Table, UsedNonce::User)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(UsedNonce::UsedAt).timestamp_with_time_zone().not_null())
                    .primary_key(Index::create().col(UsedNonce::Nonce).col(UsedNonce::User))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(UserGoogleLogin::Table)
                    .col(
                        ColumnDef::new(UserGoogleLogin::GoogleId)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(UserGoogleLogin::User).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(UserGoogleLogin::Table, UserGoogleLogin::User)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Manufacturer::Table)
                    .col(
                        ColumnDef::new(Manufacturer::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Manufacturer::Name).string().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Sauce::Table)
                    .col(
                        ColumnDef::new(Sauce::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Sauce::Name).string().not_null())
                    .col(ColumnDef::new(Sauce::Manufacturer).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Sauce::Table, Sauce::Manufacturer)
                            .to(Manufacturer::Table, Manufacturer::Id),
                    )
                    .to_owned(),
            )
            .await?;

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

        manager
            .drop_table(Table::drop().table(Sauce::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Manufacturer::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(UserGoogleLogin::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(UsedNonce::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;

        Ok(())
    }
}
