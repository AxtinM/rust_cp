use std::path::Path;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let from_file_path = Path::new(&args[1]);
    let to_file_path = Path::new(&args[2]);
    if from_file_path.exists() {
        if to_file_path.exists() {
            fs::File::create(&args[2]).expect("Cannot Create File For Some Reason ...");
            fs::copy(&args[1], &args[2]).expect("Cannot Copy file content !");
        }
        else {
            fs::copy(&args[1], &args[2]).expect("Cannot Copy file content !");
        }
    }
}
