use crate::prelude::*;

define_commands_web!(
    Cover(CoverRequest),
    Download(DownloadRequest),
    Emulate(EmulateRequest),
    Fetch(FetchRequest),
);
#[cfg(feature = "server")]
define_commands_server!(
    Cover(CoverRequest, CoverHandler),
    Download(DownloadRequest, DownloadHandler),
    Emulate(EmulateRequest, EmulateHandler),
    Fetch(FetchRequest, FetchHandler),
);
