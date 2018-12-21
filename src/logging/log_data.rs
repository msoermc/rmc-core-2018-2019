use chrono::DateTime;
use chrono::Utc;

use crate::comms::SendableMessage;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum LogType {
    Debug,
    Info,
    Warning,
    Error,
    Fatal,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LogData {
    severity: LogType,
    timestamp: DateTime<Utc>,
    description: String,
}

impl LogData {
    pub fn get_severity(&self) -> &LogType {
        &self.severity
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    pub fn get_timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }

    pub fn to_string(&self) -> String {
        let severity = match self.severity {
            LogType::Debug => "Debug",
            LogType::Info => "Info",
            LogType::Warning => "Warning",
            LogType::Error => "Error",
            LogType::Fatal => "Fatal",
        };

        let timestamp = &self.timestamp.to_string();
        let description = self.get_description();

        format!("[{}]\n[TIMESTAMP]:\t{}\n[DESCRIPTION]:\t{}\n\n", severity, timestamp, description)
    }

    pub fn new(severity: LogType, timestamp: DateTime<Utc>, description: String) -> LogData {
        LogData {
            severity,
            timestamp,
            description,
        }
    }

    pub fn fatal(description: &str) -> Self {
        let severity = LogType::Fatal;

        self::create_log_data(severity, description)
    }

    pub fn error(description: &str) -> Self {
        let severity = LogType::Error;

        self::create_log_data(severity, description)
    }

    pub fn warning(description: &str) -> Self {
        let severity = LogType::Warning;

        self::create_log_data(severity, description)
    }

    pub fn info(description: &str) -> Self {
        let severity = LogType::Info;

        self::create_log_data(severity, description)
    }

    pub fn debug(description: &str) -> Self {
        let severity = LogType::Debug;

        self::create_log_data(severity, description)
    }
}

impl SendableMessage for LogData {
    fn encode(&self) -> String {
        let timestamp = self.timestamp.to_string();
        let severity = match self.severity {
            LogType::Debug => "debug",
            LogType::Info => "info",
            LogType::Warning => "warning",
            LogType::Error => "error",
            LogType::Fatal => "fatal",
        };
        let description = &self.description;
        "log".to_string() + &severity.to_string() + &timestamp.to_string() + description
    }
}

pub fn get_timestamp() -> DateTime<Utc> {
    Utc::now()
}

fn create_log_data(severity: LogType, description: &str) -> LogData {
    let timestamp = get_timestamp();

    LogData::new(severity, timestamp, description.to_string())
}