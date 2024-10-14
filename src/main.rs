
fn main() {
    use std::ffi::CString;
    use ssc::c;
    use ssc::mmap;
    use mmap::file;

    if file::IS_INT {
        println!("In this implementation, the files are integers.");
    } else {
        println!("In this implementation, the files are probably pointers.");
    }

    let mut c_file: file::Type = file::NULL;
    println!("Prior to calling SSC_FilePath_create(), c_file was {c_file}");
    let path = CString::new("/ram/u/test.txt").unwrap();
    let err = unsafe {
        mmap::SSC_FilePath_create(path.as_ptr() as *const cty::c_char, &mut c_file as *mut file::Type)
    };
    println!("The c_file became {c_file}");
    println!("The path became {:?}", path);
    println!("The err beccame {err}");

    {
        let nproc = unsafe { c::SSC_getNumberProcessors() };
        println!("We have {nproc} processors.");
    }
}
