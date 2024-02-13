pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_parent_table;
mod m20240207_181705_create_children_table;
mod m20240207_183220_create_quiz_table;
mod m20240207_184348_create_answer_record_table;
mod m20240209_142558_insert_records;
mod m20240213_020340_add_parent_authorize;


pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_parent_table::Migration),
            Box::new(m20240207_181705_create_children_table::Migration),
            Box::new(m20240207_183220_create_quiz_table::Migration),
            Box::new(m20240207_184348_create_answer_record_table::Migration),
            Box::new(m20240209_142558_insert_records::Migration),
            Box::new(m20240213_020340_add_parent_authorize::Migration)
        ]
    }
}
