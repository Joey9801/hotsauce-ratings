use sea_orm_migration::{prelude::*, sea_query::Table};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "001_create_user_tables"
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

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
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
