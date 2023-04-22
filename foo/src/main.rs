use std::sync::Arc;
use logger::{ DanLogger, AxiomProvider };
use log::{self, error, info, warn, debug, trace};


fn main() {
    let token = "";
    let dataset = "worker_logs";

    let provider = Arc::new(AxiomProvider::new(token, dataset));

    let logger = DanLogger::new(provider);
    let boxed_logger = Box::new(logger);
    boxed_logger.init().unwrap();

    info!("Info logging");
    warn!("warn logging");
    error!("Error logging");
}
