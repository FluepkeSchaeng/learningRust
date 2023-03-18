/*
   Goals:
   - convert temperatures between Celsius and Fahrenheit
   - generate the nth fibonacci number
   - print the lyrics to the Christmas carol "The Twelve Days of Christmas",
     taking advantage ot the repetition in the song
     -> I don't do the last one yet beecause I don't like christmas songs very much :D
*/

fn main() {
    let upper_bound: u32 = 11;
    println!("Printing Fibonacci numbers from 1 to {}", upper_bound - 1);

    for x in 1..upper_bound {
        let fibonacci: u128 = fibonacci(x);
        println!("Fibonacci of {x}: {fibonacci}");
    }

    let celsius: f64 = 20.0;
    let fahrenheit = celsius_to_fahrenheit(celsius);
    println!("{celsius}°C are {fahrenheit}°F");

    let fahrenheit: f64 = 90.0;
    let celsius: f64 = fahrenheit_to_celsius(fahrenheit);
    println!("{fahrenheit}°F are {celsius}°C");
}

fn fibonacci(n: u32) -> u128 {
    if n == 1 || n == 2 {
        return 1;
    }
    return fibonacci(n - 1) + fibonacci(n - 2);
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    return celsius * 1.8 + 32.0;
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    return (fahrenheit - 32.0) * 5.0 / 9.0;
}
