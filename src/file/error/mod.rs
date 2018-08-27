use std::process;
use std::io::{self, Write};

pub fn bad_format_close_app() -> ! {
    writeln!(
        &mut io::stderr(),
        "{}",
        r#"Format of files is not accepted."#
    ).ok();
    process::exit(1);
}