use clap::Parser;

#[derive(Parser)]
#[clap(
    name = "CoreOverlay",
    about = "CoreOverlay Main Server",
    version = "0.0.1"
)]
struct Cli {
    #[clap(short, long)]
    name: String,

    #[clap(short, long, default_value = "1")]
    count: u32,
}

fn main() {
    let cli = Cli::parse();

    for _ in 0..cli.count {
        println!("Hello, {}!", cli.name);
    }
}
