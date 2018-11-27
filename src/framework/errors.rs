pub enum LogType {
    Debug(),
    Info(),
    Warning(),
    Error(),
    Fatal()
}

pub trait LogData<PayloadType> {
    fn get_severity(&self) -> &LogType;
    fn get_short_description(&self) -> &str;
    fn get_full_description(&self) -> Option<&str>;
    fn get_payload(&self) -> &PayloadType;
}