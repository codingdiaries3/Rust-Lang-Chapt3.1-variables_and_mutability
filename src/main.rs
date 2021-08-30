fn main() {
  //Variables and Mutability Section
  let mut x = 5;
  println!("The value of x is: {}", x);
  
  x = 6;
  println!("The value of x is: {}", x);

 //Shadowing Section

 let x = 5;

 let x = x + 1;

 let x = x * 2;

 println!("The value of x is: {}", x)
}
