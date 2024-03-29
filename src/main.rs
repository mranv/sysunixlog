extern crate syslog;

use syslog::{Facility, Formatter3164};

fn main() {
    // Initialize the logger
    let formatter = Formatter3164 {
        facility: Facility::LOG_USER,
        hostname: None,
        process: "sysunixlog".into(),
        pid: 0,
    };
    
    match syslog::unix(formatter) {
        Err(e) => println!("impossible to connect to syslog: {}", e),
        Ok(mut writer) => {
            // Log the message
            writer.err("Genesis of Generic Germination U0001F49A, Anubhav!").expect("could not write error message");
        }
    }
}

