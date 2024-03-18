use async_graphql::{Context, Object, Schema};
use async_graphql::{EmptyMutation, EmptySubscription};

pub(crate) type UserSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub(crate) struct QueryRoot;


#[Object]
impl QueryRoot {
    async fn user(&self, _ctx: &Context<'_>) -> String {
        "Hello, user!".to_string()
    }
}
