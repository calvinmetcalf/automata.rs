use automata::compute;
use std::os::args;
mod automata;
fn main() {
    let args : ~[~str] = args();
    let code = from_str::<u8>(args[1]).unwrap_or(30u8);
    let value = from_str::<u8>(args[2]).unwrap_or(0u8);
    println!("next value is: {}",compute(code,value));
}