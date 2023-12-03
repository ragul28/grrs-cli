use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let args = Cli::parse();
    let result = std::fs::read_to_string(&args.path)?;
    for line in result.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
    Ok(())
}