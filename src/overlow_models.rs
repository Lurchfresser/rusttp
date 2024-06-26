use diesel::prelude::*;
use chrono::NaiveDateTime;

use crate::schema::overflows;


#[derive(Insertable)]
#[diesel(table_name = overflows)]
#[diesel(treat_none_as_default_value = true)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewOverflow <'a>{
    pub title: &'a str,
    pub author: &'a str,
    pub body: &'a str,
    pub rating: i32,
    pub overflow_id: i32,
    pub published: NaiveDateTime,
}



#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = overflows)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Overflow {
    pub id: i32,
    pub title: String,
    pub author: String,
    pub body: String,
    pub rating: i32,
    pub overflow_id: i32,
    pub published: NaiveDateTime,
    pub deleted: bool,
}






impl<'a> NewOverflow<'a> {
    pub fn from_now(title: &'a str, author: &'a str, body: &'a str, overflow_id: i32, rating: i32) -> Self {
        Self {
            title,
            author,
            body,
            overflow_id,
            published: chrono::Local::now().naive_local(),
            rating,
        }
    }
}