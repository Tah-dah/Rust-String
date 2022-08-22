use substring::Substring;
use std::io::stdin;



fn main() {
    let mut string =String::new(); // user input string
    stdin().read_line(&mut string)
        .ok()
        .expect("failed readline");
    
        if string.len() > 3 {
            let _firstthree = string.substring(0, 3).to_owned();
            //let substr = string.len() + _firstthree.len();
            let backstr = string.substring(3,string.len()).to_owned();
            //let newstr = backstr +&_firstthree; 
                println!("{}", backstr +&_firstthree);
        } else {
                 println!("{}", "use a larger string");
    }
}


