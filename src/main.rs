use std::{fs, io::SeekFrom};
use std::fs::OpenOptions;
use std::io::Write;

// use fscanf::Scan1;
use scan2::Scan2;
fn main() {
    let f = fs::File::open("test.txt").expect("opening file");
    let mut scan = Scan2::new(f);

    let mut value = Default::default();
    let mut count = 0;
    while scan.next_i32(&mut value).unwrap() {
        count += 1;
    }
    println!(">> count : {count}");

    scan.seek(SeekFrom::Start(0)).unwrap();

    let mut arr = vec![0; count];
    #[allow(clippy::needless_range_loop)]
    for i in 0..count {
        if !scan.next_i32(&mut arr[i]).unwrap() {
            std::process::exit(1);
        }
    }
    println!(">> arr : {arr:?}");

    // Test cases
    let f_test = fs::File::open("test.txt").expect("opening test file");
    let mut scan_test = Scan2::new(f_test);

    let mut file = OpenOptions::new()
    .write(true)
    .create(true)
    .open("output.txt")
    .unwrap();
    // let mut variable: i32 = 0;

    scan_test.seek(SeekFrom::Start(0)).unwrap();

    while !scan_test.is_eof().unwrap() {
        let value = scan_test.next_u8().unwrap();
        let character = value as char;
        write!(file, "{}", character).unwrap();
    }
    
}
