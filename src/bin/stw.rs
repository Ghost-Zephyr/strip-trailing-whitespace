use strip_trailing_whitespace::{strip_file, strip_line};
use std::{fs, io::{self, Write}};
use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    files: Option<Vec<String>>,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    if let Some(files) = args.files {
        for file in files {
            strip_file(&file)?;
        }
    } else {
        let inp = io::stdin().lines();
        let mut out = io::stdout();

        for line in inp {
            out.write(strip_line(&line?).as_bytes())?;
            out.write(&[b'\n'])?;
            out.flush()?;
        }
    }

    Ok(())
}
