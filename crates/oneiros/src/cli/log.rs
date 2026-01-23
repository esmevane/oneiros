use clap::Parser;
use clap_verbosity_flag::Verbosity;
use colorchoice_clap::Color;

#[derive(Parser, Clone, Default)]
pub(crate) struct LogConfig {
    #[command(flatten)]
    pub(crate) verbosity: Verbosity,

    #[command(flatten)]
    pub(crate) color: Color,
}
