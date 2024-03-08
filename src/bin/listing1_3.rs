fn main(){
    let x1 = 42;
    let y1 = Box::new(42);
    // We can not compare both values directly
    //println!("{}", x1 == y1);

    {
        let _z = (x1,y1);
        println!("Z is the owner now from the values of x1,y1 ")
    }
    let _x2 = x1; // This work because i32 use the trait Copy indirectly
    let _x3 = y1; // This doesnt work because Box y1 doesnt implement Copy Trait
    // Solution create a pointer to the value let _z = (x1,&y1) or implement Copy Trait to the value;

}