// To accept student's five courses marks and compute his/her result. 
// Student is passing if he/she scores marks equal to and above 40 in each course. 
// If student scores aggregate greater than 75%, then the grade is distinction. 
// If aggregate is 60>= and <75 then the grade if first division. 
// If aggregate is 50>= and <60, then the grade is second division. 
// If aggregate is 40>= and <50, then the grade is third division.

#[derive(Debug)]
enum Grade{
    Distinction,
    FirstDivision,
    SecondDivision,
    ThirdDivision,
    Fail
}

fn grade(list: [u8; 5]) -> Grade{
    let mut sum: u32 = 0;

    // Check score of each course and return Fail if any course marks is less than 40
    for marks in list{
        if marks <= 40 {
            return Grade::Fail;
        }

        sum += marks as u32;
    }

    let aggregate = sum as f64 / 5.0;

    if aggregate > 75.0 {
        return Grade::Distinction;
    } else if aggregate >= 60.0 {
        return Grade::FirstDivision;
    } else if aggregate >= 50.0 {
        return Grade::SecondDivision;
    } else if aggregate >= 40.0 {
        return Grade::ThirdDivision;
    } else {
        return Grade::Fail;
    }
}

fn main(){
    let marks1 = [80, 75, 90, 85, 70];
    let marks2 = [60, 65, 70, 55, 75];
    let marks3 = [35, 45, 50, 55, 60]; // One course mark is less than 40

    println!("Marks1: {:?}", grade(marks1));
    println!("Marks2: {:?}", grade(marks2));
    println!("Marks3: {:?}", grade(marks3));
}