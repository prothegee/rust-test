use std::io;
use rand::Rng;


//
fn main()
{

    println!("Guess the numba!!");
    println!("Range is from 0 to 9");

    /*
    use mut to mute the variable
    - mean that you can't change the variable again
    */


    //      |name   |type   |bound
    let secret_numba = rand::thread_rng().gen_range(0, 9);

    println!("The secret number is: {}", secret_numba);
    println!("Now input it again");


    let mut guess = String::new();

    // using Rust standard library
    io::stdin()

        // handling the input
        .read_line(&mut guess)

        // handling potential failure
        .expect("Failed to read line");

    // familiar with this formaat?
    println!("Yo numba is: {}", guess);

}