fn main() {
    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    if_else_expression();
    if_on_the_right();
    while_concept();
    for_loop_concept();
    for_loop_range();
}

fn if_else_expression() {
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3 or 2");
    }
}

fn if_on_the_right() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);
}
fn while_concept() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }
    println!("LITOFF!!");
}
fn for_loop_concept() {
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is {}", element);
    }
}
fn for_loop_range() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LITOFF!");
}
