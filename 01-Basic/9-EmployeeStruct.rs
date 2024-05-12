// Create class EMPLOYEE for storing details (Name, Designation, gender, Date of Joining and Salary). 
// Define function members to compute 
// a)total number of employees in an organization 
// b) count of male and female employee 
// c) Employee with salary more than 10,000 

enum Designation{
    Manager,
    Developer,
    Intern
}

#[derive(PartialEq)] 
enum Gender {
    Male,
    Female,
    Other
}

struct Employee {
    name: String,
    designation: Designation,
    gender: Gender,
    date_of_joining: String,
    salary: i32
}

fn add_employee(employee_list: &mut Vec<Employee>, employee: Employee){
    employee_list.push(employee);
}

fn total_employees(employee_list: &Vec<Employee>) -> usize {
    employee_list.len()
}

fn count_male(employee_list: &Vec<Employee>) -> u32 {
    let mut male_count: u32 = 0;

    for employee in employee_list {
        if employee.gender == Gender::Male {
            male_count += 1;
        }
    }

    male_count
}

fn count_10k_plus_salary(employee_list: &Vec<Employee>) -> u32{
    let mut count: u32 = 0;

    for employee in employee_list {
        if employee.salary > 10000 {
            count += 1;
        }
    }

    count
}

fn main(){

    let mut employees_list: Vec<Employee> = Vec::new();
      
    let employee1 = Employee {
        name: String::from("Piyush Agarwal"),
        designation: Designation::Intern,
        gender: Gender::Male,
        date_of_joining: String::from("12/05/2024"),
        salary: 25000
    };

    add_employee(&mut employees_list, employee1);

    let employee2 = Employee {
        name: String::from("Esha Agarwal"),
        designation: Designation::Developer,
        gender: Gender::Female,
        date_of_joining: String::from("12/05/2022"),
        salary: 250000
    };

    add_employee(&mut employees_list, employee2);

    let employee3 = Employee {
        name: String::from("Aarav Arora"),
        designation: Designation::Intern,
        gender: Gender::Male,
        date_of_joining: String::from("12/05/2024"),
        salary: 5000
    };

    add_employee(&mut employees_list, employee3);

    println!("The number of employees are {}", total_employees(&employees_list));

    println!("The number of male employees are {}", count_male(&employees_list));

    println!("The number of employees whose salary is more than 10k are {}", count_10k_plus_salary(&employees_list));


}

// In Rust, you're correct that you can only have one mutable reference to a particular piece of data at a time. However, in the code you've provided, the mutable reference is not being shared across different functions simultaneously. Let's break it down:

// In the add_employee function, a mutable reference &mut Vec<Employee> is taken as an argument. This mutable reference is only valid within the scope of the add_employee function. When the function call ends, the mutable reference goes out of scope.
// In the total_employees function, an immutable reference &Vec<Employee> is taken as an argument. This is allowed because you can have multiple immutable references to the same data simultaneously.
// In the main function, the mutable reference &mut employees_list is passed to the add_employee function three times. However, each time it is passed, a new mutable reference is created for the duration of the function call, and it goes out of scope once the function call returns.

// Here's the sequence of events:

// employees_list is created as a mutable vector in the main function.
// When add_employee(&mut employees_list, employee1) is called, a new mutable reference is created for the duration of the add_employee function call. After the call returns, the mutable reference goes out of scope.
// When add_employee(&mut employees_list, employee2) is called, a new mutable reference is created for the duration of the add_employee function call. After the call returns, the mutable reference goes out of scope.
// When add_employee(&mut employees_list, employee3) is called, a new mutable reference is created for the duration of the add_employee function call. After the call returns, the mutable reference goes out of scope.
// When total_employees(&employees_list) is called, an immutable reference is created, which is allowed because there are no active mutable references at that point.

// So, the mutable references are not being shared across different functions simultaneously. Each function gets a new mutable reference for the duration of its call, and the reference goes out of scope when the function returns.