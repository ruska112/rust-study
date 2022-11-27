use std::io::{stdin, stdout, Write};

pub fn hello_world() {
    println!("Hello, World!");
}

pub fn three_real_num() {
    let mut line = String::new();

    stdin().read_line(&mut line).expect("msg");
    let mut x: f32 = line.trim().parse().unwrap();

    let mut line = String::new();

    stdin().read_line(&mut line).expect("msg");
    let mut y: f32 = line.trim().parse().unwrap();

    let mut line = String::new();

    stdin().read_line(&mut line).expect("msg");
    let  mut z: f32 = line.trim().parse().unwrap();

    let multi = x * y * z;
    let mid = (multi) as f32 / 3.0;

    println!("x = {:?}, y = {:?}, z = {:?}, multiplication = {:?}", x, y, z, multi);
    println!("x = {:?}, y = {:?}, z = {:?}, average = {:?}", x, y, z, mid);

    if x > y {
        let tmp = x;
        x = y;
        y = tmp;
    }
    if y > z {
        let tmp = y;
        y = z;
        z = tmp;
    }
    if x > y {
        let tmp = x;
        x = y;
        y = tmp;
    }
    println!("x = {:?}, y = {:?}, z = {:?}", x, y, z);
}

fn main() {
    hello_world();
}

