
use std::io::stdin;

pub fn read_console()->String {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer);
    return buffer
}
