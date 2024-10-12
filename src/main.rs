fn main() {
    if ssc::c::file::IS_INT {
        println!("In this implementation, the files are integers.");
    } else {
        println!("In this implementation, the files are probably pointers.");
    }
}
