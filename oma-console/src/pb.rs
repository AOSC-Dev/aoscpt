use std::{fmt::Write, time::Duration};

use console::style;
use indicatif::{HumanBytes, ProgressState, ProgressStyle};

use crate::writer::Writer;

const SPINNER: &[&str] = &[
    "( ●    )",
    "(  ●   )",
    "(   ●  )",
    "(    ● )",
    "(     ●)",
    "(    ● )",
    "(   ●  )",
    "(  ●   )",
    "( ●    )",
    "(●     )",
];

pub fn progress_bar_style(writer: &Writer) -> ProgressStyle {
    let max_len = writer.get_length();
    let template = if max_len < 100 {
        " {msg} {percent:>3}"
    } else {
        " {msg:<59} {total_bytes:<11} [{wide_bar:.white/black}] {percent:>3}"
    };

    ProgressStyle::default_bar()
        .template(template)
        .unwrap()
        .progress_chars("=>-")
        .with_key("percent", |state: &ProgressState, w: &mut dyn Write| {
            write!(w, "{:.*}%", 0, state.fraction() * 100f32).unwrap()
        })
}

pub fn global_progress_bar_style(writer: &Writer) -> ProgressStyle {
    let max_len = writer.get_length();
    let template = if max_len < 100 {
        " {prefix:.blue.bold} {bytes:>14.green.bold} {total_bytes:.green.bold} {binary_bytes_per_sec:<10.green.bold}"
    } else {
        " {progress_msg:<59} {elapsed_precise:<11.blue.bold} [{wide_bar:.cyan/blue}] {percent:>3}"
    };

    ProgressStyle::default_bar()
        .template(template)
        .unwrap()
        .progress_chars("=>-")
        .with_key("percent", |state: &ProgressState, w: &mut dyn Write| {
            write!(w, "{:.*}%", 0, state.fraction() * 100f32).unwrap()
        })
        .with_key("progress_msg", oma_global_bar_template)
}

fn oma_global_bar_template(state: &ProgressState, w: &mut dyn Write) {
    write!(w, "{}  ", style("Progress").blue().bold()).unwrap();

    write!(
        w,
        "{}",
        style(format!(
            "{} / {}",
            HumanBytes(state.pos()),
            HumanBytes(state.len().unwrap_or(0))
        ))
        .green()
        .bold()
    )
    .unwrap();

    write!(
        w,
        "{}",
        style(format!(" @ {}/s", HumanBytes(state.per_sec() as u64)))
            .green()
            .bold()
    )
    .unwrap();
}

pub fn spinner_style() -> (ProgressStyle, Duration) {
    let (template, inv) = (SPINNER, 80);

    let style = ProgressStyle::with_template(" {msg:<48} {spinner}")
        .unwrap()
        .tick_strings(template);

    (style, Duration::from_millis(inv))
}
