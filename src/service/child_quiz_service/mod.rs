use crate::service::DatabaseServiceTrait;
use crate::PersistenceConnection;

mod child_statical;
mod new_ans_record;
pub mod next_quiz;
pub mod quiz_record;

pub struct ChildQuizService<D = PersistenceConnection>(D);

impl<D> DatabaseServiceTrait<D> for ChildQuizService<D> {
    fn with_db(db: D) -> Self {
        Self(db)
    }

    fn db(&self) -> &D {
        &self.0
    }
}
