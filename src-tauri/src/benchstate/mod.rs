use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::Serialize;

#[derive(Debug,Serialize)]
pub struct Statistic {
    pub name: String,
    pub rss: u64,
    pub mem: f32,
    pub vsz: u64,
    pub cpu: f32,
    pub ni: i32,
}

impl fmt::Display for Statistic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#"        {{
          "name": "{}", "rss": {}, "mem": {}, "vsz": {}, "cpu": {}, "ni": {}
        }}"#, self.name, self.rss, self.mem, self.vsz, self.cpu, self.ni)
    }
}


#[derive(Debug,Serialize)]
pub struct StatisticsEntry {
    pub time: u64,
    pub stats: Vec<Statistic>,
}

impl fmt::Display for StatisticsEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#"    {{
      "time": {},
      "stats": [
"#, self.time)?;
        for (i, stat) in self.stats.iter().enumerate() {
            if i != 0 { write!(f, ",\n")?; }
            write!(f, "{}", stat)?;
        }
        write!(f, "\n      ]\n    }}")
    }
}

#[derive(Debug,Serialize)]
pub struct Statistics {
    pub entries: Vec<StatisticsEntry>,
}

impl fmt::Display for Statistics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#""statistics": ["#)?;
        for (i, entry) in self.entries.iter().enumerate() {
            if i != 0 { write!(f, ",\n")?; }
            write!(f, "{}", entry)?;
        }
        write!(f, "\n]")
    }
}

pub struct BenchState {
    pub statistics: Statistics,
}

impl BenchState {
    pub fn new() -> Self {
        Self {
            statistics: Statistics { entries: Vec::new() },
        }
    }

    pub fn add_statistic(&mut self, stat: Statistic) {
        // Update or create a new entry with the current timestamp
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        if let Some(last_entry) = self.statistics.entries.last_mut() {
            if last_entry.time == current_time {
                // Add to the current entry if it matches the timestamp
                last_entry.stats.push(stat);
            } else {
                // Otherwise, create a new entry
                self.statistics.entries.push(StatisticsEntry {
                    time: current_time,
                    stats: vec![stat],
                });
            }
        } else {
            // If no entries exist, create the first one
            self.statistics.entries.push(StatisticsEntry {
                time: current_time,
                stats: vec![stat],
            });
        }
    }
}

impl Default for BenchState {
    fn default() -> Self {
        Self::new()
    }
}
