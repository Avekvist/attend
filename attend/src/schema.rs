table! {
    attendance (attendance_id) {
        attendance_id -> Integer,
        class_id -> Integer,
        attendee_id -> Integer,
        attendance_date -> Varchar,
        attendance_time -> Varchar,
    }
}

table! {
    attendee (attendee_id) {
        attendee_id -> Integer,
        tag_id -> Varchar,
        attendee_name -> Varchar,
    }
}

table! {
    attendeeclassbridge (attendee_class_bridge_id) {
        attendee_class_bridge_id -> Integer,
        attendee_id -> Integer,
        class_id -> Integer,
    }
}

table! {
    class (class_id) {
        class_id -> Integer,
        class_name -> Varchar,
    }
}

table! {
    tag (tag_id) {
        tag_id -> Varchar,
        tag_date -> Datetime,
    }
}

table! {
    teacher (username) {
        username -> Varchar,
        password -> Varchar,
        teacher_name -> Varchar,
        is_admin -> Bool,
    }
}

table! {
    teacherclassbridge (teacher_class_bridge_id) {
        teacher_class_bridge_id -> Integer,
        username -> Varchar,
        class_id -> Integer,
    }
}

allow_tables_to_appear_in_same_query!(
    attendance,
    attendee,
    attendeeclassbridge,
    class,
    tag,
    teacher,
    teacherclassbridge,
);
