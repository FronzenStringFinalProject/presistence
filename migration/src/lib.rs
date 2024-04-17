pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_parent_table;
mod m20240207_181705_create_children_table;
mod m20240207_183220_create_quiz_table;
mod m20240207_184348_create_answer_record_table;
mod m20240209_142558_insert_records;
mod m20240213_020340_add_parent_authorize;
mod m20240215_022426_parent_add_pwd_version;
mod m20240314_033251_fix_quiz_ploblem;
mod m20240314_090754_child_check_create;
mod m20240325_022626_child_select_quiz_group_crate;
mod m20240415_100440_update_safty_pwd;
mod m20240417_005652_ans_score;
mod m20240417_020951_update_ans_score;

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
            Box::new(m20240213_020340_add_parent_authorize::Migration),
            Box::new(m20240215_022426_parent_add_pwd_version::Migration),
            Box::new(m20240314_033251_fix_quiz_ploblem::Migration),
            Box::new(m20240314_090754_child_check_create::Migration),
            Box::new(m20240325_022626_child_select_quiz_group_crate::Migration),
            Box::new(m20240415_100440_update_safty_pwd::Migration),
            Box::new(m20240417_005652_ans_score::Migration),
            Box::new(m20240417_020951_update_ans_score::Migration),
        ]
    }
}
