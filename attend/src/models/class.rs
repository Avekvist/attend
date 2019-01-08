use crate::schema::class;

#[derive(Queryable)]
pub struct Class {
    pub class_id: i32,
    pub class_name: String,
}

#[table_name = "class"]
#[derive(Insertable)]
pub struct ClassCreate {
    pub class_name: String,
}
