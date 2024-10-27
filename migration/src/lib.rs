pub use sea_orm_migration::prelude::*;

mod alert_create_table;
mod alertconfig_create_table;
mod integration_create_table;
mod location_create_table;
mod locationdata_create_table;
mod session_create_table;
mod trackingstatus_create_table;
mod user_create_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(alert_create_table::Migration),
            Box::new(alertconfig_create_table::Migration),
            Box::new(integration_create_table::Migration),
            Box::new(location_create_table::Migration),
            Box::new(locationdata_create_table::Migration),
            Box::new(session_create_table::Migration),
            Box::new(trackingstatus_create_table::Migration),
            Box::new(user_create_table::Migration),
        ]
    }
}
