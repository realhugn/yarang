include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

fn main() {
    unsafe {
        let result = add(5, 3);
        println!("The result is: {}", result);
    }
}