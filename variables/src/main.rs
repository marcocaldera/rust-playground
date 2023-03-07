fn main() {
    let mut x = 5;
    // let x = 5; // will cause error because x is immutable 

    println!("The value of x is: {x}");

    x = 6;

    println!("The value of x is: {x}");

    // const are always immutable (you can't use the "mut" keyword)
    // with const you must the clear the type
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Var: {THREE_HOURS_IN_SECONDS}");

    shadowing();
}

fn shadowing() {
    let x = 5;

    // the first variable is shadowed by the second
    // yes, you can redefine variables
    let x = x + 1;

    // inner scope
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

// Shadowing is different from marking a variable as mut because we'll get a compile-time error
// if we accidentally try to reassign to this variable without using the let keyword.
// By using let, we can perform a few transformations on a value but have the variable be immutable after those transformations have been completed.
//
// The other difference between mut and shadowing is that because we're effectively creating 
// a new variable when we use the let keyword again, we can change the type of the value but reuse the same name.
// For example, say our program asks a user to show how many spaces they want between some text
// by inputting space characters, and then we want to store that input as a number:
// 
// let spaces = "   ";
// let spaces = spaces.len();