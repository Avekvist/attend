pub mod attendance;
pub mod classes;
pub mod index;
pub mod operations;
pub mod students;
pub mod support;
pub mod tags;

#[derive(Serialize)]
pub struct Context<T> {
    pub logged_in: bool,
    pub data: T,
}
