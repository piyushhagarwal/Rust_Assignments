// To check whether input number is Armstrong number or not. 
// An Armstrong number is an integer with three digits such that the sum of the cubes of its digits is equal to the number itself. Ex. 371.

fn is_armstong(num: i32) -> bool {
    let mut sum = 0;
    let mut temp_num = num;

    while temp_num > 0 {
        let digit = temp_num % 10;
        sum += digit.pow(3);
        temp_num = temp_num / 10;
    }
    
    if sum == num {
        return true;
    } else {
        return false;
    }
}


fn main(){
    println!("{}", is_armstong(377));
    println!("{}", is_armstong(371));
}