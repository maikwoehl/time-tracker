mod timer;

use timer::Timer;

#[derive(Debug, Deserialize, Serialize, Default)]
pub(crate) struct UnitTracker {
    save_file: String,
    timer: Timer,
    units: Vec<Unit>,
}

impl UnitTracker {
    pub new(&self, &str save_file) -> Self {
        let mut unit = Unit::default();
        unit.save_file = save_file.to_string();

        unit
    }
}
