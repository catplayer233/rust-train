use std::fs::File;
use std::io;
use std::io::{ErrorKind, Read};
use std::path::Path;

pub fn explore() {
    let path = Path::new("./test.txt");

    println!("{:?}", path.as_os_str());
    let result = File::open(path);
    match result {
        Ok(handle) => handle,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create(path) {
                Ok(handle) => handle,
                Err(create_error) => panic!("can not create the file, reason is {:?}", create_error),
            },
            _ => panic!("other error happened!, {:?}", err),
        },
    };
}

pub fn read_explore() -> Result<String, io::Error> {
    let mut read_string = String::new();
    //this is the shortcut to get the result add ? operator,
    //if the result is Error, the program will return the error result
    //there is ok to use ? operator, because the io::Error provide From trait
    let mut file = File::open("test1.txt")?;
    file.read_to_string(&mut read_string)?;
    Ok(read_string)
}