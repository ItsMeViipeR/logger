use macro_rules_attribute::apply;
use std::os::raw::c_char;
use std::ffi::CStr;
use std::fs::OpenOptions;
use std::io::Write;

mod macros;

// Define a struct to hold logging options
#[repr(C)]
pub struct LogOptions {
    pub recreate_file: bool,        // Whether to recreate the log file
    pub file_path: *const c_char,   // Path to the log file as a C-style string
}

// Apply the `dll_function` attribute to the following function
#[apply(dll_function)]
fn init_log(options: LogOptions) {
    // Convert the C-style file path to a Rust String
    let file_path_str: String = unsafe {
        CStr::from_ptr(options.file_path)
            .to_string_lossy()
            .into_owned()
    };

    // Open the log file with options based on whether recreation is needed
    let file = if options.recreate_file {
        OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(&file_path_str)
    } else {
        OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&file_path_str)
    };

    // If the file was opened successfully, write an initialization message
    if let Ok(mut f) = file {
        f.write_all(b"Logging initialized\n").ok();
    }
}

// Apply the `dll_function` attribute to the following function
#[apply(dll_function)]
fn log_message(options: LogOptions, message: *const c_char) {
    // Convert the C-style message to a Rust String
    let message_str = unsafe {
        CStr::from_ptr(message)
            .to_string_lossy()
            .into_owned()
    };

    // Convert the C-style file path to a Rust String
    let file_path_str: String = unsafe {
        CStr::from_ptr(options.file_path)
            .to_string_lossy()
            .into_owned()
    };

    // Get the current timestamp in the format HH:MM:SS
    let now = chrono::Local::now();

    // Format the log message with a timestamp
    let message_str = format!("[{}] {}\n", now.format("%H:%M:%S"), message_str);

    // Open the log file with options based on whether recreation is needed
    let mut file = if options.recreate_file {
        OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(&file_path_str)
    } else {
        OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&file_path_str)
    }.unwrap();

    // Write the formatted log message to the file
    if let Err(e) = writeln!(file, "{}", message_str) {
        eprintln!("Failed to write to log file: {}", e);
    }
}