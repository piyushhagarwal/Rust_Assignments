// To accept an object mass in kilograms and velocity in meters per second and display its momentum.
// Momentum is calculated as e=mc^2 where m is the mass of the object and c is its velocity.

fn momentum(mass : f64, velocity : f64) -> f64{
    let momentum_value = mass * velocity.powf(2.0);
    momentum_value
}
fn main(){
    let mass = 60.0;
    let velocity = 5.0;
    let momentum_value = momentum(mass, velocity);

    println!("The momentum of mass: {} and velocity : {} is {}", mass, velocity, momentum_value);
}