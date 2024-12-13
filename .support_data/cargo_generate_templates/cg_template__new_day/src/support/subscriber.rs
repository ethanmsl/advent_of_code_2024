//! Tracing Subscriber configuration for {{ project-name | upper_camel_case }} of Advent of Code 2024.
//!
//! `generate_tracing_subscriber()` is a convenience function designed to be used with `tracint::subscriber::set_global_default(_)`
//! Unfortunately, the return type created by composing Layers is fragile.
//! And the desired trait (Subscriber) is not Sized and therefore not amenable to use of the `--> dyn _` syntax.
//! Similarly, this makes dynamic choice difficult.
//!
//! A prefer solution may be to simple set the global default subscriber *in* the convenience function as a side-effect.
//! This would allow various branches and customizations.
//!
//! For now, this is workable.
//!
//! ## Caution
//! - Tracing is poorly documented and methods poorly named.  One can easily use, e.g., `::fmt()` instead of `::fmt` and be greeted with cryptic or even misdirecting errors.
//!   - I have no solution for this.  *Just be careful!*  It is very easy to lose a lot of time chain one's tail, on seemingly trivial configuration.

use tracing::level_filters::LevelFilter;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_error::ErrorLayer;
use tracing_subscriber::prelude::*;

use crate::Result;

/// (Convenience function.) Generates a tracing_subcsriber and sets it as global default, while returning a writer guard.
///
/// # Caveat
///   - Side effect. (sets global default tracing subscriber)
///
/// # Use:
/// ```text
/// fn main() -> Result<()> {
///     let _tracing_writer_worker_guard = generate_tracing_subscriber()?;
///    // ...
///    Ok(())
/// }
/// ```
pub fn active_global_default_tracing_subscriber() -> Result<WorkerGuard> {
        let envfilter_layer = tracing_subscriber::EnvFilter::builder()
                .with_default_directive(LevelFilter::TRACE.into())
                .from_env_lossy();

        let tree_layer = tracing_tree::HierarchicalLayer::new(2)
                // .compact()
                // .pretty()
                // .with_timer(<timer>)
                .with_target(true)
                .with_thread_ids(true)
                .with_thread_names(true)
                .with_file(true)
                .with_line_number(true)
                .with_span_events(FmtSpan::FULL)
                .with_writer(non_blocking_writer);

        let error_layer = ErrorLayer::default().with_filter(LevelFilter::TRACE);

        let (non_blocking_writer, trace_writer_guard) = tracing_appender::non_blocking(std::io::stderr());
        let fmt_layer = tracing_subscriber::fmt::Layer::default()
                .with_writer(non_blocking_writer)
                .compact();

        let subscriber = tracing_subscriber::Registry::default()
                .with(error_layer)
                .with(fmt_layer.with_filter(envfilter_layer));
        // .with(fmt_layer.and_then(tree_layer).with_filter(envfilter_layer));

        tracing::subscriber::set_global_default(subscriber)?;
        Ok(trace_writer_guard)
}
