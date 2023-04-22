pub mod providers;

pub use crate::providers::prelude::*;
use std::sync::Arc;
use log::{Log, SetLoggerError};
use anyhow::Result;
use serde::Serialize;
use async_trait::async_trait;
use crossbeam::channel;
use tokio::runtime::Runtime;



#[async_trait]
pub trait LogProvider: Send + Sync {
    async fn send_log(&self, message: AnywhereLogRecord);
}


pub struct DanLogger {
    provider: Arc<dyn LogProvider>
}

impl DanLogger {
    pub fn new(provider: Arc<dyn LogProvider>) -> Self {
        DanLogger {
            provider
        }
    }

    pub fn init(self: Box<Self>) -> Result<(), SetLoggerError> {
        log::set_boxed_logger(self)?;
        log::set_max_level(log::LevelFilter::Info);
        Ok(())
    }
}

#[derive(Debug, Serialize)]
pub struct AnywhereLogRecord {
    level: String,
    message: String,
    file: String,
    line: Option<u32>
}

unsafe impl Sync for DanLogger {}
unsafe impl Send for DanLogger {}

impl Log for DanLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        true 
    }

    fn log(&self, record: &log::Record) {
        let anywhere_log = AnywhereLogRecord {
            level: record.level().to_string(),
            message: record.args().to_string(),
            file: record.file().unwrap().to_string(),
            line: record.line()
        };
        
        let rt = Runtime::new().unwrap();
        let handle = rt.handle();

        let (tx, rx) = channel::bounded(1);
        let provider = self.provider.clone();

        handle.spawn(async move {
            tx.send(provider.send_log(anywhere_log).await).unwrap();
        });

        let rec = rx.recv();
        if let Err(e) = rec {
            eprintln!("Error encountered when receiving log thread, error: {}", e);
        }

    }

    fn flush(&self) {
        
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // let result = init(2, 2);
        assert_eq!(4, 4);
    }
}

