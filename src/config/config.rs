use serde::{Deserialize, Serialize};
use crate::config::mount::Mount;
use crate::config::process::Process;
use crate::config::linux::Linux;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    #[serde(rename = "ociVersion")]
    oci_version: String,
    root: Root,
    mounts: Vec<Mount>,
    process: Option<Process>,
    user: Option<User>,
    hostname: Option<String>,
    domainname: Option<String>,
    linux: Option<Linux>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Root {
    path: String,
    readonly: Option<bool>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    uid: u32,
    gid: u32,
    umask: Option<Vec<u32>>,
    #[serde(rename = "additionalGids")]
    additional_gids: Option<Vec<u32>>,
}
