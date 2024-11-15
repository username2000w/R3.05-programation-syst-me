use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    count: u32,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    
    let mut tasks: Vec<tokio::task::JoinHandle<()>> = Vec::new();
    let mut total = 0;

    for _ in 0..args.count {
        let task = tokio::spawn(async move {
            total += 1;

            println!("Hello {}", total);
            println!("Au revoir {}", total);

        });

        tasks.push(task);
    }

    for task in tasks {
        task.await.unwrap();
    }
}