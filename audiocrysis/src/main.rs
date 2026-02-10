use audiocrysis::Audiocrysis;
use nih_plug::nih_export_standalone;

fn main() {
    println!("<START>");
    nih_export_standalone::<Audiocrysis>();
    println!("<END>");
}
