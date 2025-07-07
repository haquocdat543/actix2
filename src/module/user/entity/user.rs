use chrono::{DateTime, NaiveDate, Utc};
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::user; // use table module, not dsl

#[derive(Queryable, Serialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub dob: Option<NaiveDate>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = user)]
pub struct NewUser {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Queryable, Serialize)]
#[diesel(table_name = user)]
pub struct Users {
    pub email: String,
    pub dob: Option<NaiveDate>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}
