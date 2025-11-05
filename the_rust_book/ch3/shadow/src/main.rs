fn main() {

    let x = 5;

    let x = x + 1;

    let space = "    ";
    let space = space.len();


    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The Value of x is: {x}");
}
