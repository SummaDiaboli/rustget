mod cli;
mod fetcher;

// pub use cli::Cli;
use fetcher::get_file;

pub fn start() {
	// Downloads the requested file
	get_file().unwrap();
}
