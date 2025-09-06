use clap::{Parser, ValueEnum};

#[derive(Clone, Debug, ValueEnum)]
enum SortingAlgorithm {
    Bogo,
    Bubble,
    Quick,
}

#[derive(Parser)]
struct Args {
    #[arg(long)]
    algorithm: SortingAlgorithm,

    #[arg(long)]
    num_elements: usize,
}

fn main() {
    let args = Args::parse();

    println!("Algorithm: {:?}", args.algorithm);
    println!("Elements: {}", args.num_elements);
}
