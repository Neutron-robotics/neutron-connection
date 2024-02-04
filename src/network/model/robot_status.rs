use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct RobotStatus {
    pub status: String,
    pub battery: Option<IBatteryStatus>,
    pub system: Option<IRobotSystemStatus>,
    pub location: Option<IRobotLocationStatus>,
    pub processes: Option<Vec<IRobotProcess>>,
    pub context: Option<IRobotContextProcess>,
}

#[derive(Serialize, Deserialize)]
pub struct IBatteryStatus {
    pub charging: bool,
    pub level: f64,
}

#[derive(Serialize, Deserialize)]
pub struct IRobotProcess {
    pub cpu: f64,
    pub mem: f64,
    pub mem_usage: f64,
    pub active: bool,
    pub pid: i32,
    pub name: String,
    pub id: String,
}

#[derive(Serialize, Deserialize)]
pub struct IRobotContextProcess {
    pub cpu: f64,
    pub mem: f64,
    pub mem_usage: f64,
    pub active: bool,
    pub pid: i32,
    pub name: String,
    pub id: String,
    pub port: i32,
}

#[derive(Serialize, Deserialize)]
pub struct IRobotLocationStatus {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct IRobotSystemStatus {
    pub cpu: f64,
    pub memory: f64,
}
