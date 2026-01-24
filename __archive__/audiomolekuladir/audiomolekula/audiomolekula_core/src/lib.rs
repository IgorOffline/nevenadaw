pub fn core() {
    println!("<CORE::START>");
    let regina = audiomolekula_io::audiomolekula_load_toml()
        .expect("aafcf099 Failed to load audiomolekula config");
    println!("{}", regina);
    audiomolekula_frontend::frontend_show_window();
    println!("<CORE::END>");
}
