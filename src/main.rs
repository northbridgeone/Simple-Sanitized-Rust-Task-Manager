use std::time::{Duration, Instant};

#[macro_use]
extern crate log;
use anyhow::Context;
use dialoguer::Input;
use sd_bus::object_server;
use sd_bus::Error as BusError;
use sd_bus::VariantType;
use structopt::StructOpt;
use tokio::task;

mod cli;
mod config;
mod controller;
mod service_watcher;

#[derive(Debug)]
enum MyError {
    Io(std::io::Error),
    Logic(String),
}
impl From<std::io::Error> for MyError {
    fn from(err: std::io::Error) -> Self {
        MyError::Io(err)
    }
}
impl From<BusError> for MyError {
    fn from(err: BusError) -> Self {
        MyError::Logic(format!("{:?}", err))
    }
}

#[tokio::main]
async fn main() -> Result<(), MyError> {
    simple_logger::init_with_env().context("Failed to initialize logger")?;
    let opt = cli::Options::from_args();

    match opt.command {
        cli::Command::Watch { interval } => watch_services(interval).await?,
        _ => println!("{}", opt.help()), // Handle other cases not covered in this example
    };

    Ok(())
}

async fn watch_services(interval: u64) -> Result<(), MyError> {
    let mut interval = Duration::from_secs(interval);
    loop {
        let (controller, bus) = controller::new().await?;
        task::spawn(service_watcher::run(&bus));

        let next_wake = Instant::now() + interval;
        controller.poll_loop().await?;

        interval = next_wake.duration_since(Instant::now());
    }
}
