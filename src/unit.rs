#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Unit {
    pub project_code: String,
    pub start_time: u64,
    pub end_time: u64,
    pub duration: u64,
    pub task: String,
}
