Proposed changes to address the GUI lifecycle warning and keep the plugin window alive.

Files:
- proposal/src/clap_loader.rs: Track GUI extension lifecycle and call gui.destroy before plugin.destroy.
- proposal/src/win32_host_window.rs: Minimal Win32 host window + message loop.
- proposal/src/main.rs: Create host window, parent plugin GUI, and run message loop.
- proposal/src/windows_utils.rs: Copy of existing helper utilities.

Notes:
- The host window handle is passed to set_parent; the plugin's own window handle is not returned by CLAP.
- The message loop keeps the process alive so the GUI stays visible.
