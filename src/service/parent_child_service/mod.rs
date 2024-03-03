pub mod all_children;

use crate::service::DatabaseServiceTrait;
use crate::PersistenceConnection;

pub struct ParentChildService<D = PersistenceConnection>(D);

impl<D> ParentChildService<D> {}

impl<D> DatabaseServiceTrait<D> for ParentChildService<D> {
    fn with_db(db: D) -> Self {
        Self(db)
    }

    fn db(&self) -> &D {
        &self.0
    }
}
