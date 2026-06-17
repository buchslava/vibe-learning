use std::time::Duration;

trait Gateway: Send + Sync {
    async fn connect(&self) -> Result<u32, &'static str>;
}

struct MockGateway {
    device_id: u32,
}

impl Gateway for MockGateway {
    async fn connect(&self) -> Result<u32, &'static str> {
        tokio::time::sleep(Duration::from_millis(5)).await;
        if self.device_id == 0 {
            return Err("invalid device");
        }
        Ok(self.device_id)
    }
}

async fn run<G: Gateway + Send + 'static>(gw: G) {
    match gw.connect().await {
        Ok(id) => println!("connected device {}", id),
        Err(e) => println!("connect failed: {}", e),
    }
}

#[tokio::main]
async fn main() {
    let gw = MockGateway { device_id: 42 };
    run(gw).await;

    let handle = tokio::spawn(run(MockGateway { device_id: 7 }));
    handle.await.expect("join");
}
