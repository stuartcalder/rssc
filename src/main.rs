#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))] // Warn about this stuff on Release mode only.

fn main() {
    use ssc::proc;

    let result = proc::get_executable_path();
    match result {
        Ok(exe_path) => println!("The executable path was {:?}.", exe_path),
        Err(_)       => println!("Failed to obtain the executable path."),
    }
}

#[cfg(feature = "Disable")]
fn main() -> Result<(), std::process::ExitCode> {
    use std::ffi::CString;
    use ssc::c;
    use ssc::mmap;
    use mmap::file;
    use mmap::init_flag;
    use mmap::init_code;

    if file::IS_INT {
        println!("In this implementation, the files are integers.");
    } else {
        println!("In this implementation, the files are probably pointers.");
    }
    
    let fpath = CString::new("/ram/u/mmap_test").unwrap();
    let size  = 1024usize * 1024usize;
    let map_result = mmap::Map::new(
        &fpath, 
        size,
        init_flag::ALLOW_SHRINK |
        init_flag::FORCE_EXIST
    );
    use std::process::ExitCode;
    match map_result {
        Ok(mut m) => {
            m.get_slice().fill(0xffu8);
            println!("Filled the memory map with 0xff!");
        },
        Err(e) => {
            let msg:&'static str = match e {
                init_code::ERR_FILE_EXIST_NO   => "Failed to force a file to not exist!",
                init_code::ERR_FILE_EXIST_YES  => "Failed to force a file to exist!",
                init_code::ERR_READONLY        => "Violated readonly!",
                init_code::ERR_SHRINK          => "Attempted to shrink while disallowed!",
                init_code::ERR_NO_SIZE         => "Size not provided!",
                init_code::ERR_OPEN_FILEPATH   => "Failed to open the filepath!",
                init_code::ERR_CREATE_FILEPATH => "Failed to create the filepath!",
                init_code::ERR_GET_FILE_SIZE   => "Failed to get the file size!",
                init_code::ERR_SET_FILE_SIZE   => "Failed to set the file size!",
                init_code::ERR_MAP             => "Failed to map the file!",
                _                              => "Unaccounted for error!"
            };
            eprintln!("{msg}");
            return Err(ExitCode::FAILURE)
        },
    }
    Ok(())
}
