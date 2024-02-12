fn main() {
    // u8,u16,u32,u64,u128, and isize
    // i8,i16,i32,i64,i128, and isize
    println!("Hello from primitive types");

    let letter = 'A';
    let space = ' ';
    let other_language_char = 'ক';
    let cat_face = '😺';

    // println!("{}",letter);
    // println!("{}",space);
    // println!("{}",other_language_char);
    // println!("{}",cat_face);

    let my_number: u8 = 100; // we didn't type of integer
                             //so rust choose i32 always.
                             //Rust always choose i32 if you don't tell rust it's type
    println!("{}", my_number as char); // prints out the 100 ascii value

    // len -> shows the datatype bytes
    println!("{}", "Abced".len()); // 5
    println!("{}", "a".len()); // 1
    println!("{}", "b".len()); // 1
    println!("{}", "ক".len()); // 3
    println!("{}", "🔥".len()); // 4
}
