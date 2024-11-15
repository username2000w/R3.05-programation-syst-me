use std::sync::{Arc, RwLock};

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    count: u32,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    
    let mut tasks: Vec<tokio::task::JoinHandle<()>> = Vec::new();
    let number = Arc::new(RwLock::new(0));

    for _ in 0..args.count {
        let number_clone = Arc::clone(&number);
        
        let task = tokio::spawn(async move {
            let mut number_clone = number_clone.write().unwrap();

            println!("Hello {}", number_clone);
            println!("Au revoir {}", number_clone);

            *number_clone += 1;
        });

        tasks.push(task);
    }

    for task in tasks {
        task.await.unwrap();
    }
}