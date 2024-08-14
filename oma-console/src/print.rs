use std::collections::BTreeMap;

use console::{style, StyledObject};
use num_enum::IntoPrimitive;
use tracing::{field::Field, Level};
use tracing_subscriber::Layer;

use crate::WRITER;

pub enum StyleFollow {
    OmaTheme,
    TermTheme,
}

#[derive(IntoPrimitive)]
#[repr(u8)]
pub enum Action {
    Emphasis = 148,
    Foreground = 72,
    Secondary = 182,
    EmphasisSecondary = 114,
    WARN = 214,
    Purple = 141,
    Note = 178,
}

pub struct OmaColorFormat {
    follow: StyleFollow,
}

impl OmaColorFormat {
    pub fn new(follow: StyleFollow) -> Self {
        Self { follow }
    }

    pub fn color_str<D>(&self, input: D, color: Action) -> StyledObject<D> {
        match self.follow {
            StyleFollow::OmaTheme => style(input).color256(color.into()),
            StyleFollow::TermTheme => match color {
                Action::Emphasis => style(input).green(),
                Action::Secondary => style(input).dim(),
                Action::EmphasisSecondary => style(input).cyan(),
                Action::WARN => style(input).yellow().bold(),
                Action::Purple => style(input).magenta(),
                Action::Note => style(input).yellow(),
                Action::Foreground => style(input).cyan().bold(),
            },
        }
    }
}

pub struct OmaLayer;

impl<S> Layer<S> for OmaLayer
where
    S: tracing::Subscriber,
    S: for<'lookup> tracing_subscriber::registry::LookupSpan<'lookup>,
{
    fn on_event(
        &self,
        event: &tracing::Event<'_>,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        let level = event.metadata().level().to_owned();
        let prefix = match level {
            Level::DEBUG => console::style("DEBUG").dim().to_string(),
            Level::INFO => console::style("INFO").blue().bold().to_string(),
            Level::WARN => console::style("WARNING").yellow().bold().to_string(),
            Level::ERROR => console::style("ERROR").red().bold().to_string(),
            Level::TRACE => console::style("TRACE").dim().to_string(),
        };

        let mut visitor = OmaRecorder(BTreeMap::new());
        event.record(&mut visitor);

        for (k, v) in visitor.0 {
            if k == "message" {
                WRITER.writeln(&prefix, &v).ok();
            }
        }
    }
}

struct OmaRecorder(BTreeMap<String, String>);

impl tracing::field::Visit for OmaRecorder {
    fn record_f64(&mut self, field: &Field, value: f64) {
        self.0.insert(field.name().to_owned(), value.to_string());
    }

    fn record_i64(&mut self, field: &Field, value: i64) {
        self.0.insert(field.name().to_owned(), value.to_string());
    }

    fn record_u64(&mut self, field: &Field, value: u64) {
        self.0.insert(field.name().to_owned(), value.to_string());
    }

    fn record_bool(&mut self, field: &Field, value: bool) {
        self.0.insert(field.name().to_owned(), value.to_string());
    }

    fn record_str(&mut self, field: &Field, value: &str) {
        self.0.insert(field.name().to_owned(), value.to_string());
    }

    fn record_error(&mut self, field: &Field, value: &(dyn std::error::Error + 'static)) {
        self.0
            .insert(field.name().to_owned(), format!("{value:#?}"));
    }

    fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
        self.0
            .insert(field.name().to_owned(), format!("{value:#?}"));
    }
}

/// oma display normal message
#[macro_export]
macro_rules! msg {
    ($($arg:tt)+) => {
        oma_console::WRITER.writeln("", &format!($($arg)+)).ok();
    };
}

/// oma display success message
#[macro_export]
macro_rules! success {
    ($($arg:tt)+) => {
        oma_console::WRITER.writeln(&oma_console::console::style("SUCCESS").green().bold().to_string(), &format!($($arg)+)).ok();
    };
}

/// oma display due_to message
#[macro_export]
macro_rules! due_to {
    ($($arg:tt)+) => {
        oma_console::WRITER.writeln(&oma_console::console::style("DUE TO").yellow().bold().to_string(), &format!($($arg)+)).ok();
    };
}
