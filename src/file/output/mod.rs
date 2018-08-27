use std::fs::File;
use std::io::{self, BufWriter, Write};

pub fn outfile(filepath: &String, text: &String) {
    let f = File::create(filepath).expect("Unable to create file");
    let mut f = BufWriter::new(f);
    f.write_all(text.as_bytes()).expect("Unable to write data");
}

pub fn out_term(text: &String) {
    writeln!(
        &mut io::stdout(),
        "{}",
        text
    ).ok();
}
