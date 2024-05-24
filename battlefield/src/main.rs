use walkdir::WalkDir;

fn main() {
	for entry in WalkDir::new("/Users/alexkazakov/src/github/angry-cellophane/rust-hello-world") {
		print!("path = {:?}", &entry);
	}
}
