fn main() {

    let tup: (i32, f64, u8) = (500, 6.7, 80);
    
    let (a, _b, c) = tup;

    println!("The first and third  tuples are: {a} and {c}");
}
