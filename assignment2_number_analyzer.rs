fn is_even(n: i32) -> bool {

  
    n % 2 == 0

  
}

fn main() {

  
    let numbers: [i32; 10] = [1, 3, 5, 6, 9, 10, 12, 15, 18, 20];

    for n in numbers {
      
        if n % 3 == 0 && n % 5 == 0 {


          
            println!("{} FizzBuzz", n);
        } else if n % 3 == 0 {
          
            println!("{} Fizz", n);
        } else if n % 5 == 0 {



          
            println!("{} Buzz", n);
        } else if is_even(n) {
            println!("{} Even", n);

          
        } else {
            println!("{} Odd", n);
        }
    }

    let mut sum: i32 = 0;
    let mut index: usize = 0;

  

    while index < numbers.len() {
        sum = sum + numbers[index];
        index = index + 1;
    }

    println!("Sum is {}", sum);

    let mut largest: i32 = numbers[0];

    for n in numbers {
      
        if n > largest {

          
            largest = n;

          
        }
    }



  
    println!("Largest number is {}", largest);
}
