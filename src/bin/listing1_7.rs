fn replace_with_84(s: &mut Box<i32>){
    //let was = *s;
    println!("El valor inicial de s = {}", s);
    let was = std::mem::take(s);
    println!("El valor de was = {}", was);
    *s = was;
    println!("El valor de s = {}", *s);
    let mut r = Box::new(84);
    println!("El valor de r = {}", r);
    std::mem::swap(s, &mut r);
    println!("El valor de r = {} y s = {}", r,s);
    assert_ne!(*r,84)
}

fn main(){
    replace_with_84(&mut Box::new(42));
}