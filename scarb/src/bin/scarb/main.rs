use anyhow::Result;
use clap::Parser;
use tracing_log::AsTrace;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::EnvFilter;

use args::Args;
use scarb::core::Config;
use scarb::dirs::AppDirs;
use scarb::ops;
use scarb::ui::Ui;

mod args;
mod commands;

fn main() {
    let args: Args = Args::parse();

    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(args.verbose.log_level_filter().as_trace().into())
                .with_env_var("SCARB_LOG")
                .from_env_lossy(),
        )
        .init();

    // Copy values used in error reporting.
    let output_format = args.output_format();

    if let Err(err) = cli_main(args) {
        let ui = Ui::new(output_format);
        ui.error(format!("{err:?}"));
        std::process::exit(1);
    }
}

fn cli_main(args: Args) -> Result<()> {
    let mut dirs = AppDirs::std()?;
    dirs.apply_env_overrides()?;

    let ui = Ui::new(args.output_format());

    let manifest_path = ops::find_manifest_path(args.manifest_path.as_deref())?;
    let mut config = Config::init(manifest_path, dirs, ui)?;
    commands::run(args.command, &mut config)
}