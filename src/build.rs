extern crate clap;
extern crate structopt;

include!("cli/mod.rs");
use clap::Shell;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;
use std::path::Path;

fn get_top_template() -> String {
    String::from(
        r#"pub mod process;

pub use process::CompletionProcess;

pub struct Completions {
    
}

impl Completions {
    pub fn bash() {
        println!("{}",r#"
"#,
    )
}

fn get_fish_template() -> String {
    String::from(
        "
\"#);
}

pub fn fish() {
    println!(\"{}\",r#\"
",
    )
}

fn get_zsh_template() -> String {
    String::from(
        "
\"#);
}

pub fn zsh() {
    println!(\"{}\",r#\"
",
    )
}

fn get_ps1_template() -> String {
    String::from(
        "
\"#);
}

pub fn powershell() {
    println!(\"{}\",r#\"
",
    )
}

fn get_elvish_template() -> String {
    String::from(
        "
\"#);
}

pub fn elvish() {
    println!(\"{}\",r#\"
",
    )
}

fn get_bottom_template() -> String {
    String::from(
        "
\"#);
}
}
",
    )
}

fn main() {
    let app_name = "mc";
    let mut app = Opt::clap();

    let out_dir = Path::new("./src/completions/");
    let file_path = Path::new(out_dir.clone()).join("mod.rs");

    File::create(&file_path).unwrap();

    app.gen_completions(app_name, Shell::Bash, out_dir);
    app.gen_completions(app_name, Shell::Fish, out_dir);
    app.gen_completions(app_name, Shell::Zsh, out_dir);
    app.gen_completions(app_name, Shell::PowerShell, out_dir);
    app.gen_completions(app_name, Shell::Elvish, out_dir);

    let templates = vec![
        get_top_template(),
        get_fish_template(),
        get_zsh_template(),
        get_ps1_template(),
        get_elvish_template(),
    ];

    let completion_scripts = vec![
        Path::new("./src/completions/mc.bash"),
        Path::new("./src/completions/mc.fish"),
        Path::new("./src/completions/_mc"),
        Path::new("./src/completions/_mc.ps1"),
        Path::new("./src/completions/mc.elv"),
    ];

    for i in 0..5 {
        merge_files_to_completion(
            file_path.clone().to_path_buf(),
            completion_scripts[i].to_path_buf(),
            templates[i].clone(),
        );
    }
    file_to_completion(file_path, get_bottom_template());

    clean(completion_scripts);
}

fn clean(files: Vec<&Path>) {
    for i in 0..5 {
        match std::fs::remove_file(files[i].to_path_buf()) {
            Ok(x) => println!("{:#?}", x),
            Err(e) => eprint!("Error removing file: {}", e),
        }
    }
}

fn merge_files_to_completion(out: PathBuf, script: PathBuf, template: String) {
    let mut tmp_script = String::new();
    let mut file_script = File::open(script).expect("");

    file_script.read_to_string(&mut tmp_script).expect("");

    let mut outfile = OpenOptions::new()
        .write(true)
        .append(true)
        .open(out)
        .unwrap();
    outfile
        .write_fmt(format_args!("{}{}", template, tmp_script))
        .expect("Error writing updated completions.rs module.");
}

fn file_to_completion(out: PathBuf, template: String) {
    let mut outfile = OpenOptions::new()
        .write(true)
        .append(true)
        .open(out)
        .unwrap();
    outfile
        .write_fmt(format_args!("{}", template))
        .expect("Error writing updated completions.rs module.");
}
