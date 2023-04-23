pub mod providers;
pub use crate::providers::prelude::*;

use std::sync::{ Arc, Mutex };
use std::time::Duration;
use std::mem;

use log::{Log, SetLoggerError};
use anyhow::Result;
use serde::Serialize;
use async_trait::async_trait;
use crossbeam::channel;

use tokio::runtime::Runtime;
use tokio::task;
use tokio::time::sleep;



#[async_trait]
pub trait LogProvider: Send + Sync {
    async fn send_log(&self, messages: Vec<LogAnywhereRecord>);
}


#[derive(Clone)]
pub struct LogAnywhereLogger {
    provider: Arc<dyn LogProvider>,
    log_buffer_records: Arc<Mutex<Vec<LogAnywhereRecord>>>
}

async fn buffer_loop(log_buffer_records: Arc<Mutex<Vec<LogAnywhereRecord>>>, provider: Arc<dyn LogProvider>) {
    loop {
        let messages = {
            let mut records_guard = log_buffer_records.lock().unwrap();
            mem::take(&mut *records_guard)
        };
        provider.send_log(messages).await;
        sleep(Duration::from_secs(3)).await
    }
}

impl LogAnywhereLogger {
    pub fn new(provider: Arc<dyn LogProvider>) -> Self {
        LogAnywhereLogger {
            provider,
            log_buffer_records: Arc::new(Mutex::new(Vec::new()))
        }
    }

    pub fn init(self: Box<Self>) -> Result<(), SetLoggerError> {
        let buff_rec_clone = Arc::clone(&self.log_buffer_records);
        let provider_clone = Arc::clone(&self.provider);
        task::spawn(buffer_loop(buff_rec_clone, provider_clone));

        log::set_boxed_logger(self)?;
        log::set_max_level(log::LevelFilter::Info);
        Ok(())
    }
}

#[derive(Debug, Serialize)]
pub struct LogAnywhereRecord {
    level: String,
    message: String,
    file: String,
    line: Option<u32>
}

unsafe impl Sync for LogAnywhereLogger {}
unsafe impl Send for LogAnywhereLogger {}

impl Log for LogAnywhereLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        true 
    }

    fn log(&self, record: &log::Record) {
        let anywhere_log = LogAnywhereRecord {
            level: record.level().to_string(),
            message: record.args().to_string(),
            file: record.file().unwrap().to_string(),
            line: record.line()
        };
        self.log_buffer_records.lock().unwrap().push(anywhere_log);
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

