#![allow(unused_imports)]
pub(crate) use clap::{ArgAction::SetTrue, Args, Parser, Subcommand};
pub(crate) use chrono::{DateTime, Datelike, FixedOffset, NaiveDateTime, Utc};
pub(crate) use colored::Colorize;
pub(crate) use dioxus::logger::tracing::{debug, error, info, trace, warn};
pub(crate) use dioxus::prelude::*;
pub(crate) use rss::{
    Channel as RssChannel, Enclosure as RssEnclosure, Guid as RssGuid, Item as RssItem,
};
pub(crate) use serde::de::DeserializeOwned;
pub(crate) use serde::{Deserialize, Serialize};
pub(crate) use std::any::Any;
pub(crate) use std::collections::BTreeMap;
pub(crate) use std::collections::HashMap;
pub(crate) use std::error::Error;
pub(crate) use std::fmt::Result as FmtResult;
pub(crate) use std::fmt::{Display, Formatter};
pub(crate) use std::fs::File;
pub(crate) use std::io::{BufReader, BufWriter};
pub(crate) use std::mem::take;
pub(crate) use std::path::{Path, PathBuf};
pub(crate) use url::Url;


pub(crate) use rss::extension::itunes::ITunesChannelExtension;
pub(crate) use strum_macros::AsRefStr;

pub(crate) use reqwest::StatusCode;