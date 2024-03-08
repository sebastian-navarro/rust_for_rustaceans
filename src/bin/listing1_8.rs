fn main(){
    let mut x = Box::new(42);
    let r = &x;  // 'a
    if rand() > 0.5 {
        *x = 84;
    } else {
        println!("{}", r);  // 'a
    }

}