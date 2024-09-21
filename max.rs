fn main() {
    let array = [1, 4, 3, 18, 2];
    
    let max = max(array);
    
    println!("Il valore più grande è: {}", max);
    println!("{:?}", array);
}

fn max<const N: usize>(array: [i32; N]) -> i32 {
    let mut max: i32 = array[0];
    
    for &i in array.iter() {
        println!("{}", i);
        if i > max {
            max = i;
        }
    }
    
    return max;
}
