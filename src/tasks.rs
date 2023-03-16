use chrono{serde::ts_seconds, DateTime, Local, Utc};
use serde::Serialize;
use serde::Deserialize;

#[derive(Deserialize, Serialize, Debug)]
pub struct Task{
    pub text : String,
    pub created_at : DateTime<Utc>,
}

impl Task{
    pub fn new(text: String) -> Task {
        let created_at = Utc::now();
        Task{text,created_at}
    }
}
