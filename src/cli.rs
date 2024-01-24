use structopt::StructOpt;

/// Simple Systemd Service Manager
#[derive(Debug, StructOpt)]
#[structopt(name = "systemd_manager")]
struct Options {
    /// Command to run
    #[structopt(subcommand)]
    command: Commands,
}

#[derive(Debug, StructOpt)]
enum Commands {
    /// Watch services at regular intervals
    Watch {
        /// Interval in seconds between checks
        #[structopt(short, long, default_value = "5")]
        interval: u64,
    },
    // Add more commands later
}
