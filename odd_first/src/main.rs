
fn odd_first(array: Vec<i32>) {
    let mut output_odd = Vec::<i32>::new();
    let mut output_pair = Vec::<i32>::new();
    let mut output = Vec::<i32>::new();

    for elem in array.iter() {
        if elem % 2 != 0 {
            output_odd.push(*elem)
        } else {
            output_pair.push(*elem)
        }
    }
    for i in output_odd.iter() {
        output.push(*i)
    }
    output.reverse();
    for i in output_pair.iter() {
        output.push(*i)
    }
    println!("{:?}", output)
}

fn main() {
    let array = vec![1,2,3,4,5,6,7,8,9];
    odd_first(array) 
}
