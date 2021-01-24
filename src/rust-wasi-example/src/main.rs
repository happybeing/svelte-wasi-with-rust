use wasm_bindgen::prelude::*;

use std::fs;
use std::io::Read;
use structopt::StructOpt;

// Note:    For WebAssembly to call Rust exports (above), main() must be empty.
//          You can do one or the other, but not both.
fn main() {}

// #[wasm_bindgen]
// pub fn runtest() -> Result<(), JsValue> {
//     // set_panic_hook();

//     println!("runtest called!");
//     // // ...
//     // let p: web_sys::Node = document.create_element("p")?.into();
//     // p.set_text_content(Some("Hello from Rust, WebAssembly, and Webpack!"));
//     // // ...
//     Ok(())
// }

// -------------------------------------------------------------
// Misc tests

#[wasm_bindgen(module = "test")]
extern "C" {
    fn js_test();
    fn js_test_n(n: u32);
 }


#[wasm_bindgen]
pub fn rust_js_test() {
    println!("Rust calling js_test()...");
    js_test();

    println!("Rust calling js_test_n(123)...");
    js_test_n(123);
}

#[wasm_bindgen]
pub fn rust_print_bg() {
    println!("Hello, world BG!");
}

#[wasm_bindgen]
pub fn rust_print_bg_n(n: u32) {
    println!("From rust_print_bg_n(n): {}", n);
}

#[wasm_bindgen]
pub fn rust_say(s: String) -> String {
  let r = String::from("hello ");
  return r + &s;
}

// -------------------------------------------------------------
// wasm-bindgen example by github.com/ibaryshnikov
// (https://github.com/ibaryshnikov/rust-workshop-21-dec-2018)

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen(js_name = doubleList)]
pub fn double_list(list: &[i32]) -> Vec<i32> {
    list.iter().map(|x| x * 2).collect()
}

// #[wasm_bindgen(js_name = addElement)]
// pub fn add_element(name: String) -> Result<(), JsValue> {
//     let window = web_sys::window().expect("should have a window");
//     let document = window.document().expect("should have a document");
//     let body = document.body().expect("should have a body");

//     let p = document.create_element("p")?;
//     p.set_inner_html(&format!("New element with name: {}", name));

//     body.append_child(&p)?;

//     Ok(())
// }

// -------------------------------------------------------------
// Examples from Getting started with Rust functions in Node.js
// Article: https://www.secondstate.io/articles/getting-started-with-rust-function/
// Source:  https://github.com/second-state/wasm-learning/tree/master/nodejs/functions
// More:    https://cloud.secondstate.io/server-side-webassembly/getting-started

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Point {
  x: f32, 
  y: f32
}

#[derive(Serialize, Deserialize, Debug)]
struct Line {
  points: Vec<Point>,
  valid: bool,
  length: f32,
  desc: String
}

#[wasm_bindgen]
pub fn create_line (p1: &str, p2: &str, desc: &str) -> String {
  let point1: Point = serde_json::from_str(p1).unwrap();
  let point2: Point = serde_json::from_str(p2).unwrap();
  let length = ((point1.x - point2.x) * (point1.x - point2.x) + (point1.y - point2.y) * (point1.y - point2.y)).sqrt();

  let valid = if length == 0.0 { false } else { true };
  let line = Line { points: vec![point1, point2], valid: valid, length: length, desc: desc.to_string() };
  return serde_json::to_string(&line).unwrap();
}

#[wasm_bindgen]
pub fn say(s: &str) -> String {
  let r = String::from("hello ");
  return r + s;
}

#[wasm_bindgen]
pub fn obfusticate(s: String) -> String {
  (&s).chars().map(|c| {
    match c {
      'A' ..= 'M' | 'a' ..= 'm' => ((c as u8) + 13) as char,
      'N' ..= 'Z' | 'n' ..= 'z' => ((c as u8) - 13) as char,
      _ => c
    }
  }).collect()
}

use num_integer::lcm;
use sha3::{Digest, Sha3_256, Keccak256};

#[wasm_bindgen]
pub fn lowest_common_denominator(a: i32, b: i32) -> i32 {
  let r = lcm(a, b);
  return r;
}

