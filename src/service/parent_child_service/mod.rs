mod all_children;

use crate::service::DatabaseServiceTrait;

pub struct ParentChildService<D>(D);

impl<D> ParentChildService<D> {}

impl<D> DatabaseServiceTrait<D> for ParentChildService<D> {
    fn with_db(db: D) -> Self {
        Self(db)
    }

    fn db(&self) -> &D {
        &self.0
    }
}
