use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Status {
    New,
    Finished,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Status::New => write!(f, "new"),
            Status::Finished => write!(f, "finished"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Task {
    pub id: u32,
    pub name: String,
    pub created: DateTime<Utc>,
    pub ended: DateTime<Utc>,
    pub status: Status,
}

impl Task {
    pub(crate) fn new(id: u32, name: &str) -> Task {
        Task {
            id: id,
            name: name.to_lowercase(),
            created: Utc::now(),
            ended: Utc::now(),
            status: Status::New,
        }
    }
}
