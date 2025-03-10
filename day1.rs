//fn main () {
   //let emp_name = emp_info.0;
  // let emp_age = emp_info.1;
    //destructring 
   // let (empp_name, empp_age ,) = emp_info;
//println!("Employee Name {}, Employee Age {}", empp_name , empp_age);
    
   // println!("Employee Name {}, Employee Age {}", emp_name , emp_age);
   
   //let num_1:u8 = 26;
  // let num_2:u8 = 35;
   
  // let result:u8 = add(num_1,num_2);
   //println!("Result is {}",result);
   
//}

//fn add(item1:u8, item2:u8) -> u8{
  //  return item1+item2;
    
//}


//fn main (){
    //let name:String = String::from ("Hello");
    //outside(name);
    //println!("{}",name);
//}

//fn outside(items:String){
   // println!("{}", items);
//}


//use std::io;

//fn main() {
    // Create a new string to store user input
    //let mut input = String::new();

    //println!("Enter something:");

    // Read the input from the user
   // io::stdin()
       // .read_line(&mut input)
       // .expect("Failed to read input");

    // Print the input back to the user
    //println!("You entered: {}", input.trim());
//}

//fn main (){
  // let name:String = get_string();
   // println!(" Name {}",name); 
    //Returning the Name so name is the owner of Hello.
    
   // let name2:String = String::from("World"); 
   // let name3:String = sent_get_string(name2); // transfer the ownrship world to recieved_string.
   // println!(" the name is now {}" , name3);
//}

//fn get_string()-> String{
    //let new_string:String = String::from("Hello"); //New string owner
    //return new_string; // transfering the owner
//}
//owner of world
//fn sent_get_string (recived_string:String ) -> String{
    //return recived_string; //transfer of ownership again recieved to name3
    
//}
