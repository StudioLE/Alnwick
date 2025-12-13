use crate::prelude::*;

#[macro_export]
macro_rules! define_commands_web {
    ($($kind:ident($req:ty)),* $(,)?) => {
        #[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
        pub enum CommandRequest {
            $(
                $kind($req),
            )*
        }

        impl IRequest for CommandRequest {}

        $(
            impl From<$req> for CommandRequest {
                fn from(request: $req) -> Self {
                    Self::$kind(request)
                }
            }
        )*

        #[derive(Clone, Debug, Deserialize, Serialize)]
        pub enum CommandSuccess {
            $(
                $kind(<$req as Executable>::Response),
            )*
        }

        impl ISuccess for CommandSuccess {}

        #[derive(Debug)]
        pub enum CommandFailure {
            $(
                $kind(<$req as Executable>::ExecutionError),
            )*
        }

        impl IFailure for CommandFailure {}

        #[derive(Clone, Debug, Deserialize, Serialize)]
        pub struct CommandEvent {
            kind: EventKind,
            request: CommandRequest,
            success: Option<CommandSuccess>,
        }

        impl IEvent<CommandRequest, CommandSuccess> for CommandEvent {
            fn new(kind: EventKind, request: CommandRequest, success: Option<CommandSuccess>) -> Self {
                Self { kind, request, success }
            }

            fn get_kind(&self) -> &EventKind {
                &self.kind
            }

            fn get_request(&self) -> &CommandRequest {
                &self.request
            }

            fn get_success(&self) -> &Option<CommandSuccess> {
                &self.success
            }
        }

        pub struct CommandInfo;

        impl ICommandInfo for CommandInfo {
            type Request = CommandRequest;
            #[cfg(feature = "server")]
            type Command =  Command;
            #[cfg(feature = "server")]
            type Handler = CommandHandler;
            type Success = CommandSuccess;
            type Failure = CommandFailure;
            type Event = CommandEvent;
        }
    };
}

pub trait IRequest:
    Clone + Debug + DeserializeOwned + Eq + Hash + PartialEq + Send + Serialize + Sync
{
}
pub trait ISuccess: Clone + Debug + DeserializeOwned + Send + Serialize + Sync {}
pub trait IFailure: Debug + Send + Sync {}
pub trait IEvent<Req: IRequest, S: ISuccess>: Clone + Debug + Send + Sync {
    fn new(kind: EventKind, request: Req, success: Option<S>) -> Self;

    fn get_kind(&self) -> &EventKind;

    fn get_request(&self) -> &Req;

    fn get_success(&self) -> &Option<S>;
}

pub trait ICommandInfo {
    type Request: IRequest;
    #[cfg(feature = "server")]
    type Command: ICommand<Self::Handler, Self::Success, Self::Failure>;
    #[cfg(feature = "server")]
    type Handler: IHandler;
    type Success: ISuccess;
    type Failure: IFailure;
    type Event: IEvent<Self::Request, Self::Success>;
}
