unsafe extern "C" {
    fn cinterop(flag: i32) -> i32;
}

fn main() {
    println!("<START>");
    unsafe {
        let result = cinterop(42);
        println!("Rust::C::{}", result);
    }
    println!("<END>");
}
