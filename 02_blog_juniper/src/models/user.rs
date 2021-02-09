/// User model
/// Includes login and profile info
use uuid::Uuid;
use argonautica::Hasher;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use futures::compat::Future01CompatExt;
use tokio_pg_mapper_derive::PostgresMapper;
use juniper::{GraphQLObject, GraphQLInputObject};


#[derive(Clone, Serialize, Deserialize, PostgresMapper, GraphQLObject)]
#[pg_mapper(table = "users")]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    #[graphql(skip)]
    pub password: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}


#[derive(GraphQLInputObject)]
pub struct CreateUser{
    pub username: String,
    pub email: String,
    pub password: String,
    pub bio: Option<String>,
    pub image: Option<String>
}
