

fn cache(input: &i32, sum: &mut i32) {
    *sum = *input + *input;
    println!("sum= {}", sum);
    assert_eq!(*sum, 2* *input);
}

fn main(){
    let input = 10;
    let mut sum = 10;
    cache(&input, &mut sum);
}