use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();
    let target = target.as_str();

    if target.contains("wasm") {
        println!("Hey there wasm world!");
    }
}
