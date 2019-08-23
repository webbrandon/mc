pub mod completions;
pub mod create;

pub use completions::Completions;
pub use create::Create;

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt, Default, Clone)]
#[structopt(raw(
    global_settings = "&[structopt::clap::AppSettings::DeriveDisplayOrder, structopt::clap::AppSettings::DisableVersion, structopt::clap::AppSettings::DisableHelpSubcommand]"
))]
pub struct Opt {
    /// Define which api config file to use.
    #[structopt(long = "config", short = "c", parse(from_os_str))]
    pub file: Option<PathBuf>,

    /// Load dot env file. Overrides env-prompts api configs.
    #[structopt(long = "env", short = "e", parse(from_os_str))]
    pub env: Option<PathBuf>,

    /// Proceed prompts default to yes and env-prompts are disabled.
    #[structopt(long = "no-prompt", short = "n")]
    pub prompt: bool,

    /// Silence output.
    #[structopt(short = "m", long = "mute")]
    pub mute: bool,

    /// Use flow pattern.
    #[structopt(long = "flow", short = "f")]
    pub flow: Option<String>,

    /// Clone git repository.
    #[structopt(long = "repo", short = "r")]
    pub repo: Option<String>,

    /// Docker image to run in.
    #[structopt(long = "docker", short = "d")]
    pub docker: Option<String>,

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

    #[structopt(subcommand)]
    pub commands: Option<Commands>,
}

#[derive(Debug, StructOpt, Clone)]
#[structopt(raw(
    global_settings = "&[structopt::clap::AppSettings::DeriveDisplayOrder, structopt::clap::AppSettings::DisableVersion, structopt::clap::AppSettings::DisableHelpSubcommand]"
))]
pub enum Commands {
    /// Create api files.
    #[structopt(
        raw(setting = "structopt::clap::AppSettings::DisableHelpSubcommand"),
        name = "create"
    )]
    Create(Create),
    
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
