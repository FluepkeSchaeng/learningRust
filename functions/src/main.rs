fn main() {
    another_function(5);
    print_labeled_measurements(5, 'h');

    // thie expression in curled brackets evaluates to 4
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");

    let five = five();
    println!("The value of five is: {five}");
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurements(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}
