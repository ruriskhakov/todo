use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Task {
    pub id: u32,
    pub name: String,
    pub created: DateTime<Utc>,
    pub ended: DateTime<Utc>,
}

impl Task {
    pub(crate) fn new(id: u32, name: &str) -> Task {
        Task {
            id: id,
            name: name.to_lowercase(),
            created: Utc::now(),
            ended: Utc::now(),
        }
    }
}
