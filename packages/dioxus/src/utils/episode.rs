use crate::prelude::*;
use html2text::config::plain;
use std::fmt::Write as _;

pub trait EpisodeInfoExt {
    fn get_subtitle(&self) -> String;

    fn get_description(&self) -> Option<String>;
}

impl EpisodeInfoExt for EpisodeInfo {
    fn get_subtitle(&self) -> String {
        let mut subtitle = Vec::new();
        subtitle.push(self.published_at.format("%-d %B %Y").to_string());
        if self.season.is_some() || self.episode.is_some() {
            let mut season_episode = String::new();
            if let Some(season) = self.season {
                let _ = write!(season_episode, "S{season:02}");
            }
            if let Some(number) = self.episode {
                let _ = write!(season_episode, "E{number:02}");
            }
            subtitle.push(season_episode);
        }
        if let Some(duration) = self.source_duration {
            subtitle.push(format_duration_human(duration));
        }
        if let Some(kind) = self.kind
            && kind != EpisodeKind::Full
        {
            subtitle.push(kind.to_string());
        }
        subtitle.join(" · ")
    }

    fn get_description(&self) -> Option<String> {
        let description = self.description.clone()?;
        if description.starts_with('<') {
            plain()
                .no_link_wrapping()
                .do_decorate()
                .link_footnotes(true)
                .string_from_read(description.as_bytes(), 1000)
                .ok()
        } else {
            Some(description)
        }
    }
}
