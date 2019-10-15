use std::io;
//1 is mean to perform addition 
//2 is mean to perform Multiplication 
//3 is mean to perform Division 
//4 is mean to perform Subtraction 
//5 is mean to perform Factorial
//6 is mean to perform Square
//7 is mean to perform Sqare Root 
fn main() {
    println!("<<<<<<<<<Calculator>>>>>>>>>>>");
    println!("<<<<<<<<<THIS Calculator WILL PERFORM BASIC CALCULATIONS AND OPERATIONS>>>>>>>>>>>");
    let mut key=String::new(); //Decalare a string variable
    println!("ENTER THE FUNCTION YOU WANT TO PERFORM");
    io::stdin().read_line(&mut key).ok();
    let key:i32=key.trim().parse().unwrap();

   // println!("The Input value is : {}",key);
   if key==1{ 
       println!("You want to perform Addition");
       add();
   }
   else if key==2{
        println!("You want to perform Multiplication");
        multiplication();
   }
   else if key==3{
        println!("You want to perform Division");
        division();
   }
   else if key==4{
        println!("You want to perform Subtraction");
        subtraction();
   }
   else if key==5{
        println!("You want to perform Factorial");
        factorial();
   }
    else if key==6{
        println!("You want to perform Square Operations");
        square();
   }
    else if key==7{
        println!("You want to perform Square Root Operations");
        square_root();
   }
    else if key==8{
        println!("You want to perform Modulus Operation");
        modules();
   }
   else{
       println!("You entered a wrong value");
   }

}

fn add(){
    println!("Please Enter First number that you want to add");
    let mut num1= String::new();
    io::stdin().read_line(&mut num1).ok();
    let num1:i32=num1.trim().parse().unwrap(); 

    println!("Please Enter Second number that you want to add");
    let mut num2= String::new();
    io::stdin().read_line(&mut num2).ok();
    let num2:i32=num2.trim().parse().unwrap(); 
    println!("The Result of Given number is: {}",(num1+num2));
}
fn multiplication(){
    println!("Please Enter First number that you want to Multiplication");
    let mut num1= String::new();
    io::stdin().read_line(&mut num1).ok();
    let num1:i32=num1.trim().parse().unwrap(); 

    println!("Please Enter Second number that you want to Multiplication");
    let mut num2= String::new();
    io::stdin().read_line(&mut num2).ok();
    let num2:i32=num2.trim().parse().unwrap(); 
    println!("The Result of Given number is: {}",(num1*num2));
}
fn subtraction(){
    println!("Please Enter First number that you want to subtraction");
    let mut num1= String::new();
    io::stdin().read_line(&mut num1).ok();
    let num1:i32=num1.trim().parse().unwrap(); 

    println!("Please Enter Second number that you want to subtraction");
    let mut num2= String::new();
    io::stdin().read_line(&mut num2).ok();
    let num2:i32=num2.trim().parse().unwrap(); 
    println!("The Result of Given number is: {}",(num1-num2));
}
fn division(){
    println!("Please Enter First number that you want to divide");
    let mut num1= String::new();
    io::stdin().read_line(&mut num1).ok();
    let num1:i32=num1.trim().parse().unwrap(); 

    println!("Please Enter Second number that you want to divide");
    let mut num2= String::new();
    io::stdin().read_line(&mut num2).ok();
    let num2:i32=num2.trim().parse().unwrap(); 
    println!("The Result of Given number is: {}",(num1/num2));
}
fn modules(){
    println!("Please Enter First number that you want to Remainder");
    let mut num1= String::new();
    io::stdin().read_line(&mut num1).ok();
    let num1:i32=num1.trim().parse().unwrap(); 

    println!("Please Enter Second number that you want to Remainder");
    let mut num2= String::new();
    io::stdin().read_line(&mut num2).ok();
    let num2:i32=num2.trim().parse().unwrap(); 
    println!("The Remainder of Given number is: {}",(num1%num2));
}
fn factorial(){
    println!("Please Enter First number that you want to calculate Factorial");
    let mut num1= String::new();
    io::stdin().read_line(&mut num1).ok();
    let mut num1:i32=num1.trim().parse().unwrap(); 
    let mut fact=1;
    let x=num1;
    // while num1>0{  //Using while loop
    //     fact=fact*num1;
    //     num1=num1-1;
    // }
    loop{  //using Loop method
        fact=fact*num1;
        num1= num1-1;
        if num1==0{
            break ;
        }
    }
    println!("The Factorial of {} is: {}",x,fact);
}
fn square(){
    println!("Please Enter number that you want to calculate Square");
    let mut num1= String::new();
    io::stdin().read_line(&mut num1).ok();
    let  num1:f64=num1.trim().parse().unwrap(); 
    println!("The Square of number is: {}",num1.powi(2));
}
fn square_root(){
    println!("Please Enter number that you want to calculate Square Root");
    let mut num1= String::new();
    io::stdin().read_line(&mut num1).ok();
    let  num1:f64=num1.trim().parse().unwrap(); 
    println!("The Square Root of number is: {}",num1.sqrt());
}