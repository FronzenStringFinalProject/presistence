mod child_quiz_service;
mod parent_child_service;

use async_trait::async_trait;
use axum_core::extract::{FromRef, FromRequestParts};
use http::request::Parts;
use std::convert::Infallible;

pub use child_quiz_service::ChildQuizService;
pub use parent_child_service::ParentChildService;

pub trait DatabaseServiceTrait<D> {
    fn with_db(db: D) -> Self;

    fn db(&self) -> &D;
}

pub struct DbService<S>(pub S);

#[async_trait]
impl<S, State> FromRequestParts<State> for DbService<S>
where
    crate::PersistenceConnection: FromRef<State>,
    S: DatabaseServiceTrait<crate::PersistenceConnection>,
    State: Send + Sync,
{
    type Rejection = Infallible;

    async fn from_request_parts(_: &mut Parts, state: &State) -> Result<Self, Self::Rejection> {
        let db = FromRef::from_ref(state);
        Ok(Self(S::with_db(db)))
    }
}
