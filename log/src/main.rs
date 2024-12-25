use log::{ info,trace,debug,warn,error};
use log4rs;



fn main() {
    // init log file
    log4rs::init_file("log_config.yaml", Default::default()).unwrap();

    trace!("detailed tracing info");
    debug!("debug info");
    info!("relevant general info");
    warn!("warning this program doesn't do much");
    error!("error message here");

}

