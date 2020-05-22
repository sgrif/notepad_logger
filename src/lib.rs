use winapi::um::winuser::{FindWindowA, FindWindowExA, SendMessageW, EM_REPLACESEL};
use log::{Log, Record, Metadata};
use std::ptr;
use std::iter::once;

macro_rules! cstr {
    ($s:expr) => { concat!($s, "\0").as_ptr() as _ }
}

pub fn init() -> Result<(), log::SetLoggerError> {
    static LOGGER: NotepadLogger = NotepadLogger;
    log::set_logger(&LOGGER)
}

struct NotepadLogger;

impl Log for NotepadLogger {
    fn enabled(&self, _: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        log(format!("{} - {}\r\n", record.level(), record.args()));
    }

    fn flush(&self) {
    }
}

fn log(string: String) {
    let c_string: Vec<u16> = string
        .encode_utf16()
        .chain(once(0))
        .collect();
    unsafe {
        let mut notepad = FindWindowA(ptr::null(), cstr!("Untitled - Notepad"));
        if notepad.is_null() {
            notepad = FindWindowA(ptr::null(), cstr!("*Untitled - Notepad"));
        }
        if notepad.is_null() {
            return;
        }
        let edit = FindWindowExA(notepad, ptr::null_mut(), cstr!("EDIT"), ptr::null());
        if edit.is_null() {
            return;
        }
        SendMessageW(edit, EM_REPLACESEL as _, true as _, c_string.as_ptr() as _);
    }
}
