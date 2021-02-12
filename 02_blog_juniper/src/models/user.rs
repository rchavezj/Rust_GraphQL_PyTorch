/// User model
/// Includes login and profile info
use uuid::Uuid;
use chrono::NaiveDateTime;
use juniper::{GraphQLInputObject};
use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;



#[derive(Clone, Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table = "users")]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
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