#[wasm_bindgen]
pub fn sha3_digest(v: Vec<u8>) -> Vec<u8> {
  return Sha3_256::digest(&v).as_slice().to_vec();
}

#[wasm_bindgen]
pub fn keccak_digest(s: &[u8]) -> Vec<u8> {
  return Keccak256::digest(s).as_slice().to_vec();
}

// -------------------------------------------------------------
// wasm-bindgen book example 
// (https://rustwasm.github.io/docs/wasm-bindgen/reference/types/imported-js-types.html)

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub enum NumberEnum {
    Foo = 0,
    Bar = 1,
    Qux = 2,
}

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub enum StringEnum {
    Foo = "foo",
    Bar = "bar",
    Qux = "qux",
}

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub struct Struct {
    pub number: NumberEnum,
    pub string: StringEnum,
}

#[wasm_bindgen(module = "test")]
extern "C" {
    pub type SomeJsType;
}

#[wasm_bindgen]
pub fn imported_type_by_value(x: SomeJsType) {
    /* ... */
}

#[wasm_bindgen]
pub fn imported_type_by_shared_ref(x: &SomeJsType) {
    /* ... */
}

#[wasm_bindgen]
pub fn return_imported_type() -> SomeJsType {
    // let jsType = SomeJsType::new();
    // return jsType;
    unimplemented!()
}

#[wasm_bindgen]
pub fn take_option_imported_type(x: Option<SomeJsType>) {
    /* ... */
}

#[wasm_bindgen]
pub fn return_option_imported_type() -> Option<SomeJsType> {
    unimplemented!()
}

// -------------------------------------------------------------
// Tests based on the WasmerJS H9Q+ Rust example
//

fn print_hello_world() {
    println!("Hello, world!");
}

fn print_nintey_nine_bottles_of_beer_on_the_wall() {
    for i in (1..100).rev() {
        println!("{} bottles of beer on the wall,{} bottles of beer.Take one down, pass it around,{} bottles of beer on the wall.", i, i, i - 1);
    }
}

fn h9q_run(source_code: String) {
    #[allow(unused_variables)]
    let mut accumulator = 0;

    for c in source_code.chars() {
        match c {
            'H' => print_hello_world(),
            '9' => print_nintey_nine_bottles_of_beer_on_the_wall(),
            'Q' => println!("{}", source_code),
            '+' => accumulator += 1,
            _ => (),
        }
    }
}

#[derive(Debug, StructOpt)]
struct Opt {
    /// Evaluate HQ9+ source code
    #[structopt(short = "e", long = "eval")]
    source_code: Option<String>,
    /// Evaluate HQ9+ source code from a file
    #[structopt(short = "f", long = "file")]
    source_file: Option<String>,
    /// Force evaluation of the `H` instruction
    #[structopt(short = "h", long = "hello-world")]
    hello_world: bool,
    /// Force evaluation of the `9` instruction
    #[structopt(short = "9", long = "nintey-nine-bottles")]
    ninety_nine_bottles: bool,
}

#[wasm_bindgen]
pub fn h9q_string(source_code: String) {
    h9q_run(source_code);
}

#[wasm_bindgen]
pub fn h9q_file(src_file: String) {
    // let opt = Opt::from_string(&params);

    // if opt.hello_world {
    //     print_hello_world();
    //     return;
    // }

    // if opt.ninety_nine_bottles {
    //     print_nintey_nine_bottles_of_beer_on_the_wall();
    //     return;
    // }

    // if let Some(src) = opt.source_code {
    //     h9q_run(src);
    //     return;
    // }

    // if let Some(src_file) = opt.source_file {
        match fs::File::open(&src_file) {
            Ok(mut src_file_handle) => {
                let mut buf = String::new();
                src_file_handle
                    .read_to_string(&mut buf)
                    .expect(&format!("Failed to read data from file: {}", &src_file));
                h9q_run(buf);
            }, 
            Err(e) => {
                println!("Error: {}", e);
                eprintln!(
                    "Error: please pass HQ9+ source code via the `-e` flag or as a file via the `-f` flag"
                );
        //         ::std::process::exit(-1);
            }
        }
}

