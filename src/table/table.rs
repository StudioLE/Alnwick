use crate::prelude::*;
use chrono::Datelike;
use itertools::Itertools;

#[allow(clippy::absolute_paths)]
#[component]
pub fn Table() -> Element {
    let state = use_context::<EntryState>();
    let collection = state.get_cloned();
    let chunks = collection
        .entries
        .into_values()
        .chunk_by(|xZ| (xZ.date.year(), xZ.date.format("%B").to_string()));
    rsx! {
        section { class: "section",
            for ((year, month), entries) in chunks {
                h2 { class: "subtitle",
                    "{month} {year}"
                }
                table { class: "table is-fullwidth is-striped",
                    thead {
                        tr {
                            th { "Date" }
                            th { "Weight" }
                            th { "Shot" }
                        }
                    }
                    tbody {
                        for entry in entries {
                            tr {
                                td {
                                    "{entry.date.format(\"%A %e\") }"
                                    sup {
                                        "{get_ordinal(entry.date)}"
                                    }
                                }
                                td { class: "has-text-right",
                                    if let Some(weight) = &entry.weight {
                                        "{weight:.2}"
                                    } else {
                                        ""
                                    }
                                }
                                td {
                                    if let Some(shot) = &entry.shot {
                                        "{shot.dose:.2} mg at {shot.time.format(\"%-I %M %P\")}"
                                    } else {
                                        ""
                                    }
                                }
                            }
                        }
                    }
                }
            }

        }
    }
}

fn get_ordinal(date: NaiveDate) -> String {
    let day = date.day();
    let ordinal = match day {
        11..=13 => "th", // Special cases (11th, 12th, 13th)
        _ => match day % 10 {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th",
        },
    };
    ordinal.to_owned()
}
