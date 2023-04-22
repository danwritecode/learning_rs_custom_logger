use async_trait::async_trait;
use crate::{LogProvider, AnywhereLogRecord};

pub struct DbProvider<'a> {
    db_conn: &'a str
}

impl<'a> DbProvider<'a> {
    pub fn new() -> DbProvider<'a> {
        let db_conn = "";
        DbProvider {
            db_conn
        }
    }
}

#[async_trait]
impl<'a> LogProvider for DbProvider<'a> {
    async fn send_log(&self, message: AnywhereLogRecord) {
        println!("Dan Logger logged for DB: {:?}", message);
    }
}
