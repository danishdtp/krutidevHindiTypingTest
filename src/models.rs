use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Duration {
    Seconds15,
    Seconds30,
    Minute1,
}

impl Duration {
    pub fn word_count(&self) -> u32 {
        match self {
            Duration::Seconds15 => 15,
            Duration::Seconds30 => 30,
            Duration::Minute1 => 60,
        }
    }

    pub fn as_secs(&self) -> u64 {
        match self {
            Duration::Seconds15 => 15,
            Duration::Seconds30 => 30,
            Duration::Minute1 => 60,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Duration::Seconds15 => "15s",
            Duration::Seconds30 => "30s",
            Duration::Minute1 => "1m",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Score {
    pub id: Option<i64>,
    pub wpm: i32,
    pub acc: i32,
    pub date: String,
}
