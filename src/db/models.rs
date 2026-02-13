use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Blog {
    pub id:      i32,
    pub title:   String,
    pub date:    String,
    pub preview: String,
    pub body:    String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::blogs)]
pub struct NewBlog<'a> {
    pub title:   &'a str,
    pub date:    &'a str,
    pub preview: &'a str,
    pub body:    &'a str,
}