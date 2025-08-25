use strip_trailing_whitespace::{strip_file, strip_line};
use std::io::{self, Write};
use walkdir::WalkDir;
use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    paths: Option<Vec<String>>,

    #[arg(short, long)]
    recurse: bool,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    if let Some(paths) = args.paths {
        let files = if args.recurse {
            let mut files = vec![];
            for path in paths {
                files.append(&mut recurse(&path))
            }
            files
        } else {
            paths
        };
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

fn recurse(path: &str) -> Vec<String> {
    WalkDir::new(path).into_iter().filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| e.path().to_string_lossy().to_mut().clone()).collect()
}
