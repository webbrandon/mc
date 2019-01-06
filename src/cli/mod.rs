use clap::{Arg, App, AppSettings};

pub fn build_cli() -> App<'static, 'static> {
    App::new("mc")
        .version("0.4.0")
        .setting(AppSettings::DisableVersion)
        .about("\nContinuous development for todays software made easy. You can import files individually or with a \"mc.yaml\" file. Option settings will override use of \"mc.yaml\" file.")
        .arg(Arg::with_name("config")
            .help("Sets the \"mc.yaml\" file to use.")
            .short("f")
            .long("file")
            .value_name("CONFIG")
            .required(false))        
        .arg(Arg::with_name("env")
            .help("Load from .env file.")
            .short("e")
            .long("env")
            .value_name("ENV")
            .required(false))
        .arg(Arg::with_name("template")
           .short("t")
           .long("template")
           .value_name("TEMPLATE")
           .help("Sets a custom template file")
           .takes_value(true))
        .arg(Arg::with_name("param-script")
            .long("param-script")
            .value_name("PARAM_SCRIPT")
            .help("Sets a custom script to configure parameters file at render time.")
            .takes_value(true))
         .arg(Arg::with_name("parameters")
             .short("p")
             .long("param")
             .value_name("PARAM")
             .help("Sets a custom template parameters file.")
             .takes_value(true))
        .arg(Arg::with_name("template-out")
            .short("o")
            .long("template-out")
            .value_name("OUT")
            .help("Rendered template out file write location.")
            .takes_value(true))
        .arg(Arg::with_name("build-script")
           .help("Sets the script file to use for setting building software.")
           .short("b")
           .long("build-script")
           .value_name("BUILD_SCRIPT")
           .required(false))
        .arg(Arg::with_name("deploy-script")
           .help("Sets the script file to use after _build script_.")
           .short("d")
           .long("deploy-script")
           .value_name("DEPLOY_SCRIPT")
           .required(false))
        .arg(Arg::with_name("post-script")
           .help("Sets the script file to use after configuring template.")
           .short("s")
           .long("post-script")
           .value_name("POST_SCRIPT")
           .required(false))
        .arg(Arg::with_name("mute")
           .help("Silence output.")
           .short("m")
           .long("mute")
           .required(false))
        .arg(Arg::with_name("no-deploy")
           .help("Skip deploy step from mc.yaml.")
           .long("no-deploy")
           .required(false))
        .arg(Arg::with_name("no-build")
           .help("Skip build step from mc.yaml.")
           .long("no-build")
           .required(false))
        .arg(Arg::with_name("no-post")
           .help("Skip post build step from mc.yaml.")
           .long("no-post")
           .required(false))
        .arg(Arg::with_name("no-template")
           .help("Skip template step from mc.yaml settings.")
           .long("no-template")
           .required(false))
        .arg(Arg::with_name("no-prompt")
           .help("Turn prompt off for mc.yaml steps.")
           .long("no-prompt")
           .required(false))
}