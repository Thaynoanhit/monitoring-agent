use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Root};
use std::error::Error;

pub fn setup_logging() -> Result<(), Box<dyn Error>> {
    std::fs::create_dir_all("logs")?;

    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} [{l}] - {m}\n")))
        .build("logs/monitoring_agent.log")?;

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(
            Root::builder()
                .appender("logfile")
                .build(LevelFilter::Info)
        )?;

    log4rs::init_config(config)?;
    log::info!("Logging system initialized successfully");
    
    Ok(())
}
