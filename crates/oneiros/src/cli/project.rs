use clap::Parser;

#[derive(Parser, Clone, Default)]
pub struct ProjectConfig {
    #[clap(default_value = env!("CARGO_PKG_NAME"))]
    pub(crate) name: String,
}
