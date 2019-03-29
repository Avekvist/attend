use crate::schema::class;
use crate::helper::ClassTime;
use crate::models::teacher::ClassTeacher;

#[derive(Queryable, Serialize)]
pub struct Class {
    pub class_id: i32,
    pub class_name: String,
}

#[table_name = "class"]
#[derive(Insertable)]
pub struct ClassCreate {
    pub class_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct ClassJSON {
    pub id: i32,
    pub name: String,
    pub time: ClassTime,
    pub teachers: Vec<ClassTeacher>,
}
