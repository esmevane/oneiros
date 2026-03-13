use oneiros_config::{Config, ServiceConfig};
use oneiros_context::Context;
use oneiros_detect_project_name::ProjectRoot;
use oneiros_http::HttpService;
use tempfile::TempDir;
use tokio::net::TcpListener;

use crate::*;

enum ServerState {
    Idle,
    Running(tokio::task::JoinHandle<()>),
}

pub(crate) struct TestHarness {
    _temp: TempDir,
    context: Context,
    server: ServerState,
}

impl TestHarness {
    pub(crate) fn new() -> Result<Self, Box<dyn core::error::Error>> {
        let temp = TempDir::new()?;
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
            server: ServerState::Idle,
        })
    }

    pub(crate) async fn bootstrap(self) -> Result<Self, Box<dyn core::error::Error>> {
        Bootstrap::builder().build().run(self).await
    }

    pub(crate) fn with_project(mut self, name: &str) -> Self {
        let project_root = self._temp.path().join("project");
        std::fs::create_dir_all(&project_root).expect("create project dir");

        self.context = Context::builder()
            .data_dir(self.context.data_dir().to_path_buf())
            .config_dir(self.context.config_dir().to_path_buf())
            .config(self.context.config().clone())
            .project(ProjectRoot::new(name, project_root))
            .build();

        self
    }

    pub(crate) async fn with_service(mut self) -> Result<Self, Box<dyn core::error::Error>> {
        let listener = TcpListener::bind("127.0.0.1:0").await?;
        let addr = listener.local_addr()?;

        self.context = Context::builder()
            .data_dir(self.context.data_dir().to_path_buf())
            .config_dir(self.context.config_dir().to_path_buf())
            .maybe_project(
                self.context
                    .project_name()
                    .map(|name| ProjectRoot::new(name, self.context.project_root().unwrap())),
            )
            .config(
                Config::builder()
                    .service(ServiceConfig::builder().port(addr.port()).build())
                    .build(),
            )
            .build();

        let http = HttpService::init(self.context.clone())?;

        self.server = ServerState::Running(tokio::spawn(async move {
            http.serve(listener).await.unwrap();
        }));

        Ok(self)
    }

    pub(crate) fn context(&self) -> &Context {
        &self.context
    }

    pub(crate) fn client(&self) -> oneiros_client::Client {
        self.context.client()
    }

    pub(crate) fn token(&self) -> Result<oneiros_model::Token, Box<dyn core::error::Error>> {
        Ok(self.context.ticket_token()?)
    }
}

impl Drop for TestHarness {
    fn drop(&mut self) {
        if let ServerState::Running(ref handle) = self.server {
            handle.abort();
        }
    }
}
