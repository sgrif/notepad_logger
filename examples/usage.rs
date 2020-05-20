use log::*;

fn main() {
    notepad_logger::init().unwrap();
    info!("Hello!");
    info!("This should appear in your open notepad window");
}
