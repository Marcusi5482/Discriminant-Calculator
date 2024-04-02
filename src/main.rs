use std::io;

fn main() {

    //This is where we assign variables to the strings
    loop {
    let mut a_str:String = String::new();
    let mut b_str:String = String::new();
    let mut c_str:String = String::new();

    println!("Solve the quadratic equation");

    println!("Enter a");
    //this is where we get input from the user
    match io::stdin().read_line(&mut a_str) {
        Ok(_) => {},
        Err(e) => println!("Input error - {}", e)
    }
    println!("Enter b");
    match io::stdin().read_line(&mut b_str) {
        Ok(_) => {},
        Err(e) => println!("Input error - {}", e)
    }
    println!("Enter c");
    match io::stdin().read_line(&mut c_str) {
        Ok(_) => {},
        Err(e) => println!("Input error - {}", e)
    }
//This is where we convert the string to input
    let a: f64 = a_str.trim().parse().unwrap();
    let b: f64 = b_str.trim().parse().unwrap();
    let c: f64 = c_str.trim().parse().unwrap();
//here we calculate the discriminant
    let d: f64 = (b*b) - 4.0 * a * c;

    if d > 0.0 {
    let x1 = ((-b) + d.sqrt()) / (2.0 * a);
    let x2 = ((-b) - d.sqrt()) / (2.0 * a);
    println!("Resolved\nthere are two roots\nD = {}\nRoot 1 = {}\nRoot 2 = {}",d ,x1 ,x2);
    }

    if d == 0.0 {
        let x = (-b) / (2.0 * a);
        println!("Resolved\nthere's one root\nD = 0\nRoot = {}", x);
    }

    if d < 0.0 {
        println!("Roots don't exist!\nD < 0\nD = {}",d);
    }
}

}
