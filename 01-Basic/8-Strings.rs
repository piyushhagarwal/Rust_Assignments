// Write a python program that accepts a string from user and perform following string operations- 
// i. String reversal 
// ii. Equality check of two strings 
// iii. Check palindrome 
// iv. Check substring


// Leaning 1: We cant use two pointer approach to reverse the string in Rust as we can do in Java.
fn string_reversal(s: &str) -> String {
    let reverse_string: String = s.chars().rev().collect();
    return reverse_string;
}

// Learning 2: We can use == operator to compare two strings in Rust.
// First it check the address of the two strings, if they are same then it returns true.
// If the address are different then it compares length of the two strings, if they are different then it returns false.
// If the length are same then it compares each character of the two strings, if they are same then it returns true.

// In contrast, in Java we can't use == operator to compare two strings. We have to use equals() method.
// In Java, == operator compares the address of the two strings, if they are different then it returns false.
// If the address are same then it returns true.
// So, in Java we have to use equals() method to compare the content of the two strings.

fn string_equality(s1: &str, s2: &str) -> bool {
    return s1 == s2;
}

fn check_palindrome(s: &str) -> bool {
    let reverse_string: String = s.chars().rev().collect();
    return s == reverse_string; // We can compare &str and String in Rust.
}

fn check_substring(s1: &str, s2: &str) -> bool {
    return s1.contains(s2); 
}

fn main(){
    let s = "Hello World";
    let reversed_string = string_reversal(s);
    println!("Reversed String: {}", reversed_string);

    let s1 = "Hello".to_string();
    let s2 = "Hello".to_string();
    let is_equal = string_equality(&s1, &s2);
    println!("Is Equal: {}", is_equal); // true

    let s = "madam";
    let is_palindrome = check_palindrome(s);
    println!("Is Palindrome: {}", is_palindrome); // true

    let s1 = "Hello World";
    let s2 = "World";

    let is_substring = check_substring(s1, s2);
    println!("Is Substring: {}", is_substring); // true
    
}