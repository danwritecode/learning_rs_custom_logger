use log::{Log, SetLoggerError};

// Providers
pub trait LogProvider {
    fn send_log(&self, message: &log::Record);
}

pub struct AxiomProvider {
    auth_token: String
}

impl AxiomProvider {
    pub fn new() -> AxiomProvider {
        let auth_token = "".to_string();
        AxiomProvider {
            auth_token
        }
    }
}

impl LogProvider for AxiomProvider {
    fn send_log(&self, message: &log::Record) {
        println!("Dan Logger logged for Axiom: {:?}", message);
    }
}

pub struct DbProvider {
    db_conn: String
}

impl DbProvider {
    pub fn new() -> DbProvider {
        let db_conn = "".to_string();
        DbProvider {
            db_conn
        }
    }
}

impl LogProvider for DbProvider {
    fn send_log(&self, message: &log::Record) {
        println!("Dan Logger logged for DB: {:?}", message);
    }
}



// Logger
pub struct DanLogger {
    provider: Box<dyn LogProvider>
}

impl DanLogger {
    pub fn new(provider: Box<dyn LogProvider>) -> Self {
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


unsafe impl Sync for DanLogger {}
unsafe impl Send for DanLogger {}

impl Log for DanLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        true 
    }

    fn log(&self, record: &log::Record) {
        self.provider.send_log(&record);
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

