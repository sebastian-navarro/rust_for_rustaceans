

fn main() {
    let x = 42;
    let y = 43;

    println!("1.- The value of variables x and y are:  {} and {} and his directions are {:p},{:p}",x,y,&x,&y);
    let var1 = &x;

    println!("2.- The value of the variable var1 = {:p} and the value point to the value of x = {}.", var1, *var1);
    let mut var2 = &x;
    println!("3.- The value of the variable var2 = {:p} and the value point to the value of x =  {}.", var2, *var2);
    println!("4.- var1 and var2 point to the variable x, that is why they have the same value 'direction'.");
    var2 = &y;

    println!("5.- Now in var2, we change the value by 'y', then value of var2 = {:p} and point to the value y = {}.",var2, *var2);

    let string = "Hello world";
    println!("What is happen with the variable string = {}, and the direction is {:p}", string, &string);
    println!("This pointer only point to the stack, and the stack point to the heap of all the values (Maybe is as linked list)");
    println!("Value of the first element = {:p}  , and the second element is {:p}",&string.chars().nth(0), &string.chars().nth(1));
}