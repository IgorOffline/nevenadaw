pub fn core() {
    println!("<CORE::START>");
    let coin_toss = rand::random_range(1..=2);
    println!("coin_toss={}", coin_toss);
    audiomolekula_audio::setup_audio_system();
    let regina = audiomolekula_io::audiomolekula_load_toml()
        .expect("aafcf099 Failed to load audiomolekula config");
    println!("{}", regina);
    println!("<CORE::END>");
}
