use async_graphql::{Context, Object, Schema};
use async_graphql::{EmptyMutation, EmptySubscription};

pub(crate) type UserSchema = Schema<Query, EmptyMutation, EmptySubscription>;

pub(crate) struct Query;


#[Object]
impl Query {
    async fn user(&self, ctx: &Context<'_>) -> String {
        "Hello, user!".to_string()
    }
}
