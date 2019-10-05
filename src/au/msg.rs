//! Implements all internal data structures used in Augorama.
//!
//! The main data structure is the AuMsg, an envelope for commands and queries.  A command may
//! be a query to get state, a query to get journal records, or a command to update state with
//! the attached telemetry.  Telemetry is always a record with a name, datetime, and a numerical
//! value.

use std::collections::LinkedList;

use chrono::{DateTime, Utc};

#[derive(Clone)]
pub enum AuCmd {
    Get,
    //Set,
    //Ls,
}

/// The single data structure representing the source of all actor state.
#[derive(Clone)]
pub struct AuTelemetry {
    /// UTC TZ 8601 format that is ideally a representation of when the observation was made in the real world
    pub datetime: DateTime<Utc>,
    /// space (deployment) scoped name to type (not instance), ie: `refrigerator.temp.celsius`
    pub name: String,
    /// a double, ie: `22.9`
    pub value: f64,
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

#[derive(Clone)]
pub struct AuForwards(LinkedList<String>);

impl AuForwards {
    pub fn new_one(name: String) -> AuForwards {
        let mut ll = LinkedList::new();
        ll.push_back(name);
        AuForwards(ll)
    }
    pub fn new() -> AuForwards {
        AuForwards(LinkedList::new())
    }
}

impl Default for AuForwards {
    fn default() -> Self {
        AuForwards(LinkedList::new())
    }
}

#[derive(Clone)]
pub struct AuMsg<T> {
    pub cmd: AuCmd,
    pub msg: T,
    pub forward: AuForwards,
}

impl std::fmt::Display for AuForwards {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "list of stuff")
    }
}

impl std::fmt::Display for AuMsg<String> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "(msg: {} cmd: {} forward: {})",
            self.msg, self.cmd, self.forward
        )
    }
}

impl std::fmt::Debug for AuMsg<String> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "(msg: {} cmd: {} forward: {})",
            self.msg, self.cmd, self.forward
        )
    }
}

impl std::fmt::Display for AuCmd {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AuCmd::Get => write!(f, "Get"),
            //AugieCmd::Set => write!(f, "Set"),
            //AugieCmd::Ls => write!(f, "Set"),
        }
    }
}

impl std::fmt::Debug for AuCmd {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AuCmd::Get => write!(f, "Get"),
            //AugieCmd::Set => write!(f, "Set"),
            //AugieCmd::Ls => write!(f, "Set"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::au::msg::AuTelemetry;

    #[test]
    fn default_works() {
        let t = AuTelemetry::default();
        assert_eq!(t.name, "measurement".to_string());
        assert_eq!(t.value, 0.0);
    }
    //
    //    #[test]
    //    fn forward_inspect_works() {
    //        let f = AuFoward("toMe");
    //        f.
    //    }

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
