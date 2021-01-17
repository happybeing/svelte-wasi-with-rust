use wasm_bindgen::prelude::*;

use std::fs;
use std::io::Read;
use structopt::StructOpt;

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

// #[wasm_bindgen]
// pub fn runtest() -> Result<(), JsValue> {
//     // set_panic_hook();

//     println!("runtest called!\n");
//     // // ...
//     // let p: web_sys::Node = document.create_element("p")?.into();
//     // p.set_text_content(Some("Hello from Rust, WebAssembly, and Webpack!"));
//     // // ...
//     Ok(())
// }

#[wasm_bindgen(module = "test")]
extern "C" {
    fn js_test();
    fn js_test_n(n: u32);
 }

#[wasm_bindgen]
pub fn rust_js_test() {
    println!("Rust calling js_test()...\n");
    js_test();

    println!("Rust calling js_test_n(123)...\n");
    js_test_n(123);
}

#[wasm_bindgen]
pub fn rust_print_bg() {
    println!("Hello, world BG!\n");
}

// #[no_mangle]
// pub fn rust_print_nm() {
//     println!("Hello, world NM!\n");
// }

pub fn print_hello_world() {
    println!("Hello, world!\n");
}

fn print_nintey_nine_bottles_of_beer_on_the_wall() {
    for i in (1..100).rev() {
        println!("{} bottles of beer on the wall,\n{} bottles of beer.\nTake one down, pass it around,\n{} bottles of beer on the wall.", i, i, i - 1);
    }
}

fn run(source_code: String) {
    #[allow(unused_variables)]
    let mut accumulator = 0;

    for c in source_code.chars() {
        match c {
            'H' => print_hello_world(),
            '9' => print_nintey_nine_bottles_of_beer_on_the_wall(),
            'Q' => println!("{}\n", source_code),
            '+' => accumulator += 1,
            _ => (),
        }
    }
}

// Note:    For WebAssembly to call Rust exports (above), main() must be empty.
//          You can do one or the other, but not both.
fn main() {
    // let opt = Opt::from_args();

    // if opt.hello_world {
    //     print_hello_world();
    //     return;
    // }

    // if opt.ninety_nine_bottles {
    //     print_nintey_nine_bottles_of_beer_on_the_wall();
    //     return;
    // }

    // if let Some(src) = opt.source_code {
    //     run(src);
    //     return;
    // }
    // if let Some(src_file) = opt.source_file {
    //     match fs::File::open(&src_file) {
    //         Ok(mut src_file_handle) => {
    //             let mut buf = String::new();
    //             src_file_handle
    //                 .read_to_string(&mut buf)
    //                 .expect(&format!("Failed to read data from file: {}", &src_file));
    //             run(buf);
    //         }, 
    //         Err(e) => {
    //             println!("Error: {}", e);
    //             eprintln!(
    //                 "Error: please pass HQ9+ source code via the `-e` flag or as a file via the `-f` flag"
    //             );
    //     //         ::std::process::exit(-1);
    //         }
    //     }
    // }
}

