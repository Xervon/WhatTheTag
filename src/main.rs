mod args;

use log::{trace};

use structopt::StructOpt;
use pretty_env_logger::env_logger::Target;

use args::Args;

fn init_log(varname: &str) -> Result<(), log::SetLoggerError> {
    let mut builder = pretty_env_logger::formatted_builder();

    builder.target(Target::Stdout);

    if let Ok(s) = ::std::env::var(varname) {
        builder.parse_filters(&s);
    }

    builder.try_init()
}

fn main() -> () {
    init_log("RUST_LOG").unwrap();

    let args = Args::from_args();
    trace!("Parsed args: {:?}", args);

    ()
}
