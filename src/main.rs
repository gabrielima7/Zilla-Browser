mod adblock;
mod gui;
mod networking;
mod rendering;

fn main() {
    println!("Iniciando o Zilla Browser...");
    gui::create_and_run_gui();
}
