fn main() {
    let array=[1,4,3,2,2];
    let mut max:i32;
    
    for i in array {
        println!("{}",i);
        if i>max {
            max=i;
        }
    }
    
    println!("il valore piu' grande e': {}",max);
    println!("{:?}",array);
    
}