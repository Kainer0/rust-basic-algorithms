fn main() {
    //Fibonacci sequence:
    //0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144..

    //written expression: 
    //Fn = Fn1 - Fn2

    //insert the fibonacci position you want to find
    let num =10 ;
    
    let mut new_res = 1;
    let mut old_res = 0;
     
    for i in 0..num{
        
        new_res = new_res + old_res;
        old_res = new_res - old_res;
        
        println!("fib({i}): {old_res}");
    }
    println!("risultato: fib({num}): {new_res}");
}

fn fib (num: i32) -> i32 {
    if (num <= 0){
        return 0;
    } else if (num = 1){
        return 1;
    } else{
        let
        for (0..num){

        }
    }
}