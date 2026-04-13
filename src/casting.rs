pub fn casting(value: i64) {
    println!("as u16: {}", value as u16); // might max out their values when given a bigger number than the type allows;
    println!("as i16: {}", value as i16); // might max out their values when given a bigger number than the type allows;
    println!("as u8: {}", value as u8); // might max out their values when given a bigger number than the type allows;
}
