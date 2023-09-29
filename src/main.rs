use clap::{Parser, Subcommand};
use color_eyre::{eyre::Report, eyre::eyre, eyre::Result};
use tracing::{info, instrument};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Print { text: String },
}

#[instrument]
fn main() -> Result<(),Report> {
    #[cfg(feature = "capture-spantrace")]
    install_tracing();
    color_eyre::install()?;
    let cli = Cli::parse();
    println!("{}",match &cli.command {
        Commands::Print { text } => say_hello(text),
    }.unwrap());
    Ok(())
}

#[cfg(feature = "capture-spantrace")]
fn install_tracing() {
    use tracing_error::ErrorLayer;
    use tracing_subscriber::prelude::*;
    use tracing_subscriber::{fmt, EnvFilter};

    let fmt_layer = fmt::layer().with_target(false);
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .with(ErrorLayer::default())
        .init();
}

#[instrument]
fn say_hello(name: &str) -> Result<String,Report> {
    info!("say hello called");

    match name {
        "" | "a" => Err(eyre!("invalid name")), // invalid name
        x => {

    info!("valid name");
            Ok(format!("hello {}", x))
        },
    }
}
