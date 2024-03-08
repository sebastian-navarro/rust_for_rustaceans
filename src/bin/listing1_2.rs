fn main(){
    let x;
    x = 42;

    let y = &x;

    // Next doesnt work for the y borrowing
    //x = 43;


    assert_eq!(*y, 42);
}