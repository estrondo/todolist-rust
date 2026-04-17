pub use sea_orm_migration::prelude::*;

mod m001_create_todo_table;
mod m002_create_todo_permission_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m001_create_todo_table::Migration),
            Box::new(m002_create_todo_permission_table::Migration),
        ]
    }
}
