use crate::prelude::*;
use std::fmt::Write as _;

pub fn get_subtitle(
    published_at: DateTime<FixedOffset>,
    season: Option<u32>,
    episode: Option<u32>,
    source_duration: Option<u32>,
    kind: Option<EpisodeKind>,
) -> String {
    let mut subtitle = Vec::new();
    subtitle.push(published_at.format("%-d %B %Y").to_string());
    if season.is_some() || episode.is_some() {
        let mut season_episode = String::new();
        if let Some(season) = season {
            let _ = write!(season_episode, "S{season:02}");
        }
        if let Some(number) = episode {
            let _ = write!(season_episode, "E{number:02}");
        }
        subtitle.push(season_episode);
    }
    if let Some(duration) = source_duration {
        subtitle.push(format_duration_human(duration));
    }
    if let Some(kind) = kind
        && kind != EpisodeKind::Full
    {
        subtitle.push(kind.to_string());
    }
    subtitle.join(" · ")
}
