use log::*;

fn main() {
    notepad_logger::init().unwrap();
    log::set_max_level(LevelFilter::Info);
    info!("Hello!");
    info!("This should appear in your open notepad window");
    warn!("It works with unicode too \u{1F642}\u{1F642}\u{1F642}");
}
