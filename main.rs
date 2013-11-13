use automata::compute;
use std::os::args;
mod automata;
fn main() {
    let args : ~[~str] = args();
    let code = from_str::<u8>(args[1]).unwrap_or(30u8);
    let vString = args[2];
    let mut value = 0u8;
    println!("next value is: {}",compute(code,value));
}