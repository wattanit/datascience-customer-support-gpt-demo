pub use sea_orm_migration::prelude::*;

mod m20240204_120824_create_table;
mod m20240204_130004_create_thread_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240204_120824_create_table::Migration),
            Box::new(m20240204_130004_create_thread_table::Migration),
        ]
    }
}
