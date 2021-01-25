#![warn(clippy::all)]
#![deny(rust_2018_idioms)]
#![allow(clippy::cognitive_complexity)]

#[macro_use]
extern crate tracing;
use tracing_subscriber::EnvFilter;

macro_rules! check_type_size {
    ($ident:ident: $ty:ty, $size:expr) => {
        #[allow(dead_code)]
        static $ident: [(); $size] = [(); std::mem::size_of::<$ty>()];
    };
}

pub mod asm;
mod compiler;
mod errors;
#[macro_use]
pub mod lexer;
pub mod span;
pub mod transforms;
pub mod visitor;

pub mod binary;

pub use compiler::*;

pub fn init_logger() -> Result<(), String> {
    let layer = tracing_tree::HierarchicalLayer::default()
        .with_verbose_exit(true)
        .with_verbose_entry(true)
        .with_targets(true)
        .with_indent_lines(true);
    let filter = EnvFilter::from_default_env();
    use tracing_subscriber::layer::SubscriberExt;
    let subscriber = tracing_subscriber::Registry::default()
        .with(filter)
        .with(layer);
    tracing::subscriber::set_global_default(subscriber).unwrap();

    tracing_log::LogTracer::init().map_err(|e| e.to_string())
}
