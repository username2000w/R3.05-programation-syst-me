use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    count: u32,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    
    let mut tasks: Vec<tokio::task::JoinHandle<()>> = Vec::new();

    for i in 0..args.count {
        let task = tokio::spawn(async move {
            println!("Hello {}", i);
            println!("Au revoir {}", i);
        });

        tasks.push(task);
    }

    for task in tasks {
        task.await.unwrap();
    }
}