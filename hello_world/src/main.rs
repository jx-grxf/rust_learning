// hello world

// stdio

fn main() {
    // print
    println!("Hello world!");

    // variables
    let number1 = 1;
    let number2 = 2;

    // printing and adding numbers
    println!("{}", number1 + number2);

    // for loops

    for _ in 1..5 {
        println!("This is a for loop!\n");
    }

    // if else & scan

    let age1 = 16;
    let age2 = 18;

    if age1 > 17 && age2 > 17 {
        println!("Both are allowed in!\n");
    } else {
        println!("Not allowed in!\n");
    }
}
