use std::collections::LinkedList;

#[derive(Clone)]
pub enum AuCmd {
    Get,
    //Set,
    //Ls,
}

#[derive(Clone)]
pub struct AuTelemetry {
    pub datetime: String,
    pub name: String,
    pub value: f64,
}

#[derive(Clone)]
pub struct AuMsg<T> {
    pub cmd: AuCmd,
    pub msg: T,
    pub forward: Option<LinkedList<String>>,
}

impl std::fmt::Display for AuMsg<String> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.forward.is_none() {
            write!(f, "(msg: {} cmd: {})", self.msg, self.cmd)
        } else {
            write!(f, "(msg: {} cmd: {} forward: {})", self.msg, self.cmd, "self.forward")
        }
    }
}

impl std::fmt::Debug for AuMsg<String> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.forward.is_none() {
            write!(f, "(msg: {} cmd: {})", self.msg, self.cmd)
        } else {
            write!(f, "(msg: {} cmd: {} forward: {})", self.msg, self.cmd, "self.forward")
        }
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
