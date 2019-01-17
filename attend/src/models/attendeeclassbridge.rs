use crate::schema::attendeeclassbridge;

#[derive(Queryable)]
pub struct AttendeeClassBridge {
    pub attendee_class_bridge_id: i32,
    pub attendee_id: i32,
    pub class_id: i32,
}

#[table_name = "attendeeclassbridge"]
#[derive(Insertable)]
pub struct AttendeeClassBridgeCreate {
    pub attendee_id: i32,
    pub class_id: i32,
}
