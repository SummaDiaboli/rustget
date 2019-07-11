mod cli;
mod fetcher;

fn main() {
    start();
}

fn start() {
	// Downloads the requested file
	fetcher::get_file().unwrap();
}