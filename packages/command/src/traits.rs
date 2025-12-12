use crate::prelude::*;

#[async_trait]
pub trait Execute<In, Out, E> {
    async fn execute(&self, request: &In) -> Result<Out, E>;
}

pub trait Executable: Clone + Display + Sized {
    type Response: Debug + Send + Sync;
    type ExecutionError: Debug + Send + Sync;
}
