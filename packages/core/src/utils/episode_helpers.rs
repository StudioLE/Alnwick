use crate::prelude::*;
use std::fmt::Write as _;

#[must_use]
pub fn get_episode_file_stem(
    title: String,
    published_at: DateTime<FixedOffset>,
    season: Option<u32>,
    episode: Option<u32>,
    kind: Option<EpisodeKind>,
) -> String {
    let mut output = format_date(published_at);
    if let Some(season) = season {
        let _ = write!(output, " S{season:02}");
        if let Some(number) = episode {
            // If episode number with season
            let _ = write!(output, "E{number:02}");
        }
    } else if let Some(number) = episode {
        // If episode number without season
        let _ = write!(output, " {number:03}");
    }
    if let Some(kind) = kind
        && kind != EpisodeKind::Full
    {
        output.push(' ');
        output.push_str(&kind.to_string().to_uppercase());
    }
    if episode.is_none() && kind == Some(EpisodeKind::Full) {
        warn!(
            "Episode has no number and is not a trailer or bonus: {}",
            title
        );
    }
    output.push(' ');
    output.push_str(Sanitizer::execute(&title).trim());
    output
}

#[must_use]
pub fn get_episode_file_extenson(content_type: &str) -> Option<String> {
    let extension = match content_type {
        "audio/mpeg" => MP3_EXTENSION,
        "audio/x-m4a" => "m4a",
        "video/quicktime" => "mov",
        "video/mp4" => "mp4",
        "video/x-m4v" => "m4v",
        "application/pdf" => "pdf",
        _ => return None,
    };
    Some(extension.to_owned())
}

fn format_date(date: DateTime<FixedOffset>) -> String {
    date.format("%Y-%m-%d").to_string()
}
