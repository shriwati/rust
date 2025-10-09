
use log::{info, trace, debug, warn, error};
use log4rs;

pub fn init_logging() {
    // init log file
    log4rs::init_file("log_config.yaml", Default::default()).unwrap();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logging() {
        init_logging();
        trace!("detailed tracing info");
        debug!("debug info");
        info!("relevant general info");
        warn!("warning this program doesn't do much");
        error!("error message here");
    }
}
