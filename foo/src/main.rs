use logger::{ DanLogger, AxiomProvider, DbProvider };
use log::{self, error};

fn main() {
    let provider = Box::new(DbProvider::new());
    let logger = DanLogger::new(provider);
    
    let boxed_logger = Box::new(logger);
    boxed_logger.init().unwrap();

    error!("Logging from dan error");
}
