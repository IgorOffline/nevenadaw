mod clap_loader;
mod windows_utils;

use crate::clap_loader::ClapPluginHost;
use std::path::Path;

fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let plugin_path =
        Path::new(r"C:\Program Files\Common Files\CLAP\Surge Synth Team\Surge XT.clap");

    let plugin_path_str = plugin_path
        .to_str()
        .ok_or_else(|| anyhow::anyhow!("Invalid plugin path"))?;

    log::info!("Loading plugin: {}", plugin_path_str);

    let mut host = ClapPluginHost::new();
    let hwnd = host.load_and_create_gui(plugin_path_str, None)?;

    log::info!("Plugin loaded, HWND: {:?}", hwnd);

    Ok(())
}
