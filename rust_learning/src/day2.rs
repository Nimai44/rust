fn main (){

    let mut s1:String = String::from ("Hello");
     append_string(&mut s1);
     println!("The New string is {}",s1);
    
}

fn main(s3:&mut String){
     s3.push_str(" world");
   println!("The New string is {}",s3);
 }

fn main (){
     let mut s1:String = String::from ("Hello");
     let s2:String = String::from ("Nimo"); 
     append_string(&mut s1);
    println!("The New string is {}",s1);    
    println!("The New string is {}",s2);
} 

fn append_string(s3:&mut String){
         s3.push_str(" world"); 
        
         
}