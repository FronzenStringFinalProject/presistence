use crate::service::DatabaseServiceTrait;
use crate::PersistenceConnection;

mod child_scrod;

pub struct ChildSocialService<D = PersistenceConnection>(D);

impl<D> DatabaseServiceTrait<D> for ChildSocialService<D> {
    fn with_db(db: D) -> Self {
        Self(db)
    }

    fn db(&self) -> &D {
        &self.0
    }
}
pub use child_scrod::ChildScore;
