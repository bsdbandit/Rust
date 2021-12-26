//banditbox is  bsdbandit hacking toolset that automates tool installation 
//on kali linux ubuntu or secbsd etc etc ... 
//Written by bsdbandit



//Crates 
use figlet_rs::FIGfont;
extern crate os_info;

fn main() { 
    let standard_font = FIGfont::standand().unwrap();
    let _figure = standard_font.convert("Bandit Box");
    println!("{}", _figure.unwrap());

    operating_system();
}

//Checking the OS type 
fn operating_system() { 
    let info = os_info::get();
    let bandit = info.os_type();
    if bandit == bandit  {
        println!("This box is running freebsd");

    } else {
        println!("This box is running something else might need to check it out");
    }
        
    

       
    
}

