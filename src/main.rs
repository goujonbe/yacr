use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Cli {
    operation: String,
    container_id: String,
    #[structopt(parse(from_os_str), required_if("operation", "create"))]
    bundle_path: Option<std::path::PathBuf>,
}

fn main() {
    let cli = Cli::from_args();
    println!("{:?}", cli);
}
