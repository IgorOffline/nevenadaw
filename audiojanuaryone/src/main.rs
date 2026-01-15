use audiojanuaryone_lib::MyPlugin;
use nih_plug::nih_export_standalone;

fn main() {
    nih_export_standalone::<MyPlugin>();
}
