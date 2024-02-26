// To run the code: 
// cargo run

use std::io::{ self, BufRead, Write };

fn main() {
    hello_world();
    io();
    data_types();
}

fn hello_world() {
    println!("Hello, world!");
}

fn io(){
    let stdin= io::stdin();
    print!("Enter your name : ");
    io::stdout().flush();
    let mut name: String = String::new();
    stdin.lock().read_line(&mut name);
    print!("Hello {} !", name.trim());
}

// Here are the basic data types in C++ (64bit) and Rust
// C++	            Rust
// int	            i32
// short	        i16
// long	            i64
// char	            i8
// unsigned int	    u32
// unsigned short	u16
// unsigned long	u64
// unsigned char	u8
// size_t	        usize
// float	        f32
// double	        f64
// bool	            bool
// char*	        &str   //Raw string (stack allocated) object. use to_str() to convert String to &str
// std::string	    String //Owned string (heap allocated) object. use to_owned() to convert &str to String
fn data_types(){
    let a: i32 = -2000;
    let b: i16 = -2993;
    let c: i64 = -102002030i64;
    let d: i8 = -100;
    let e: u32 = 200000;
    let f: u16 = 20000;
    let g: u8 = 200;
    let k: usize = 100;
    let l: f32 = 3.14;
    let m: f64 = 3.141592654;
    let n: bool = true;
    let s: &str = "Hello";
    let st: String = s.to_owned();
    print!("a : {} \n b : {} \n c: {} \n d: {} \n e: {} \n f: {} \n g: {} \n k: {} \n l: {} \n m: {} \n n: {} \n s: {} \n st: {}", a, b, c, d, e, f, g, k, l, m, n, s, st);
}