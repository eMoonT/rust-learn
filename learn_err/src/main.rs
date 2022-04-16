use std::{fs::File, io::Read};

fn main() {
    // let f = File::open("hello.txt");
    // let r = match f {
    //     Ok(file) => file,
    //     Err(error) => panic!("error: {:?}", error),
    // };
    let f = File::open("hello.txt").unwrap();
    // let f = File::open("hello.txt").expect("Fail to open hello.txt");
    // panic!("crash here");
}
