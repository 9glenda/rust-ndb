use clap::{Parser, Subcommand};
use color_eyre::{eyre::eyre, eyre::Report, eyre::Result};
use ndb::{self, parser};
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
    JsonPrint { text: String },
}

#[instrument]
fn main() -> Result<(), Report> {
    #[cfg(feature = "capture-spantrace")]
    install_tracing();
    color_eyre::install()?;
    let cli = Cli::parse();
    match &cli.command {
        Commands::JsonPrint { text } => {
            let parsed = parse_text(text.to_string()).unwrap();
            #[cfg(not(feature = "serde"))]
            {
                warn_span!("missing feature `serde`");
                eprintln!("falling back to pretty printing");
                println!("{:#?}", &parsed);
            }
            #[cfg(feature = "serde")]
            {
                println!("{}", serde_json::to_string(&parsed).unwrap());
            }
        }
    }

    // println!("{:?}", parsed);
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
fn parse_text(input: String) -> Result<parser::NdbStmt, Report> {
    info!("parsing {}", input);

    match input.clone().as_ref() {
        "" => Err(eyre!("got empyty input")), // invalid name
        _ => {
            // TODO check if _ (text) is empty
            let (_, ndb_stmt) = ndb::parser::ndb_stmt(Box::leak(input.into_boxed_str()))?;

            Ok(ndb_stmt)
        }
    }
}
