#![allow(unused_imports)]
pub(crate) use crate::contexts::page::context::PageContext;
pub(crate) use crate::contexts::page::info::PageInfo;
pub(crate) use crate::contexts::page::selector::PageSelector;
pub(crate) use crate::contexts::settings::context::SettingsContext;
#[cfg(feature = "server")]
pub(crate) use crate::core::cover::*;
#[cfg(feature = "server")]
pub(crate) use crate::core::download::*;
#[cfg(feature = "server")]
pub(crate) use crate::core::emulate::*;
#[cfg(feature = "server")]
pub(crate) use crate::core::schema::*;
#[cfg(feature = "server")]
pub(crate) use crate::core::scrape::*;
#[cfg(feature = "server")]
pub(crate) use crate::core::services::*;
#[cfg(feature = "server")]
pub(crate) use crate::core::utils::*;
#[cfg(feature = "server")]
pub(crate) use chrono::{DateTime, Datelike, FixedOffset, NaiveDateTime};
#[cfg(feature = "server")]
pub(crate) use colored::Colorize;
pub(crate) use dioxus::logger::tracing::{debug, error, info, trace, warn};
pub(crate) use dioxus::prelude::*;
#[cfg(feature = "server")]
pub(crate) use futures::{stream, StreamExt};
#[cfg(feature = "server")]
pub(crate) use reqwest::{Client as ReqwestClient, StatusCode};
#[cfg(feature = "server")]
pub(crate) use rss::{
    Channel as RssChannel, Enclosure as RssEnclosure, Guid as RssGuid, Item as RssItem,
};
#[cfg(feature = "server")]
pub(crate) use scraper::{Html, Selector};
pub(crate) use serde::de::DeserializeOwned;
pub(crate) use serde::{Deserialize, Serialize};
pub(crate) use std::collections::BTreeMap;
pub(crate) use std::collections::HashMap;
pub(crate) use std::error::Error;
pub(crate) use std::fmt::Result as FmtResult;
pub(crate) use std::fmt::{Display, Formatter};
pub(crate) use std::fs::File;
pub(crate) use std::io::{BufReader, BufWriter};
pub(crate) use std::mem::take;
pub(crate) use std::path::{Path, PathBuf};
#[cfg(feature = "server")]
pub(crate) use tokio::fs::{
    copy, create_dir_all, hard_link, metadata, read_dir, remove_dir_all, File as AsyncFile,
};
#[cfg(feature = "server")]
pub(crate) use tokio::io::AsyncWriteExt;
pub use url::Url;
#[cfg(feature = "server")]
pub use clap::{ArgAction::SetTrue, Args, Parser, Subcommand};
