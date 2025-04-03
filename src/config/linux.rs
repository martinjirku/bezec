use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Linux {
    namespaces: Option<Vec<Namespace>>,
    #[serde(rename = "uidMappings")]
    uid_mappings: Option<Vec<IDMapping>>,
    #[serde(rename = "gidMappings")]
    gid_mappings: Option<Vec<IDMapping>>,
    timeoffsets: Option<Vec<TimeOffset>>,
    devices: Option<Vec<Device>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Device {
    #[serde(rename = "type")]
    typ: String,
    #[serde(rename = "path")]
    path: String,
    major: i64,
    minor: i64,
    #[serde(rename = "fileMode")]
    file_mode: u32,
    uid: u32,
    gid: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Namespace {
    #[serde(rename = "type")]
    typ: NamespaceType,
    path: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum NamespaceType {
    #[serde(rename = "pid")]
    PID,
    #[serde(rename = "network")]
    Network,
    #[serde(rename = "mount")]
    Mount,
    #[serde(rename = "ipc")]
    IPC,
    #[serde(rename = "uts")]
    UTS,
    #[serde(rename = "user")]
    User,
    #[serde(rename = "cgroup")]
    CGroup,
    #[serde(rename = "time")]
    Time,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IDMapping {
    #[serde(rename = "containerID")]
    container_id: u32,
    #[serde(rename = "hostID")]
    host_id: u32,
    size: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TimeOffset {
    secs: Option<i64>,
    nanosecs: Option<u32>,
}