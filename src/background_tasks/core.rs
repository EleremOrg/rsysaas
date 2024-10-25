use std::time::Duration;

use futures::future::BoxFuture;
use stefn::shutdown_signal;
use tokio::time::sleep;

pub fn run() -> BoxFuture<'static, Result<(), std::io::Error>> {
    Box::pin(async {
        loop {
            tokio::select! {
                _ = shutdown_signal() => {
                    println!("Exiting due to shutdown signal...");
                    break;
                }
                _ = task() => {}
            }
        }
        Ok(())
    })
}

async fn task() {
    println!("doing the job");
    sleep(Duration::new(5, 0)).await;
    println!("job ended");
}
