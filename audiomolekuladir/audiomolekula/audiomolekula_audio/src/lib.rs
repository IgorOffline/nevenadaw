use clap_sys::entry::clap_plugin_entry;
use clap_sys::events::clap_event_note;
use clap_sys::factory::plugin_factory::{clap_plugin_factory, CLAP_PLUGIN_FACTORY_ID};
use clap_sys::host::clap_host;
use clap_sys::version::CLAP_VERSION;
use libloading::{Library, Symbol};
use std::ffi::CString;

#[allow(dead_code)]
struct MockNoteEvent {
    note: clap_event_note,
}

unsafe extern "C" fn host_get_extension(
    _host: *const clap_host,
    _extension_id: *const i8,
) -> *const std::ffi::c_void {
    std::ptr::null()
}

unsafe extern "C" fn host_request_restart(_host: *const clap_host) {}
unsafe extern "C" fn host_request_process(_host: *const clap_host) {}
unsafe extern "C" fn host_request_callback(_host: *const clap_host) {}

static AUDIOMOLEKULA_HOST: clap_host = clap_host {
    clap_version: CLAP_VERSION,
    host_data: std::ptr::null_mut(),
    name: b"Audiomolekula\0".as_ptr() as *const i8,
    vendor: b"Independent\0".as_ptr() as *const i8,
    url: b"https://igordurbek.com\0".as_ptr() as *const i8,
    version: b"0.1.0\0".as_ptr() as *const i8,
    get_extension: Some(host_get_extension),
    request_restart: Some(host_request_restart),
    request_process: Some(host_request_process),
    request_callback: Some(host_request_callback),
};

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
    let factory_raw = unsafe { (factory_ptr as *const clap_plugin_factory).as_ref() };
    if factory_raw.is_none() {
        println!("Plugin factory is null for {}", plugin_name);
        return;
    }
    let factory = factory_raw.unwrap();

    let plugin_ptr = unsafe {
        let count = (factory.get_plugin_count.expect("get_plugin_count missing"))(factory);
        if count == 0 {
            println!("No plugins found in {}", plugin_name);
            return;
        }
        let descriptor = (factory
            .get_plugin_descriptor
            .expect("get_plugin_descriptor missing"))(factory, 0);

        (factory.create_plugin.expect("create_plugin missing"))(
            factory,
            &AUDIOMOLEKULA_HOST,
            (*descriptor).id,
        )
    };

    if plugin_ptr.is_null() {
        println!("{} failed to initialize", plugin_name);
        return;
    }

    let plugin = unsafe { &*plugin_ptr };
    unsafe {
        if let Some(init) = plugin.init {
            if !(init)(plugin) {
                println!("Error: Plugin init failed");
                return;
            }
        }
        if let Some(activate) = plugin.activate {
            if !(activate)(plugin, 44100.0, 1, 4096) {
                println!("Error: Plugin activation failed");
                return;
            }
        }
    }

    println!("(Audio system ready)");
}
