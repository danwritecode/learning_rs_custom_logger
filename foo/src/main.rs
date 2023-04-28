use std::sync::Arc;
use log_anywhere::{ LogAnywhereLogger, AxiomProvider };
use log::{self, error, info, warn, debug, trace, LevelFilter};

use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let token = "".to_string();
    let dataset = "worker_logs".to_string();

    let provider = Arc::new(AxiomProvider::new(token, dataset));

    let logger = LogAnywhereLogger::new(provider, 5, LevelFilter::Info);
    let boxed_logger = Box::new(logger);
    boxed_logger.init().unwrap();


    panic!("Panicked here")

    // let mut counter = 0;
    // loop {
    //     counter += 1;
    //     println!("Info logging num: {}", counter);
    //     info!("Info logging num: {}", counter);
    //     sleep(Duration::from_secs(1)).await
    // }
}
