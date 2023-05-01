pub use sea_orm_migration::prelude::*;

mod m20230501_000001_create_post_table;
mod m20230501_000002_create_comment_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230501_000001_create_post_table::Migration),
            Box::new(m20230501_000002_create_comment_table::Migration),
        ]
    }
}
