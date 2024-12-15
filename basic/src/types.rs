fn types() {
    //numbers
    let add = 3 + 8; // i32
    let sub = 26.7 - 8.2; // f64
    let mul = 6 * 4; // i32
    let div = 10.0 / 3.14; // f64
    let truncated = 10 / 3; // i32, is 3
    let remainder = 10 % 3; // i32

    //boolean
    let t = true;
    let f: bool = false;

    //character
    let c = 'z';
    let z: char = 'Z';
    let heart_eyed_cat = '😻'; //unicode

    //compound types
    //tuple
    let tup: (i32, f64, u8, bool) = (500, 6.4, 1, false);
    let (x, y, z, w) = tup; //destructuring
    let six_point_four = tup.1; //accessing by index

    //array
    let a: [i32; 5] = [1, 2, 3, 4, 5]; //fixed length, same type
    let first = a[0];
    let threes = [3; 5]; // [3, 3, 3, 3, 3]
}
