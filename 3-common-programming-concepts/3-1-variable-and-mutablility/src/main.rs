fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    // Does not work because variable is immutable by default ir rust
    // x = 6;
    // println!("The value of x is: {x}");

    let mut y = 6;
    println!("The value of y is: {y}");

    y = 8;
    println!("The new value of y is: {y}");

    // Constantes
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 30;
    println!("Constante: {THREE_HOURS_IN_SECONDS}");

    // Shadowing
    let sh = 6;

    let sh = sh + 1;

    {
        let sh = sh * 2;
        println!("The value of sh in the inner scope is: {sh}");
    }

    println!("The value of sh is: {sh}");

    // Another example of shadow
    // The difference between shadowing and mutable are that we use
    // let keyword to create a new variable with the same name
    let spaces = "----------";
    println!("Spaces: {spaces}");
    let spaces = spaces.len();
    println!("Spaces: {spaces}");
}
