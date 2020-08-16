use structopt::StructOpt;
use std::path::PathBuf;

#[derive(StructOpt, Debug)]
pub enum Command {
}

#[derive(StructOpt, Debug)]
#[structopt(name = "WTT - WhatTheTag")]
pub struct Args {
    /// WTT config file
    #[structopt(short, long, parse(from_os_str))]
    pub config: Option<PathBuf>,

    #[structopt(subcommand)]
    pub command: Option<Command>,
}
