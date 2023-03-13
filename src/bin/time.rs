use chrono::prelude::*;
use serde::Serialize;
use std::time::SystemTime;

pub fn main() {
    let now = SystemTime::now();
    println!("SystemTime now: {:?}", now);

    #[derive(Serialize)]
    struct Nows {
        utc: DateTime<Utc>,
        local: DateTime<Local>,
    }

    let nows: Nows = Nows {
        utc: Utc::now(),
        local: Local::now(),
    };

    println!("Chrono Local now: {}", nows.local);

    let res = serde_json::to_string(&nows).unwrap();
    println!(" as JSON {}", res);
}
