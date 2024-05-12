// To accept N numbers from user. 
//Compute and display maximum in list, minimum in list, sum and average of numbers.


// Function that accepts a list of numbers and returns a tuple of (max, min, sum, average)
fn compute(list: [i32;5]) -> (i32, i32, i32, f64){
    let mut max = list[0];
    let mut min = list[1];
    let mut sum = 0;

    for i in 0..list.len(){
        if list[i] > max{
            max = list[i];
        }
        if list[i] < min{
            min = list[i];
        }
        sum += list[i];
    }

    let avg = sum as f64 / list.len() as f64;

    (max, min, sum, avg)
}

fn main(){
    let list: [i32; 5] = [1, 2, 3, 4, 6];

    let result = compute(list);

    println!("Max: {}", result.0);
    println!("Min: {}", result.1);
    println!("Sum: {}", result.2);
    println!("Average: {}", result.3);
}