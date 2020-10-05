use juniper::{EmptyMutation, EmptySubscription, FieldResult, IntoFieldError, RootNode};

use super::bragi;
use crate::state;

#[derive(Debug, Clone)]
pub struct Context {
    pub state: state::State,
}

impl juniper::Context for Context {}

pub struct Query;

#[juniper::graphql_object(
    Context = Context
)]
impl Query {
    /// Return a list of all features
    async fn autocomplete(
        &self,
        request: bragi::AutocompleteRequestBody,
        context: &Context,
    ) -> FieldResult<bragi::AutocompleteResponseBody> {
        let bragi_autocomplete_url = format!(
            "http://{}:{}/autocomplete",
            context.state.settings.bragi.host, context.state.settings.bragi.port
        );
        bragi::autocomplete(&request, &bragi_autocomplete_url)
            .await
            .map_err(IntoFieldError::into_field_error)
    }
}

type Schema = RootNode<'static, Query, EmptyMutation<Context>, EmptySubscription<Context>>;

pub fn schema() -> Schema {
    Schema::new(Query, EmptyMutation::new(), EmptySubscription::new())
}
