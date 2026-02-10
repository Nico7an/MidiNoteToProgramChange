fn main() {
    if let Err(e) = nih_plug_xtask::main() {
        eprintln!("Error: {:#}", e);
        std::process::exit(1);
    }
}
