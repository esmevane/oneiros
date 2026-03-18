use clap::Parser;
use oneiros_cli::Context;
use oneiros_config::{Config, ServiceConfig};
use oneiros_detect_project_name::ProjectRoot;
use oneiros_http::HttpService;
use oneiros_usage::*;

use crate::cases;

pub struct Legacy {
    _temp: tempfile::TempDir,
    context: Context,
    server: Option<tokio::task::JoinHandle<()>>,
}

impl Backend for Legacy {
    async fn start() -> Result<Self, Box<dyn core::error::Error>> {
        let temp = tempfile::TempDir::new()?;
        let data_dir = temp.path().join("data");
        let config_dir = temp.path().join("config");

        std::fs::create_dir_all(&data_dir)?;
        std::fs::create_dir_all(&config_dir)?;

        let context = Context::builder()
            .data_dir(data_dir)
            .config_dir(config_dir)
            .build();

        Ok(Self {
            _temp: temp,
            context,
            server: None,
        })
    }

    async fn exec(&self, command: &str) -> Result<serde_json::Value, Box<dyn core::error::Error>> {
        let args = shell_words(command);
        let mut full_args = vec!["oneiros"];
        full_args.extend(args.iter().map(String::as_str));

        let cli = oneiros_cli::Cli::try_parse_from(full_args)?;
        let result = cli.run_with(&self.context).await?;

        let values: Vec<serde_json::Value> = result
            .outcomes
            .into_iter()
            .filter_map(|outcome| serde_json::to_value(outcome).ok())
            .collect();

        Ok(serde_json::Value::Array(values))
    }

    async fn start_service(&mut self) -> Result<(), Box<dyn core::error::Error>> {
        let project_root = self._temp.path().join("project");
        std::fs::create_dir_all(&project_root)?;

        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await?;
        let addr = listener.local_addr()?;

        self.context = Context::builder()
            .data_dir(self.context.data_dir().to_path_buf())
            .config_dir(self.context.config_dir().to_path_buf())
            .project(ProjectRoot::new("test-project", project_root))
            .config(
                Config::builder()
                    .service(ServiceConfig::builder().port(addr.port()).build())
                    .build(),
            )
            .build();

        let http = HttpService::init(self.context.clone())?;

        self.server = Some(tokio::spawn(async move {
            http.serve(listener).await.unwrap();
        }));

        Ok(())
    }
}

impl Drop for Legacy {
    fn drop(&mut self) {
        if let Some(handle) = &self.server {
            handle.abort();
        }
    }
}

/// Split a command string into words, respecting single and double quotes.
fn shell_words(input: &str) -> Vec<String> {
    let mut words = Vec::new();
    let mut current = String::new();
    let mut chars = input.chars().peekable();
    let mut in_quote: Option<char> = None;

    while let Some(ch) = chars.next() {
        match (ch, in_quote) {
            ('\'' | '"', None) => in_quote = Some(ch),
            (q, Some(open)) if q == open => in_quote = None,
            (' ' | '\t', None) => {
                if !current.is_empty() {
                    words.push(std::mem::take(&mut current));
                }
            }
            _ => current.push(ch),
        }
    }

    if !current.is_empty() {
        words.push(current);
    }

    words
}

#[tokio::test]
async fn system_init_creates_tenant_and_actor() -> TestResult {
    cases::system::init_creates_tenant_and_actor::<Legacy>().await
}

#[tokio::test]
async fn system_init_is_idempotent() -> TestResult {
    cases::system::init_is_idempotent::<Legacy>().await
}

#[tokio::test]
async fn project_init_creates_brain() -> TestResult {
    cases::project::init_creates_brain::<Legacy>().await
}

#[tokio::test]
async fn level_set_creates_a_new_level() -> TestResult {
    cases::level::set_creates_a_new_level::<Legacy>().await
}

#[tokio::test]
async fn level_set_updates_existing_level() -> TestResult {
    cases::level::set_updates_existing_level::<Legacy>().await
}

#[tokio::test]
async fn level_list_returns_empty_when_none_exist() -> TestResult {
    cases::level::list_returns_empty_when_none_exist::<Legacy>().await
}

#[tokio::test]
async fn level_list_returns_created_levels() -> TestResult {
    cases::level::list_returns_created_levels::<Legacy>().await
}

#[tokio::test]
async fn level_remove_makes_it_unlisted() -> TestResult {
    cases::level::remove_makes_it_unlisted::<Legacy>().await
}

#[tokio::test]
async fn seed_core_creates_default_levels() -> TestResult {
    cases::seed::core_creates_default_levels::<Legacy>().await
}
