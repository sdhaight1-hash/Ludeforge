use ludeforge_core::{load, normalize, references, Diagnostic, MAX_INPUT_BYTES};
use std::env;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;
use std::process::ExitCode;

fn input(path: &Path) -> io::Result<Vec<u8>> {
    let mut bytes = Vec::new();
    File::open(path)?
        .take((MAX_INPUT_BYTES + 1) as u64)
        .read_to_end(&mut bytes)?;
    Ok(bytes)
}

fn new_output(path: &Path, bytes: &[u8]) -> io::Result<()> {
    // Never replace an existing path. A crash can leave an incomplete *new* file;
    // atomic replacement and recovery are deliberately not supported yet.
    let mut file = OpenOptions::new().write(true).create_new(true).open(path)?;
    if let Err(error) = file.write_all(bytes).and_then(|_| file.sync_all()) {
        drop(file);
        let _ = fs::remove_file(path);
        return Err(error);
    }
    Ok(())
}

fn diagnostics(ds: Vec<Diagnostic>) -> ExitCode {
    let stderr = io::stderr();
    let mut err = stderr.lock();
    let _ = serde_json::to_writer_pretty(&mut err, &ds);
    let _ = writeln!(err);
    ExitCode::from(1)
}

fn io_error(error: io::Error) -> ExitCode {
    diagnostics(vec![Diagnostic::new("IO", "", error.to_string())])
}

fn usage() -> ExitCode {
    eprintln!("Usage: ludeforge validate <project.json> | references <project.json> | normalize <project.json> <new-output.json>");
    ExitCode::from(2)
}

fn main() -> ExitCode {
    let args: Vec<_> = env::args_os().skip(1).collect();
    let command = args.first().and_then(|s| s.to_str()).unwrap_or("");
    let arity = match command {
        "validate" | "references" => 2,
        "normalize" => 3,
        _ => return usage(),
    };
    if args.len() != arity {
        return usage();
    }
    let bytes = match input(Path::new(&args[1])) {
        Ok(b) => b,
        Err(e) => return io_error(e),
    };
    let project = match load(&bytes) {
        Ok(p) => p,
        Err(d) => return diagnostics(d),
    };
    match command {
        "validate" => {
            println!("valid: {}", project.project_id);
            ExitCode::SUCCESS
        }
        "references" => match references(&project) {
            Ok(edges) => {
                let stdout = io::stdout();
                let mut out = stdout.lock();
                match serde_json::to_writer_pretty(&mut out, &edges) {
                    Ok(()) => match writeln!(out) {
                        Ok(()) => ExitCode::SUCCESS,
                        Err(e) => io_error(e),
                    },
                    Err(e) => diagnostics(vec![Diagnostic::new("SERIALIZE", "", e.to_string())]),
                }
            }
            Err(d) => diagnostics(d),
        },
        "normalize" => match normalize(&project) {
            Ok(bytes) => match new_output(Path::new(&args[2]), &bytes) {
                Ok(()) => ExitCode::SUCCESS,
                Err(e) => io_error(e),
            },
            Err(d) => diagnostics(d),
        },
        _ => usage(),
    }
}
