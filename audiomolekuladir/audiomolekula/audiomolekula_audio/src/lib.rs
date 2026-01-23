use clap_sys::entry::clap_plugin_entry;
use clap_sys::factory::plugin_factory::{clap_plugin_factory, CLAP_PLUGIN_FACTORY_ID};
use libloading::{Library, Symbol};
use std::ffi::CString;

pub fn setup_audio_system() {
    let plugin_path = r"C:\Program Files\Common Files\CLAP\Vital.clap";
    let plugin_name = "Vital";

    if !std::path::Path::new(plugin_path).exists() {
        println!("{} not found at {}", plugin_name, plugin_path);
        return;
    }

    let lib = unsafe {
        Library::new(plugin_path).unwrap_or_else(|_| panic!("Failed to load {}", plugin_name))
    };

    let entry_symbol: Symbol<*const clap_plugin_entry> =
        unsafe { lib.get(b"clap_entry\0").expect("Failed to get clap_entry") };
    let entry = unsafe { &**entry_symbol };

    let plugin_path_cstring = CString::new(plugin_path).unwrap();
    unsafe { (entry.init.expect("Plugin init missing"))(plugin_path_cstring.as_ptr()) };

    let factory_ptr = unsafe {
        entry.get_factory.expect("get_factory missing")(CLAP_PLUGIN_FACTORY_ID.as_ptr() as *const i8)
    };
    let factory = unsafe { (factory_ptr as *const clap_plugin_factory).as_ref() };
    if factory.is_none() {
        println!("Plugin factory is null for {}", plugin_name);
        return;
    }
    //let _factory = factory.unwrap();

    println!("(Factory ready)");
}
