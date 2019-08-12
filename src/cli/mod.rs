pub mod completions;
pub use completions::Completions;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt, Default, Clone)]
#[structopt(raw(
    global_settings = "&[structopt::clap::AppSettings::DeriveDisplayOrder, structopt::clap::AppSettings::DisableVersion, structopt::clap::AppSettings::DisableHelpSubcommand]"
))]
pub struct Opt {
    /// Sets the "mc.yaml" file to use.
    #[structopt(long = "config", short = "c", parse(from_os_str))]
    pub file: Option<PathBuf>,

    /// Load from .env file.
    #[structopt(long = "env", short = "e", parse(from_os_str))]
    pub env: Option<PathBuf>,

    /// Silence output.
    #[structopt(short = "m", long = "mute")]
    pub mute: bool,

    /// Use flow pattern from mc.yaml.
    #[structopt(long = "flow", short = "f")]
    pub flow: Option<String>,

    /// Clone git repository.
    #[structopt(long = "repo", short = "r")]
    pub repo: Option<String>,

    /// Sets the script to run the the start.
    #[structopt(long = "script", short = "s", parse(from_os_str))]
    pub script: Option<PathBuf>,

    /// Sets a custom template file.
    #[structopt(long = "template", short = "t", parse(from_os_str))]
    pub template: Option<PathBuf>,

    /// Rendered template out file write location.
    #[structopt(long = "template-out", short = "o", parse(from_os_str))]
    pub template_out: Option<PathBuf>,

    /// Sets a custom template parameters file.
    #[structopt(long = "param", short = "p", parse(from_os_str))]
    pub param: Option<PathBuf>,

    /// Sets the script to run at the end.
    #[structopt(long = "post-script", short = "l", parse(from_os_str))]
    pub post_script: Option<PathBuf>,

    /// Turn prompt off for mc.yaml steps.
    #[structopt(long = "no-prompt", short = "n")]
    pub prompt: bool,

    #[structopt(subcommand)]
    pub commands: Option<Commands>,
}

#[derive(Debug, StructOpt, Clone)]
#[structopt(raw(
    global_settings = "&[structopt::clap::AppSettings::DeriveDisplayOrder, structopt::clap::AppSettings::DisableVersion, structopt::clap::AppSettings::DisableHelpSubcommand]"
))]
pub enum Commands {
    /// Completion scripts for various shells.
    #[structopt(
        raw(setting = "structopt::clap::AppSettings::DisableHelpSubcommand"),
        name = "completions"
    )]
    Completions(Completions),
}

impl Opt {
    pub fn new() -> Opt {
        Default::default()
    }
}
