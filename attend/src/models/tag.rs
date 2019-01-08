use crate::schema::tag;

#[table_name = "tag"]
#[derive(Queryable, Insertable)]
pub struct Tag {
    pub tag_id: String,
    pub tag_date: chrono::NaiveDateTime,
}
