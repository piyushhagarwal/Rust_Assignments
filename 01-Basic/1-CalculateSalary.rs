// To calculate salary of an employee given his basic pay (take as input from user).
// Calculate gross salary of employee. 
// Let HRA be 10 % of basic pay and TA be 5% of basic pay. 
//Let employee pay professional tax as 2% of total salary. 
//Calculate net salary payable after deductions.


// Steps to solve
// Take the basic pay of the employee as input from the user.
// Calculate the HRA (House Rent Allowance) by taking 10% of the basic pay.
// Calculate the TA (Travel Allowance) by taking 5% of the basic pay.
// Calculate the gross salary by adding the basic pay, HRA, and TA.
// Calculate the professional tax by taking 2% of the gross salary.
// Calculate the net salary by subtracting the professional tax from the gross salary.
// Print the net salary.


fn net_salary(base_salary : f64) -> f64 {
    let HRA = base_salary * 0.1;
    let TA = base_salary * 0.05;
    let gross_salary = base_salary + HRA + TA;
    let professional_tax = gross_salary * 0.02;

    let net_salary_amount = gross_salary - professional_tax;
    net_salary_amount
}

fn main(){
    let base_salary : f64 = 10000.0;
    let net_salary_amount : f64 = net_salary(base_salary);
    println!("The net salary for the base salary: {} is {}", base_salary, net_salary_amount);
}