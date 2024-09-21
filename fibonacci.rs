fn main() {
    //Fibonacci sequence:
    //1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144..
    //F0=1
    //F1=1
    //F2=2..
    //----------------------------------------
    //written expression: 
    //Fn = Fn1 - Fn2
    //Fn=F(n-1)+F(n-2)
    //----------------------------------------
    //the "n" after "F" is the position
    //so the Fn(final_res||new_result) = (new_result) + (old_result)
    //so new_res is: Fn2 = Fn2 + Fn1
    //and old_res is: Fn1 = Fn2 - Fn1

    //insert the fibonacci position you want to find
    let num =10 ;
    
    let _result= fib(num);
    //now you have the result of fibonacci in position "num" :)
}

fn fib (num: i32) -> i32 {
    let mut new_res = 1;
    let mut old_res = 0;
    
        for i in 0..num{
            new_res = new_res + old_res;
            old_res = new_res - old_res;
            //print old_res:
            println!("fib({i}): {old_res}");   
        }
        //this is the num position result:
        println!("result: fib({num}): {new_res}");
        return new_res;
}