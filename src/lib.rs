use winapi::um::winuser::{FindWindowA, FindWindowExA, SendMessageA, EM_REPLACESEL};
use log::{Log, Record, Level, Metadata};
use std::ffi::CString;
use std::ptr;

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
        log(format!("{} - {}", record.level(), record.args()));
    }

    fn flush(&self) {
    }
}

fn log(mut string: String) {
    string += "\r\n";
    let c_string = CString::new(string).unwrap();
    unsafe {
        let mut notepad = FindWindowA(ptr::null(), cstr!("Untitled - Notepad"));
        if notepad.is_null() {
            notepad = FindWindowA(ptr::null(), cstr!("*Untitled - Notepad"));
        }
        if notepad.is_null() {
            return;
        }
        let edit = FindWindowExA(notepad, ptr::null_mut(), cstr!("EDIT"), ptr::null());
        assert!(!edit.is_null());
        SendMessageA(edit, EM_REPLACESEL as _, true as _, c_string.as_ptr() as _);
    }
}
