fn main(){
    println!("===簡単な計算機===\n");

    let a = 10;
    let b = 5;

    let sum = add(a,b);
    let diff = subtract(a,b);
    let prod = multiply(a,b);
    let quot = divide(a,b);

    println!("a = {}, b= {}", a, b);
    println!();
    println!("足し算：{} + {} = {}", a, b, sum);
    println!("引き算：{} - {} = {}", a, b, diff);
    println!("掛け算：{} * {} = {}", a, b, prod);
    println!("割り算：{} / {} = {}", a, b, quot);
}

fn add(x:i32,y:i32)->i32{
    x + y
}

fn subtract(x:i32,y:i32)->i32{
    x - y
}

fn multiply(x:i32,y:i32)->i32{
    x * y
}

fn divide(x:i32,y:i32)->i32{
    x / y
}
