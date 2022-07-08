use sea_orm_migration::{prelude::*, sea_query::Table};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "002_create_user_tables"
    }
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

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
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

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Sauce::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Manufacturer::Table).to_owned())
            .await?;

        Ok(())
    }
}