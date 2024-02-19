use std::mem;
fn main() {
    // deep dive into array
    let t = (1, 'a', false);
    let f = (2, t); // (2,(1,'a',false));
    println!("{} {} {}", t.0, t.1, t.2);
    // println!("{:#?}", f); // pretty printing

    // let z = (1,2,3,4,5,6,7,8,9,10,11,12,13);
    // println!("{:?}",z);

    let xs: [i32; 5] = [4, 5, 6, 7, 1];
    println!(
        "First value of the array : {}, length of the array: {}",
        xs[0],
        xs.len()
    );
    println!("Memory space taken: {} bytes.", mem::size_of_val(&xs));

    // slicing array
    let zs: [i32; 5] = [1, 2, 3, 4, 5];
    let ys = &zs[2..4]; // will start from 2 but this will not include the 4th element
    println!("{} {}", ys[0], ys[1]);
    //printing with square brackets
    println!("{:?} {:?}", ys, zs);

    // empty tuple
    let t_t = ();

    // little bit of string
    let s = "String".to_string();
    let ss = String::from("String");
    println!("{}", ss);

    // concat string
    let hh = "Hello, ".to_string();
    let ww = String::from("World!");
    let c_s = hh + &ww;
    println!("Concated String: {}", c_s);

    //slice string
    let text = String::from("String");
    let slice = &text[0..4];
    println!("Sliced string: {}", slice);
}
