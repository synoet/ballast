use crate::output::Output;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

const SNAPSHOT_PATH: &str = "./.ballast_snapshot.json";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Snapshot {
    pub outputs: Vec<Output>,
    pub timestamp: u64,
}

impl Snapshot {
    pub fn new(outputs: Vec<Output>) -> Result<Self> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .context("Failed to get timestamp")?
            .as_secs();

        Ok(Self { outputs, timestamp })
    }

    pub fn read() -> Result<Vec<Snapshot>> {
        let snapshots: Vec<Snapshot> = match std::fs::read_to_string(SNAPSHOT_PATH) {
            Ok(s) => serde_json::from_str(&s).context("Failed to parse .ballast_snapshot.json, where changes manually added?")?,
            Err(_) => Vec::new(),
        };

        return Ok(snapshots);
    }

    pub fn write(&self) -> Result<()> {
        let mut snapshots = Self::read()?;
        snapshots.push(self.clone());
        let snapshot_json = serde_json::to_string(&snapshots)
            .context("Failed to parse .ballast_snapshot.json, where changes manually added?")?;
        std::fs::write(SNAPSHOT_PATH, snapshot_json)
            .context("Failed to write to .ballast_snapshot.json")?;

        Ok(())
    }

    pub fn latest() -> Result<Option<Self>> {
        let mut snapshots = Self::read()?;

        if snapshots.len() == 0 {
            return Ok(None);
        }

        snapshots.sort_by_key(|s| s.timestamp);

        Ok(Some(snapshots.last().unwrap().clone()))
    }
}
