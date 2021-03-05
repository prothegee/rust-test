use std::io;
use rand::Rng;
use std::cmp::Ordering;


//
fn main()
{

    println!("Read the numba!!");
    println!("Range is from 0 to 9");

    /*
    use mut to mute the variable
    - mean that you can't change the variable again
    */


    //      |name   |type   |bound
    let secret_numba = rand::thread_rng().gen_range(0, 9);

    println!("The secret numba is: {}", secret_numba);
    println!("So what is the numba?");


    // let's match it
    loop
    {
        let mut guess = String::new();

        // using Rust standard library
        io::stdin()

            // handling the input
            .read_line(&mut guess)

            // handling potential failure
            .expect("Failed to read line");

        // handling invalid input
        // let guess: u32 = guess.trim().parse().expect("Please type a number");
        let guess: u32 = match guess.trim().parse()
        {
            Ok(num) => num,
            Err(_) => continue,
        };

        // familiar with this formaat?
        println!("Yo numba is: {}", guess);

        match guess.cmp(&secret_numba)
        {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

}