pub use sea_orm_migration::prelude::*;

mod m20240128_142240_create_quiz_table;
mod m20240128_142246_create_users_table;
mod m20240128_142254_create_analytics_table;
mod m20240128_142337_create_active_quiz_table;
mod m20240128_142720_create_permissions_table;
mod m20240130_124944_create_user_links_table;
mod m20240130_140620_create_user_refresh_tokens_table;
mod m20240207_233443_create_resource_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240128_142246_create_users_table::Migration),
            Box::new(m20240128_142240_create_quiz_table::Migration),
            // Box::new(m20240128_142254_create_analytics_table::Migration),
            // Box::new(m20240128_142337_create_active_quiz_table::Migration),
            // Box::new(m20240128_142720_create_permissions_table::Migration),
            Box::new(m20240130_124944_create_user_links_table::Migration),
            Box::new(m20240130_140620_create_user_refresh_tokens_table::Migration),
            Box::new(m20240207_233443_create_resource_table::Migration),
        ]
    }
}
