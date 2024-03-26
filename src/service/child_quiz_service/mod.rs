use crate::service::DatabaseServiceTrait;
use crate::PersistenceConnection;

pub mod child_statical;
pub mod fetch_quiz_group;
pub mod new_ans_record;
pub mod next_quiz;
pub mod quiz_record;
mod wrong_records;

pub struct ChildQuizService<D = PersistenceConnection>(D);

impl<D> DatabaseServiceTrait<D> for ChildQuizService<D> {
    fn with_db(db: D) -> Self {
        Self(db)
    }

    fn db(&self) -> &D {
        &self.0
    }
}
