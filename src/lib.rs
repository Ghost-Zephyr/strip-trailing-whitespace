use std::{fs, io::{self, Read, Write}};

pub fn strip_file(file: &str) -> io::Result<()> {
    let mut contents = String::new();
    fs::File::open(file)?.read_to_string(&mut contents)?;
    fs::write(file, strip(&contents).as_bytes())?;
    Ok(())
}

pub fn strip(inp: &str) -> String {
    let mut out = String::new();
    for line in inp.lines() {
        out += &strip_line(line);
        out += "\n";
    }
    out
}

pub fn strip_line(inp: &str) -> String {
    let mut idx = inp.len();
    while let Some(_) = inp[..idx].strip_suffix(" ") {
        idx -= 1;
    }
    inp[..idx].to_string()
}

#[test]
fn test_strip() {
    assert_eq!("balle", strip("balle"));
    assert_eq!("balle", strip("balle "));
    assert_eq!("balle", strip("balle  "));
    assert_eq!("balle", strip("balle   "));
}
