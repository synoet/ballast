use crate::output::Output;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

const SNAPSHOT_PATH: &str = "./.ballast_snapshot.json";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Snapshot {
    pub outputs: Vec<Output>,
    pub timestamp: u64,
}

impl Snapshot {
    pub fn new(outputs: Vec<Output>) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Failed to get timestamp")
            .as_secs();

        Self { outputs, timestamp }
    }

    pub fn read() -> Vec<Snapshot> {
        let snapshots: Vec<Snapshot> = match std::fs::read_to_string(SNAPSHOT_PATH) {
            Ok(s) => serde_json::from_str(&s).unwrap(),
            Err(_) => Vec::new(),
        };

        return snapshots;
    }

    pub fn write(&self) {
        let mut snapshots: Vec<Snapshot> = match std::fs::read_to_string(SNAPSHOT_PATH) {
            Ok(s) => serde_json::from_str(&s).unwrap(),
            Err(_) => Vec::new(),
        };
        snapshots.push(self.clone());
        let snapshot_json = serde_json::to_string(&snapshots).unwrap();
        std::fs::write(SNAPSHOT_PATH, snapshot_json).unwrap();
    }

    pub fn latest() -> Option<Self> {
        let mut snapshots: Vec<Snapshot> = match std::fs::read_to_string(SNAPSHOT_PATH) {
            Ok(s) => serde_json::from_str(&s).unwrap(),
            Err(_) => Vec::new(),
        };

        if snapshots.len() == 0 {
            return None;
        }

        snapshots.sort_by_key(|s| s.timestamp);

        return Some(snapshots.last().unwrap().clone());
    }
}
