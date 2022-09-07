use clap::Parser;

#[derive(Debug, Parser)]
struct Cli {
    #[clap(flatten)]
    permission: clap_permission_flag::Permission,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    args.permission.drop()?;
    Ok(())
}
