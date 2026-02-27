fn main() {
    let hello = plugins::hello();
    let info = plugderive::log_info();
    println!("{} {}", hello, info);
}
