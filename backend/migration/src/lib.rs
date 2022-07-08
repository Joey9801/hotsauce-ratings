pub use sea_orm_migration::prelude::*;

mod m001_create_user_tables;
mod m002_create_sauce_tables;
mod m003_create_ratings_tables;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m001_create_user_tables::Migration),
            Box::new(m002_create_sauce_tables::Migration),
            Box::new(m003_create_ratings_tables::Migration),
        ]
    }
}
