use clap::Parser;
use oneiros_engine::{Cli, OutputMode, Rendered, Responses};

#[tokio::main]
async fn main() -> Result<(), Box<dyn core::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .init();

    let cli = Cli::parse();
    let config = cli.config().clone().with_config_file();
    let result: Rendered<Responses> = cli.execute(&config).await?;

    match config.output {
        OutputMode::Json => {
            println!("{}", serde_json::to_string_pretty(result.response())?);
        }
        OutputMode::Prompt => {
            if result.has_prompt() {
                print!("{}", result.prompt());
            } else {
                println!("{}", serde_json::to_string_pretty(result.response())?);
            }
        }
        OutputMode::Text => {
            if result.has_text() {
                print!("{}", result.text());
            } else {
                println!("{}", serde_json::to_string_pretty(result.response())?);
            }
        }
    }

    Ok(())
}
