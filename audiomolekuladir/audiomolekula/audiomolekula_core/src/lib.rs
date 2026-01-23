pub fn core() {
    println!("<CORE::START> {}", audiomolekula_audio::add(2, 3));
    let coin_toss = rand::random_range(1..=2);
    let regina = audiomolekula_io::audiomolekula_load_toml()
        .expect("aafcf099 Failed to load audiomolekula config");
    println!("coin_toss= {}", coin_toss);
    println!("{}", regina);
    println!("<CORE::END>");
}
