use crate::schema::attendance;

#[derive(Queryable)]
pub struct Attendance {
    pub attendance_id: i32,
    pub class_id: i32,
    pub attendee_id: i32,
    pub attendance_date: String,
    pub attendance_time: String,
}

#[table_name = "attendance"]
#[derive(Insertable)]
pub struct AttendanceCreate {
    pub class_id: i32,
    pub attendee_id: i32,
    pub attendance_date: String,
    pub attendance_time: String,
}
