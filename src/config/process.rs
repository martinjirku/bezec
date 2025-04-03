use serde::{Deserialize, Serialize};
use crate::config::scheduler::Scheduler;
use crate::config::capabilities::Capabilities;


#[derive(Serialize, Deserialize, Debug)]
pub struct Process {
    terminal: Option<bool>,
    #[serde(rename = "consoleSize")]
    console_size: Option<ConsoleSize>,
    cwd: String,
    env: Option<Vec<String>>,
    // with similar semantics to [IEEE Std 1003.1-2008 execvp's argv](https://pubs.opengroup.org/onlinepubs/9699919799/functions/exec.html).
    args: Option<Vec<String>>,
    rlimits: Option<Vec<Rlimit>>,
    #[serde(rename = "apparmorProfile")]
    apparmor_profile: Option<String>,
    capabilities: Option<Capabilities>,
    #[serde(rename = "noNewPrivileges")]
    no_new_privileges: Option<bool>,
    #[serde(rename = "oomScoreAdj")]
    oom_score_adj: Option<i32>,
    scheduler: Option<Scheduler>,
    #[serde(rename = "selinuxLabel")]
    selinux_label: Option<String>,
    #[serde(rename = "ioPriority")]
    io_priority: Option<IOPriority>,
    #[serde(rename = "execCPUAffinity")]
    exec_cpu_affinity: Option<ExecCPUAffinity>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConsoleSize {
    height: u16,
    width: u16
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Rlimit {
    #[serde(rename = "type")]
    typ: String,
    soft: u64,
    hard: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IOPriority {
    class: String,
    level: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExecCPUAffinity {
    initial: Option<String>,
    #[serde(rename = "final")]
    final_: Option<Vec<u32>>,
}