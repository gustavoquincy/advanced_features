extern "C" {
    fn abs(input: i32) -> i32;
}

#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just calld a Rust function from C!");
}

fn main() {
    unsafe {
        println!("Absolute value of -3 is {}", abs(-3));
    }
}
