use substring::Substring;



fn main() {
    let string = "launchcode";
    let  backstr = string.substring(3,10).to_owned();
    let  new_string = string.substring(0,3).to_owned();
    
    let fullstr = backstr + &new_string;
     
    println!("{}", fullstr);

}


