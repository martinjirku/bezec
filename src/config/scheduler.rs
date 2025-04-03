use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Scheduler {
    policy: SchedulerPolicy,
    nice: Option<i32>,
    priority: Option<i32>,
    flags: Option<Vec<SchedulerFlag>>,
    runtime: Option<u64>,
    deadline: Option<u64>,
    period: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SchedulerFlag {
    #[serde(rename = "SCHED_FLAG_RESET_ON_FORK")]
    ResetOnFork,
    #[serde(rename = "SCHED_FLAG_RECLAIM")]
    Reclaim,
    #[serde(rename = "SCHED_FLAG_DL_OVERRUN")]
    Overrun,
    #[serde(rename = "SCHED_FLAG_KEEP_POLICY")]
    KeepPolicy,
    #[serde(rename = "SCHED_FLAG_KEEP_PARAMS")]
    KeepParams,
    #[serde(rename = "SCHED_FLAG_UTIL_CLAMP_MIN")]
    UtilClampMin,
    #[serde(rename = "SCHED_FLAG_UTIL_CLAMP_MAX")]
    UtilClampMax,
}
#[derive(Serialize, Deserialize, Debug)]
pub enum SchedulerPolicy {
    #[serde(rename = "SCHED_OTHER")]
    Other,
    #[serde(rename = "SCHED_FIFO")]
    Fifo,
    #[serde(rename = "SCHED_RR")]
    RR,
    #[serde(rename = "SCHED_BATCH")]
    Batch,
    #[serde(rename = "SCHED_ISO")]
    ISO,
    #[serde(rename = "SCHED_IDLE")]
    Idle,
    #[serde(rename = "SCHED_DEADLINE")]
    Deadline,
}

