fn sum_vec(v: &[i32]) -> i32 {
    //    let mut sum = 0;
    //    for val in v {
    //        sum += val;
    //    }
    //    sum

    v.iter().sum()
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    println!("De som is: {}", sum_vec(&numbers));
}
