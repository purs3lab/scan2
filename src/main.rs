use std::{fs, io::SeekFrom};

// use fscanf::Scan1;
use scan2::Scan2;

fn main() {
    // println!("Hello, world!");

    let f = fs::File::open("test.txt").expect("opening file");
    // let mut scan = Scan1::new(f);
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
}
