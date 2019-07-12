extern crate parallel_getter;
extern crate reqwest;

use std::fs::{remove_file, File};
use std::path::PathBuf;
use std::sync::Arc;
use std::thread;

use indicatif::{ProgressBar, ProgressStyle};
use parallel_getter::ParallelGetter;
use quicli::prelude::*;
use reqwest::Client;
use structopt::StructOpt;

use crate::cli::Cli;

pub fn get_file() -> CliResult {
	// Collect the arguments from the cli file
	let args = Cli::from_args();
	args.verbose.setup_env_logger("head")?;

	let url = args.url;
	let filename = args.filename;
	let retries = args.retry;
	let threads = args.threads;
	// let mut attempts = 0; // Number of attempts to download file

	let mut done = false;

	let task1 = thread::spawn(move || {
		let result = fetch_file_from_url(&url, &filename, threads, retries);
		match result {
			Ok(_x) => {
				done = true;
				// break;
			}
			Err(x) => {
				println!("Error: {}", x);
				remove_file(&filename).unwrap();
			}
		}
		done = true;
	});

	thread::spawn(move || {
		// The trick to keep the spinner going until
		// the download finishes or comes to an error
		// is to continuously increase the value the spinner is spinning at
		// by increasing the value of x

		let mut x = 1;
		let bar = ProgressBar::new_spinner();
		bar.set_style(
			ProgressStyle::default_spinner().template("{msg} {spinner:2.red} [{elapsed}]"),
		);
		bar.set_message("Downloading...");
		while done == false {
			bar.inc(x);
			x += 1;
		}
		bar.finish();
	});

	task1.join().expect("Something went wrong");
	println!("Finished");

	Ok(())
}

fn fetch_file_from_url(
	url: &String,
	filename: &String,
	threads: usize,
	retries: u32,
) -> Result<(), std::io::Error> {
	let client = Arc::new(Client::new());
	let mut file = File::create(&filename).unwrap();

	ParallelGetter::new(&url, &mut file)
		.client(client)
		// Create cache path
		.cache_path(PathBuf::from("~/.cache/curlr"))
		// Number of threds to spawn
		.threads(threads)
		// threshold (length in bytes) to determine when multiple threads are required.
		.threshold_parallel(1 * 1024 * 1024)
		// threshold for defining when to store parts in memory or on disk.
		.threshold_memory(10 * 1024 * 1024)
		// number of times to retry download
		.retries(retries)
		// Commit the parallel GET requests.
		.get()?;

	Ok(())
}
