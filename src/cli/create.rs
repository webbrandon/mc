use structopt::StructOpt;

#[derive(Debug, StructOpt, Clone)]
#[structopt(raw(global_settings = "&[structopt::clap::AppSettings::DisableHelpSubcommand]"))]
pub struct Create {
    /// Use the api create guide.
    #[structopt(long = "guide", short = "g")]
    pub guide: bool,
    
    /// What API type you would like to create.
    #[structopt(long = "api", default_value = "mc")]
    pub api: String,
}