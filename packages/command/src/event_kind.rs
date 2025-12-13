use crate::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum EventKind {
    Queued,
    Executing,
    Succeeded,
    Failed,
}
