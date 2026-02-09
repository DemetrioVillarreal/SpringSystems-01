

const FREEZING_POINT: f64 = 32.0;




fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - FREEZING_POINT) * 5.0 / 9.0
}



fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + FREEZING_POINT
}



fn main() {



  
    let mut temperature_f: f64 = 32.0;


  
    let temperature_c = fahrenheit_to_celsius(temperature_f);
  
    println!("{} Fahrenheit is {} Celsius", temperature_f, temperature_c);

  
    for _ in 0..5 {
      
        temperature_f = temperature_f + 1.0;
      
        let c = fahrenheit_to_celsius(temperature_f);

      
        println!("{} Fahrenheit is {} Celsius", temperature_f, c);
      
    }


  
}
