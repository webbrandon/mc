use clap::{Arg, App, AppSettings};

pub fn build_cli() -> App<'static, 'static> {
    App::new("mc")
        .version("0.1.0")
        .setting(AppSettings::DisableVersion)
        .about("\nBuild configuration for todays software made easy. You can import files individually or with a \"Makeconfig\" YAML file. Option settings will override use of \"Makeconfig\" file.")
        .arg(Arg::with_name("config")
            .help("Sets the \"Makeconfig\" file to use.")
            .short("f")
            .long("file")
            .value_name("CONFIG")
            .required(false))
        .arg(Arg::with_name("template")
           .short("t")
           .long("template")
           .value_name("TEMPLATE")
           .help("Sets a custom template file")
           .takes_value(true))
        .arg(Arg::with_name("parameters")
            .short("p")
            .long("params")
            .value_name("PARAMS")
            .help("Sets a custom template parameters file.")
            .takes_value(true))
        .arg(Arg::with_name("template-out")
            .short("o")
            .long("template-out")
            .value_name("OUT")
            .help("Rendered template out file write location.")
            .takes_value(true))
        .arg(Arg::with_name("script")
           .help("Sets the script file to use for setting template parameters.")
           .short("s")
           .long("script")
           .value_name("SCRIPT")
           .required(false))
        .arg(Arg::with_name("post-script")
           .help("Sets the script file to use after configuring template.")
           .short("S")
           .long("post-script")
           .value_name("POST_SCRIPT")
           .required(false))
        .arg(Arg::with_name("mute")
           .help("Silence output.")
           .short("m")
           .long("mute")
           .required(false))
}