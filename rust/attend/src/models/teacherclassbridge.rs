use crate::schema::teacherclassbridge;

#[derive(Queryable)]
pub struct TeacherClassBridge {
    pub teacher_class_bridge_id: i32,
    pub username: String,
    pub class_id: i32,
}

#[table_name = "teacherclassbridge"]
#[derive(Insertable)]
pub struct TeacherClassBridgeCreate {
    pub username: String,
    pub class_id: i32,
}
