use clap_sys::entry::clap_plugin_entry;
use clap_sys::factory::plugin_factory::{clap_plugin_factory, CLAP_PLUGIN_FACTORY_ID};
use clap_sys::host::clap_host;
use clap_sys::plugin::clap_plugin;
use clap_sys::version::CLAP_VERSION;
use libloading::{Library, Symbol};
use std::ffi::{c_void, CString};

unsafe extern "C" fn host_get_extension(
    _host: *const clap_host,
    _extension_id: *const i8,
) -> *const c_void {
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

struct PluginCleanup<'a> {
    entry: &'a clap_plugin_entry,
    entry_inited: bool,
    plugin: *const clap_plugin,
    activated: bool,
}

impl<'a> PluginCleanup<'a> {
    fn new(entry: &'a clap_plugin_entry) -> Self {
        Self {
            entry,
            entry_inited: false,
            plugin: std::ptr::null(),
            activated: false,
        }
    }
}

impl Drop for PluginCleanup<'_> {
    fn drop(&mut self) {
        unsafe {
            if !self.plugin.is_null() {
                let plugin = &*self.plugin;
                if self.activated {
                    if let Some(deactivate) = plugin.deactivate {
                        deactivate(plugin);
                    }
                }
                if let Some(destroy) = plugin.destroy {
                    destroy(plugin);
                }
            }

            if self.entry_inited {
                if let Some(deinit) = self.entry.deinit {
                    deinit();
                }
            }
        }
    }
}

unsafe fn load_entry(lib: &Library) -> Result<&clap_plugin_entry, String> {
    let entry_symbol: Symbol<*const clap_plugin_entry> = lib
        .get(b"clap_entry\0")
        .map_err(|err| format!("Failed to get clap_entry: {}", err))?;
    let entry_ptr = *entry_symbol;
    if entry_ptr.is_null() {
        return Err("clap_entry symbol is null".to_string());
    }

    Ok(&*entry_ptr)
}

unsafe fn get_factory(entry: &clap_plugin_entry) -> Result<&clap_plugin_factory, String> {
    let get_factory = entry
        .get_factory
        .ok_or_else(|| "get_factory missing".to_string())?;
    let factory_ptr = get_factory(CLAP_PLUGIN_FACTORY_ID.as_ptr() as *const i8);
    if factory_ptr.is_null() {
        return Err("Plugin factory is null".to_string());
    }

    Ok(&*(factory_ptr as *const clap_plugin_factory))
}

pub fn setup_audio_system() {
    let plugin_path = r"C:\Program Files\Common Files\CLAP\Vital.clap";
    let plugin_name = "Vital";

    if !std::path::Path::new(plugin_path).exists() {
        println!("{} not found at {}", plugin_name, plugin_path);
        return;
    }

    let lib = match unsafe { Library::new(plugin_path) } {
        Ok(lib) => lib,
        Err(err) => {
            println!("Failed to load {}: {}", plugin_name, err);
            return;
        }
    };

    let entry = match unsafe { load_entry(&lib) } {
        Ok(entry) => entry,
        Err(msg) => {
            println!("{}", msg);
            return;
        }
    };

    let plugin_path_cstring = match CString::new(plugin_path) {
        Ok(value) => value,
        Err(err) => {
            println!("Invalid plugin path: {}", err);
            return;
        }
    };

    let entry_init = match entry.init {
        Some(init) => init,
        None => {
            println!("Plugin init missing for {}", plugin_name);
            return;
        }
    };

    if unsafe { !(entry_init)(plugin_path_cstring.as_ptr()) } {
        println!("Plugin entry init failed for {}", plugin_name);
        return;
    }
    let mut cleanup = PluginCleanup::new(entry);
    cleanup.entry_inited = true;

    let factory = match unsafe { get_factory(entry) } {
        Ok(factory) => factory,
        Err(msg) => {
            println!("{} for {}", msg, plugin_name);
            return;
        }
    };

    let get_plugin_count = match factory.get_plugin_count {
        Some(get_plugin_count) => get_plugin_count,
        None => {
            println!("get_plugin_count missing for {}", plugin_name);
            return;
        }
    };

    let count = unsafe { get_plugin_count(factory) };
    if count == 0 {
        println!("No plugins found in {}", plugin_name);
        return;
    }

    let get_plugin_descriptor = match factory.get_plugin_descriptor {
        Some(get_plugin_descriptor) => get_plugin_descriptor,
        None => {
            println!("get_plugin_descriptor missing for {}", plugin_name);
            return;
        }
    };
    let descriptor = unsafe { get_plugin_descriptor(factory, 0) };
    if descriptor.is_null() {
        println!("Plugin descriptor is null for {}", plugin_name);
        return;
    }

    let plugin_id = unsafe { (*descriptor).id };
    if plugin_id.is_null() {
        println!("Plugin descriptor id is null for {}", plugin_name);
        return;
    }

    let create_plugin = match factory.create_plugin {
        Some(create_plugin) => create_plugin,
        None => {
            println!("create_plugin missing for {}", plugin_name);
            return;
        }
    };
    let plugin_ptr = unsafe { create_plugin(factory, &AUDIOMOLEKULA_HOST, plugin_id) };

    if plugin_ptr.is_null() {
        println!("{} failed to initialize", plugin_name);
        return;
    }

    let plugin = unsafe { &*plugin_ptr };
    cleanup.plugin = plugin_ptr;
    let plugin_init = match plugin.init {
        Some(init) => init,
        None => {
            println!("Plugin init missing for {}", plugin_name);
            return;
        }
    };
    if unsafe { !(plugin_init)(plugin) } {
        println!("Error: Plugin init failed");
        return;
    }

    let mut activated = false;
    if let Some(activate) = plugin.activate {
        if unsafe { !(activate)(plugin, 44100.0, 1, 4096) } {
            println!("Error: Plugin activation failed");
            return;
        }
        activated = true;
    }
    cleanup.activated = activated;

    println!("(Audio system setup completed)");
}
