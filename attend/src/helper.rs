#[derive(Serialize, Deserialize)]
pub struct ClassTime {
    pub date: String,
    pub start: String,
    pub end: String,
}

impl std::fmt::Display for ClassTime {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "Date: {} | Start time: {} | End time: {}", self.date, self.start, self.end)
    }
}
