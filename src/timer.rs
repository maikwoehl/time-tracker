#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Timer {
    pub start_time: u64,
    pub project_code: String,
    pub task: String,
}
