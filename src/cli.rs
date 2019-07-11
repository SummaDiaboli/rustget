extern crate quicli;
extern crate structopt;

use quicli::prelude::*;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
pub struct Cli {
	#[structopt(short = "u", long = "url", help = "URL to fetch from")]
	// Get the file url
	pub url: String,

	#[structopt(short = "d", long = "dest", help = "File path including name")]
	// The file destination and name
	pub filename: String,

	#[structopt(
		short = "r",
		long = "retry",
		default_value = "0",
		help = "Number of time to retry"
	)]
	// Number of time to retry the download
	pub retry: u32,

	// Number of threads to use for download
	#[structopt(
		short = "t",
		long = "threads",
		default_value = "4",
		help = "Number of threads to use for download"
	)]
	pub threads: usize,

	#[structopt(flatten)]
	// Verbose output
	pub verbose: Verbosity,
}
