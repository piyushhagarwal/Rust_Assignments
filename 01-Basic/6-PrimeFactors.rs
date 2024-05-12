fn prime_factors(mut num : i32) -> Vec<i32>{
    let mut prime_factors_list: Vec<i32> = Vec::new();
    let mut divisor = 2;

    while num > 1 {
        if num % divisor == 0 {
            num = num / divisor;
            prime_factors_list.push(divisor)
        } else {
            divisor += 1;
        }
    }

    prime_factors_list
}

fn main(){
    let prime_factors_list = prime_factors(45);
    for i in &prime_factors_list {
        println!("{i}");
    }
}