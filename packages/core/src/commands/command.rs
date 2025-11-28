use crate::prelude::*;

pub trait Command: Service {
    type Input;
    type Output;
    type CommandError;

    fn execute(
        &self,
        request: Self::Input,
    ) -> impl Future<Output = Result<Self::Output, Report<Self::CommandError>>> + Send;
}
