use std::fs::File;
use std::io::ErrorKind;

fn main() {
    File::open("file").unwrap_or_else(|error| {
        match error.kind() {
            ErrorKind::NotFound => match File::create("file") {
                Ok(fc) => fc,
                Err(ex) => panic!("cannot create file"),
            },
            other => {
                panic!("unknown error {:?}", other);
            }
        }
    });
}
