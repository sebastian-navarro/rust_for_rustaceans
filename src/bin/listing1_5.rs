
fn noalias(input: &i32, output: &mut i32) -> i32{
    if *input == 1 {
        *output = 2;
    }
    if *input != 1 {
        *output = 3;
    }

    return *output;
   
}

fn main(){
    let input = 3;
    let mut output = 0;
    let result = noalias(&input, &mut output);
    println!("Result = {:?}", result);

    println!(" Works print input = {} and sum = {} ??? : Yes!!!!", input, output)


    
}