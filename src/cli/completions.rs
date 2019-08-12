use structopt::StructOpt;
#[derive(Debug, StructOpt, Clone)]
#[structopt(raw(global_settings = "&[structopt::clap::AppSettings::DisableHelpSubcommand]"))]
pub enum Completions {
    /// Bash completion script.
    #[structopt(
        raw(setting = "structopt::clap::AppSettings::DisableHelpSubcommand"),
        name = "bash"
    )]
    Bash(Bash),

    /// Fish completion script.
    #[structopt(
        raw(setting = "structopt::clap::AppSettings::DisableHelpSubcommand"),
        name = "fish"
    )]
    Fish(Fish),

    /// Zsh completion script.
    #[structopt(
        raw(setting = "structopt::clap::AppSettings::DisableHelpSubcommand"),
        name = "zsh"
    )]
    Zsh(Zsh),

    /// PowerShell completion script.
    #[structopt(
        raw(setting = "structopt::clap::AppSettings::DisableHelpSubcommand"),
        name = "powershell"
    )]
    PowerShell(PowerShell),

    /// Elvish completion script.
    #[structopt(
        raw(setting = "structopt::clap::AppSettings::DisableHelpSubcommand"),
        name = "elvish"
    )]
    Elvish(Elvish),
}

#[derive(Debug, StructOpt, Default, Clone)]
#[structopt(raw(global_settings = "&[structopt::clap::AppSettings::DisableHelpSubcommand]"))]
pub struct Bash {
    /// Bash completion script.
    #[structopt(default_value = "bash")]
    pub name: String,
}

#[derive(Debug, StructOpt, Default, Clone)]
#[structopt(raw(global_settings = "&[structopt::clap::AppSettings::DisableHelpSubcommand]"))]
pub struct Fish {
    /// Fish completion script.
    #[structopt(default_value = "fish")]
    pub name: String,
}

#[derive(Debug, StructOpt, Default, Clone)]
#[structopt(raw(global_settings = "&[structopt::clap::AppSettings::DisableHelpSubcommand]"))]
pub struct Zsh {
    /// Zsh completion script.
    #[structopt(default_value = "zsh")]
    pub name: String,
}

#[derive(Debug, StructOpt, Default, Clone)]
#[structopt(raw(global_settings = "&[structopt::clap::AppSettings::DisableHelpSubcommand]"))]
pub struct PowerShell {
    /// PowerShell completion script.
    #[structopt(default_value = "powershell")]
    pub name: String,
}

#[derive(Debug, StructOpt, Default, Clone)]
#[structopt(raw(global_settings = "&[structopt::clap::AppSettings::DisableHelpSubcommand]"))]
pub struct Elvish {
    /// Elvish completion script.
    #[structopt(default_value = "elvish")]
    pub name: String,
}
