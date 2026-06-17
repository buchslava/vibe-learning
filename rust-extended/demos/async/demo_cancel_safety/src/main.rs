use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time;

async fn producer(tx: mpsc::Sender<u32>) {
    for i in 1..=5 {
        tx.send(i).await.expect("send");
        time::sleep(Duration::from_millis(20)).await;
    }
}

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(8);
    tokio::spawn(producer(tx));

    // State outside select! — survives branch cancellation.
    let mut collected = Vec::new();

    loop {
        tokio::select! {
            _ = time::sleep(Duration::from_millis(35)) => {
                println!("timeout branch won; collected so far: {:?}", collected);
                break;
            }
            maybe = rx.recv() => {
                match maybe {
                    Some(n) => {
                        collected.push(n);
                        println!("got {}", n);
                    }
                    None => break,
                }
            }
        }
    }
    println!("final collected: {:?}", collected);
}
