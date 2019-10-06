//! Implements all internal data structures used in Augorama.
//!
//! The main data structure is the AuMsg, an envelope for commands and queries.  A command may
//! be a query to get state, a query to get journal records, or a command to update state with
//! the attached telemetry.  Telemetry is always a record with a name, datetime, and a numerical
//! value.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, PartialEq)]
pub enum AuCmd {
    Get,
    Set,
    //Ls,
}

/// The single data structure representing the source of all actor state.
#[derive(Clone, Serialize, Deserialize)]
pub struct AuTelemetry {
    /// UTC TZ 8601 format that is ideally a representation of when the observation was made in the real world
    pub datetime: DateTime<Utc>,
    /// space (deployment) scoped name to type (not instance), ie: `refrigerator.temp.celsius`
    pub name: String,
    /// a double, ie: `22.9`
    pub value: f64,
}

impl std::fmt::Debug for AuTelemetry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({} {:?} {:?})", self.name, self.value, self.datetime)
    }
}

impl Default for AuTelemetry {
    fn default() -> Self {
        AuTelemetry {
            datetime: Utc::now(),
            name: "measurement".to_string(),
            value: 0.0,
        }
    }
}

/// Actors keep their state in collections of telemetry records - some derived and some
/// are meters (last update).
#[derive(Clone, Serialize, Deserialize)]
pub struct AuState {
    pub state: HashMap<String, AuTelemetry>,
}

#[derive(Clone, Debug)]
pub struct AuMsg<T> {
    pub cmd: AuCmd,
    pub msg: Option<T>,
    pub path: Vec<String>,
}

/*
impl std::fmt::Display for AuMsg<String> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(msg: {} cmd: {} path: ?)", self.msg, self.cmd)
    }
}

impl std::fmt::Debug for AuMsg<String> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(msg: {} cmd: {} path: ?)", self.msg, self.cmd)
    }
}
*/

impl std::fmt::Display for AuCmd {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AuCmd::Get => write!(f, "Get"),
            AuCmd::Set => write!(f, "Set"),
            //AugieCmd::Ls => write!(f, "Set"),
        }
    }
}

impl std::fmt::Debug for AuCmd {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AuCmd::Get => write!(f, "Get"),
            AuCmd::Set => write!(f, "Set"),
            //AugieCmd::Ls => write!(f, "Set"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::au::model::*;

    #[test]
    fn default_works() {
        let t = AuTelemetry::default();
        assert_eq!(t.name, "measurement".to_string());
        assert_eq!(t.value, 0.0);
    }

    #[test]
    fn path_inspect_works() {
        let m: AuMsg<Vec<AuTelemetry>> = AuMsg {
            msg: None,
            cmd: AuCmd::Get,
            path: vec![
                "root".to_string(),
                "actors".to_string(),
                "actor1".to_string(),
                "child1".to_string(),
            ],
        };

        let r = m.path.get(0);
        assert!(r.is_some());
        match r {
            Some(root) => assert_eq!(root, "root"),
            None => assert!(false),
        }
    }

    #[test]
    fn default_override_works() {
        let t = AuTelemetry {
            name: "charge_remaining".to_string(),
            value: 0.1,
            ..Default::default()
        };
        assert_eq!(t.name, "charge_remaining".to_string());
        assert_eq!(t.value, 0.1);
    }
}
