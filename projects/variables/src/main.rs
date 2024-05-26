fn show_mutability() {
    println!("\nDemonstrating mutability ...");
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is now {x}");
}

fn show_shadowing() {
    println!("\nDemonstrate shadowing ...");
    let x = 5;
    let x = x + 1;
    println!("The value of x is {x}");

    {
        let x = x * 2;
        println!("The value of x is {x}");
    }

    println!("The value of x is {x}");
}

fn main() {
    show_mutability();
    show_shadowing();
}
