pub fn get_priority(c: char) -> i32 {
    let byte_val = c as i32;

    if byte_val >= 97 {
        // a - z: 1 - 26
        byte_val - 96
    } else {
        // A - Z: 27 - 52
        byte_val - 38
    }
}
