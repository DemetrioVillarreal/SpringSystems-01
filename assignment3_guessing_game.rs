

fn check_guess(guess: i32, secret: i32) -> i32 {

  
    if guess == secret {
      
        0

      
    } else if guess > secret {

      
        1
    } else {

      
        -1
    }
}

fn main() {
    let secret_number: i32 = 5;
    let mut guess_count: i32 = 0;

    loop {
        let guess: i32 = guess_count + 3;


      
        guess_count = guess_count + 1;

        let result = check_guess(guess, secret_number);

      
        if result == 0 {
            println!("{} is correct!", guess);

          
            break;
          
        } else if result == 1 {

          
            println!("{} is too high", guess);
          
        } else {
            println!("{} is too low", guess);
        }
    }

    println!("It took {} guesses", guess_count);
}
