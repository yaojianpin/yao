use core::{clone::Clone, convert::From, fmt};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default, Clone, PartialEq)]
pub enum TaskState {
    #[default]
    None,
    Running,
    Pending,
    Success,
    WaitingEvent,
    Fail(String),
    Skip,
    Abort(String),
}

impl TaskState {
    pub fn is_completed(&self) -> bool {
        match self {
            TaskState::Success | TaskState::Fail(..) | TaskState::Skip | TaskState::Abort(..) => {
                true
            }
            _ => false,
        }
    }

    pub fn is_error(&self) -> bool {
        match self {
            TaskState::Fail(..) | TaskState::Abort(..) => true,
            _ => false,
        }
    }
}

impl fmt::Debug for TaskState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let s: String = self.into();
        f.write_str(&s)
    }
}

impl fmt::Display for TaskState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let s: String = self.into();
        f.write_str(&s)
    }
}

impl From<TaskState> for String {
    fn from(state: TaskState) -> Self {
        state_to_str(state)
    }
}

impl From<&str> for TaskState {
    fn from(str: &str) -> Self {
        str_to_state(str)
    }
}

impl From<&TaskState> for String {
    fn from(state: &TaskState) -> Self {
        state_to_str(state.clone())
    }
}

fn state_to_str(state: TaskState) -> String {
    let s = match state {
        TaskState::None => "none".to_string(),
        TaskState::WaitingEvent => "waiting_event".to_string(),
        TaskState::Pending => "pending".to_string(),
        TaskState::Running => "running".to_string(),
        TaskState::Success => "success".to_string(),
        TaskState::Fail(s) => format!("fail({})", s),
        TaskState::Skip => "skip".to_string(),
        TaskState::Abort(s) => format!("abort({})", s),
    };

    s
}

fn str_to_state(str: &str) -> TaskState {
    let re = regex::Regex::new(r"^(.*)\((.*)\)$").unwrap();
    let s = match str {
        "none" => TaskState::None,
        "waiting_event" => TaskState::WaitingEvent,
        "pending" => TaskState::Pending,
        "running" => TaskState::Running,
        "success" => TaskState::Success,
        "skip" => TaskState::Skip,
        _ => {
            let caps = re.captures(str);
            if let Some(caps) = caps {
                let name = caps.get(1).map_or("", |m| m.as_str());
                let err = caps.get(2).map_or("", |m| m.as_str());

                if name == "fail" {
                    return TaskState::Fail(err.to_string());
                }

                if name == "abort" {
                    return TaskState::Abort(err.to_string());
                }
            }

            TaskState::None
        }
    };

    s
}
