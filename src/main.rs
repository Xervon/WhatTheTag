mod args;

use log::{trace};

use structopt::StructOpt;
use pretty_env_logger::env_logger::Target;

use args::Args;

use wtt_lib::{ WTTResult, Result };

fn init_log(varname: &str) -> Result<()> {
    let mut builder = pretty_env_logger::formatted_builder();

    builder.target(Target::Stdout);

    if let Ok(s) = ::std::env::var(varname) {
        builder.parse_filters(&s);
    }

    builder.try_init()?;

    Ok(())
}

fn main() -> WTTResult {
    init_log("RUST_LOG")?;

    let args = Args::from_args();
    trace!("Parsed args: {:?}", args);

    WTTResult::Ok
}
