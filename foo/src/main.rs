use std::sync::Arc;
use logger::{ LogAnywhereLogger, AxiomProvider };
use log::{self, error, info, warn, debug, trace};

use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let token = "";
    let dataset = "worker_logs";

    // buffer here

    let provider = Arc::new(AxiomProvider::new(token, dataset));

    let logger = LogAnywhereLogger::new(provider);
    let boxed_logger = Box::new(logger);
    boxed_logger.init().unwrap();

    let mut counter = 0;
    loop {
        counter += 1;
        println!("Info logging num: {}", counter);
        info!("Info logging num: {}", counter);
        sleep(Duration::from_secs(1)).await
    }
}
