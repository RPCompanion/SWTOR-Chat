
use tracing::error;
use serde::{Deserialize, Serialize};
use reqwest::blocking::Client;

use crate::config::config;

#[derive(Serialize, Deserialize)]
pub struct CrashReporter {
    pub crash_report: String
}

impl CrashReporter {

    pub fn new(crash_report: String) -> Self {

        Self {
            crash_report: crash_report
        }

    }

    pub fn submit(&self) {

        let client = Client::new();
        let response = client.post(config().crash_reporter_url.as_str())
            .header("Content-Type", "application/json")
            .body(self.as_json())
            .send();

        if let Err(e) = response {
            error!("Error submitting crash report: {:?}", e);
        }

    }

    pub fn as_json(&self) -> String {

        serde_json::to_string(self).unwrap()

    }

}