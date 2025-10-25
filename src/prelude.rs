#![allow(unused_imports)]
pub(crate) use crate::contexts::page::context::PageContext;
pub(crate) use crate::contexts::page::info::PageInfo;
pub(crate) use crate::contexts::page::selector::PageSelector;
pub(crate) use crate::contexts::settings::context::SettingsContext;
pub(crate) use crate::core::schema::episode::Episode;
pub(crate) use crate::core::schema::podcast::Podcast;
pub(crate) use chrono::{DateTime, Datelike, FixedOffset, NaiveDateTime};
pub(crate) use colored::Colorize;
pub(crate) use dioxus::logger::tracing::{debug, error, info, trace, warn};
pub(crate) use dioxus::prelude::*;
pub(crate) use rss::{
    Channel as RssChannel, Enclosure as RssEnclosure, Guid as RssGuid, Item as RssItem,
};
pub(crate) use serde::de::DeserializeOwned;
pub(crate) use serde::{Deserialize, Serialize};
#[cfg(feature = "server")]
pub(crate) use server::*;
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

#[cfg(feature = "server")]
mod server {
    pub(crate) use crate::core::services::*;
    pub(crate) use crate::core::utils::*;
    pub(crate) use clap::{ArgAction::SetTrue, Args, Parser, Subcommand};
    pub(crate) use futures::{stream, StreamExt};
    pub(crate) use reqwest::{Client as ReqwestClient, StatusCode};
    pub(crate) use scraper::{Html, Selector};
    pub(crate) use tokio::fs::{
        copy, create_dir_all, hard_link, metadata, read_dir, remove_dir_all, File as AsyncFile,
    };
    pub(crate) use tokio::io::AsyncWriteExt;
}
