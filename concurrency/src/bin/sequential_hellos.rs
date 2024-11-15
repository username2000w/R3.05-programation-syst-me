use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    count: u32,
}

fn main() {
    let args = Args::parse();

    for i in 0..args.count {
        println!("Hello {}", i);
        println!("Au revoir {}", i);
    }
}