use log::LevelFilter;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum LddLevelFilter {
    Off,

    Error,

    Warn,

    Info,

    Debug,

    Trace,
}

impl From<LevelFilter> for LddLevelFilter {
    fn from(level: LevelFilter) -> Self {
        match level {
            LevelFilter::Off => LddLevelFilter::Off,
            LevelFilter::Error => LddLevelFilter::Error,
            LevelFilter::Warn => LddLevelFilter::Warn,
            LevelFilter::Info => LddLevelFilter::Info,
            LevelFilter::Debug => LddLevelFilter::Debug,
            LevelFilter::Trace => LddLevelFilter::Trace,
        }
    }
}

impl From<LddLevelFilter> for LevelFilter {
    fn from(level: LddLevelFilter) -> Self {
        match level {
            LddLevelFilter::Off => LevelFilter::Off,
            LddLevelFilter::Error => LevelFilter::Error,
            LddLevelFilter::Warn => LevelFilter::Warn,
            LddLevelFilter::Info => LevelFilter::Info,
            LddLevelFilter::Debug => LevelFilter::Debug,
            LddLevelFilter::Trace => LevelFilter::Trace,
        }
    }
}
