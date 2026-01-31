mod clap_loader;
mod win32_host_window;
mod windows_utils;

use crate::clap_loader::ClapPluginHost;
use crate::win32_host_window::{create_host_window, run_message_loop};
use std::path::Path;

fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let plugin_path =
        Path::new(r"C:\Program Files\Common Files\CLAP\Surge Synth Team\Surge XT.clap");

    let plugin_path_str = plugin_path
        .to_str()
        .ok_or_else(|| anyhow::anyhow!("Invalid plugin path"))?;

    log::info!("Loading plugin: {}", plugin_path_str);

    let host_hwnd = create_host_window()?;

    let mut host = ClapPluginHost::new();
    let hwnd = host.load_and_create_gui(plugin_path_str, Some(host_hwnd))?;

    log::info!("Plugin loaded, HWND: {:?}", hwnd);

    run_message_loop();

    Ok(())
}
