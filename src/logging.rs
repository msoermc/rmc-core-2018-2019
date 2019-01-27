use std::fs::OpenOptions;

use slog::Drain;
use slog::Duplicate;
use slog_scope::GlobalLoggerGuard;

use crate::robot_map::*;

pub fn launch_logger() -> GlobalLoggerGuard {
    let term_decorator = slog_term::TermDecorator::new().force_color().build();
    let term_drain = slog_term::FullFormat::new(term_decorator).build().fuse();
    let term_drain = slog_async::Async::new(term_drain).build().fuse();

    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(LOG_PATH)
        .unwrap();

    let file_decorator = slog_term::PlainDecorator::new(file);
    let file_drain = slog_term::FullFormat::new(file_decorator).build().fuse();
    let file_drain = slog_async::Async::new(file_drain).build().fuse();

    let broadcaster = Duplicate::new(term_drain, file_drain)
        .filter_level(LOG_FILTER_LEVEL);

    let logger = slog::Logger::root(broadcaster.fuse(), o!());

    let scope_guard = slog_scope::set_global_logger(logger);
    slog_stdlog::init().unwrap();

    scope_guard
}