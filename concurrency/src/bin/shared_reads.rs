use std::sync::Arc;

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    count: u32,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    
    let mut tasks: Vec<tokio::task::JoinHandle<()>> = Vec::new();
    let shared = Arc::new("Hello".to_string());

    for i in 0..args.count {
        let shared_clone = shared.clone();
        
        let task = tokio::spawn(async move {
            println!("{} {}", shared_clone, i);
            println!("Au revoir {}", i);
        });

        tasks.push(task);
    }

    for task in tasks {
        task.await.unwrap();
    }
}