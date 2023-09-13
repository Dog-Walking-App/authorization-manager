use clap::Parser;

/// Web server
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The interface to bind to.
    #[arg(long, default_value_t = ("127.0.0.1").to_string())]
    pub host: String,

    /// The port to bind to.
    #[arg(short, long, default_value_t = 8080)]
    pub port: u16,
}

impl Args {
    pub fn get() -> Self {
        Args::parse()
    }
}