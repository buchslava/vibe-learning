use std::future::Future;
use tokio::time::{sleep, Duration};

fn lazy_job_manual() -> impl Future<Output = u32> {
    println!("sync prelude: runs on call");
    async move {
        println!("async body: first poll");
        sleep(Duration::from_millis(10)).await;
        42
    }
}

#[tokio::main]
async fn main() {
    println!("main: before call");
    let fut = lazy_job_manual();
    println!("main: have future, not awaited yet");
    let n = fut.await;
    println!("main: result = {}", n);
}
