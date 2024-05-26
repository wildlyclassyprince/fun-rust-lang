use std::{thread, time};

fn main() {
    simple_example();
    if_in_a_let();
    simple_loop();
    loop_labels();
    count_down();
    count_down_v2();
    count_down_v3();
    loop_over_array();
    loop_over_array_v2();
}

fn simple_example() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

fn if_in_a_let() {
    let condition = true;
    let x = if condition { 4 } else { 7 };
    println!("The value of x is {x}");
}

fn simple_loop() {
    let mut counter: i32 = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn loop_labels() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn count_down() {
    let mut counter: i32 = 3;
    println!("Count down ...");

    let one_seconds = time::Duration::new(1, 0);

    'count_down: loop {
        if counter == 0 {
            println!("Boom!");
            break 'count_down;
        }

        // sleep for 3 seconds before printing next value
        thread::sleep(one_seconds);
        println!("{counter}");
        counter -= 1;
    }
}

fn count_down_v2() {
    let mut counter: i32 = 3;
    let one_second = time::Duration::new(1, 0);

    println!("Count down (v.2.0) ...");
    while counter >= 0 {
        thread::sleep(one_second);
        println!("{counter}");
        counter -= 1;
    }

    println!("Boom!")
}

fn count_down_v3() {
    let one_second = time::Duration::new(1, 0);

    println!("Count down (v.3.0) ...");
    for counter in (1..4).rev() {
        thread::sleep(one_second);
        println!("{counter}");
    }
    println!("Boom!");
}

fn loop_over_array() {
    let a: [i32; 5] = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("Element in position {} is {}", index, a[index]);
        index += 1;
    }
}

fn loop_over_array_v2() {
    // this is better than using a while statement
    // since compiler does not need to add code to check
    // if while condition is valid at runtime
    let a: [i32; 5] = [3, 2, 5, 8, 0];

    for element in a {
        println!("{element} is an element in the array");
    }
}
