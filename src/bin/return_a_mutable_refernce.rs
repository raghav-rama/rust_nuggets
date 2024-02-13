/*
 * This code tells about ownership and borrowing in Rust
 * Here MutStr is a struct which holds a mutable reference to a string
 * So first of all when we use refernces in rust we must use lifetimes
 * Secondly, a function cannot return a mutable refence to a value if it owns that value. eg if you initialized a value inside of a function
 * and try to return that value, it is not possible because that value will be dropped after the function ends. refer fail function below
 *
*/

fn main() {
    struct MutStr<'a>(&'a mut str);
    impl<'a> MutStr<'a> {
        fn new(s: &'a mut str) -> Self {
            MutStr(s)
        }
        fn get_mut(&'a mut self) -> &'a mut str {
            self.0
        }
    }
    // let mut str = String::from("Hello, World!");
    // let mut str = String::from("Hello, World!");
    let mut str = "Hello, World!".to_owned();
    let mut mut_str = MutStr(&mut str);
    // let mut_str_ref = mut_str.get_mut();
    println!("{}", mut_str.get_mut());
    let mut str2 = "Hello, World2!".to_owned();
    let mut some_str = MutStr::new(&mut str2);
    println!("{}", some_str.get_mut());
}
/*
fn fail<'a>() -> &'a mut str {
    let mut s = String::from("Hello, World!");
    &mut s
}
*/
