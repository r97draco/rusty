// To run the code:
// cargo run

//Guide
// https://vnduongthanhtung.gitbooks.io/migrate-from-c-to-rust/content/

use std::io::{ self, BufRead, Write };

fn main() {
    // hello_world();
    // io();
    // data_types();
    // auto_infer_datatype();
    // mut_data_types();'
    // control_flow();
    // functions();
    // collec_arrays();
    collec_vectors();
}

// 1
// Hello World
fn hello_world() {
    println!("Hello, world!");
}

// 2
// Console Input/Output
fn io() {
    let stdin = io::stdin();
    print!("Enter your name : ");
    io::stdout().flush();
    let mut name: String = String::new();
    stdin.lock().read_line(&mut name);
    print!("Hello {} !", name.trim());
}

// 3
//Variables and Data Types
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
fn data_types() {
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
    print!(
        "a : {} \n b : {} \n c: {} \n d: {} \n e: {} \n f: {} \n g: {} \n k: {} \n l: {} \n m: {} \n n: {} \n s: {} \n st: {}",
        a,
        b,
        c,
        d,
        e,
        f,
        g,
        k,
        l,
        m,
        n,
        s,
        st
    );
}

// Rust can infer data types for variables based on the value assigned to them at initialization, so there is no need for explicit declaration of data types (this is similar to the use of auto in modern C++)
fn auto_infer_datatype() {
    let a = -100000i32;
    let b = -10000i16;
    let s = "Hello";
    let st = s.to_owned();
}

// Variables in Rust are immutable by default. If a variable needs to change value during runtime, it needs to be declared as mutable with keyword mut
fn mut_data_types() {
    let a = -100000i32;
    let mut b = -10000i16;
    // a = 100000;         // error, cannot change value of immutable variable
    b += 100; // ok, since b is mutable
}

// 4
// Control Flow
fn control_flow() {
    let x = 1;
    if x > 0 {
        println!("Positive number");
    } else if x < 0 {
        println!("Negative number");
    } else {
        println!("Zero");
    }

    for i in 0..10 {
        println!("{}*{}={}", i, i, i * i);
    }

    let mut t = 10;
    while t < 100 {
        t += t / 2;
    }
    println!("{}", t);

    // do while in c++
    t = 0;
    loop {
        println!("{}", t);
        t += 2;
        if !(t <= 10) {
            break;
        }
    }

    let age = 15;
    match age {
        0..=12 => println!("Child"),
        n @ 13..=19 => println!("Teenager at the age of {}", n),
        n => println!("Adult at the age of {}", n),
    }
}

// 5
// Functions
// The return keyword can also be removed if the semicolon at the end of the return statement is not present, which is similar to some languages such as Ruby or Erlang
fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn print(x: i32) {
    println!("{}", x);
}

fn functions() {
    let x = -10;
    let y = 20;
    let z = add(x, y);
    print(z);
}

// 6
// Collections

// Arrays
// There are two ways to declare an array in rust :
// By specifying the initial value and the number of element of the array : [init_value; N] : this creates an array with N elements, each has the value of init_value
// By listing all elements of the array [element1,element2,element3...], the elements are separated by the commas.
struct Person {
    name: &'static str,
    age: i32,
}

fn collec_arrays() {
    let mut arr = [0; 10];
    for i in 0..10 {
        arr[i] = i;
    }

    for x in &arr {
        // same as : for x in arr.iter()
        print!("{}", x);
    }
    println!("");

    let persons = [
        Person { name: "Alice", age: 20 },
        Person { name: "Bob", age: 30 },
        Person { name: "Charlie", age: 40 },
        Person { name: "David", age: 50 },
    ];

    for perosn in &persons {
        println!("{} is {} years old", perosn.name, perosn.age);
    }
}

//Vectors
// Vec::new()
// vec![init_value;N]
// vec![element1,element2,....]
fn collec_vectors() {
    let mut a: Vec<i32> = Vec::new();
    let mut b: Vec<i32> = Vec::new();

    a.push(1);
    a.push(2);
    a.push(3);

    for x in &a {
        // same as : for x in a.iter()
        b.push(2 * x);
    }

    for (i, x) in a.iter().enumerate() {
        println!("a[{}] = {}", i, x);
    }

    //The zip function joins two arrays into a new array with each element being a pair of elements from the two arrays. By using this technique, the code inside the loop does not need to check for index boundary but still remains safe, so the performance remain the same as C++ programs (C++ programs do not check index boundary by default and are considered unsafe)
    for (x, y) in a.iter().zip(b.iter()) {
        println!("2*{} {}", x, y);
    }
}

//Map

//Set
