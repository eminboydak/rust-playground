fn main() {
    // variables and mutability
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    
    // const
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("3 hours to seconds: {THREE_HOURS_IN_SECONDS}");

    // shadowing
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value pf x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    // mut shadowing
    let spaces = "    ";
    let spaces = spaces.len();
    println!("Spaces lenght: {spaces}");

    //// this is not allowed
    //let mut spaces2 = "    ";
    //spaces2 = spaces2.len();
}
