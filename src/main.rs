//! reads comma-separated lines from a file, 
//! calculates the difference between the two numeric fields
//! writes lines to another file, with the same values,
//! adding an information about the state according to the difference
//! 
//! TO DO: 
//! - refactor the code: maybe the line as a struct? 

use std::fs::File;
use std::io::{Write, BufWriter, BufReader, BufRead, Error};

fn main() -> Result<(), Error> {

    let read_path = std::env::args().nth(1).unwrap();
    let write_path = std::env::args().nth(2).unwrap();

    let input = File::open(read_path)?;
    let buffered = BufReader::new(input);
    
    let output = File::create(write_path).unwrap();
    let mut buffered_output = BufWriter::new(output);

    let sep = ','.to_string();
    //let end = '\n'.to_string();

    for (idx, line) in buffered.lines().enumerate() {
                
        let new_line = line?;

        let v: Vec<&str> = new_line.split(&sep).collect();

        let (line_idx,name,previous,current) = (v[0], v[1], v[2], v[3]);

        let mut state = "state";

        if idx > 0 {

            let p = previous.parse().unwrap_or(0);
            let c = current.parse().unwrap_or(0);
            state = status(p,c);

        };


        //let new_string = line_idx.to_string() + &sep + name + &sep + &previous.to_string() 
        //                + &sep + &current.to_string() + &sep + state + &end;

        let new_string = format!("{},{},{},{},{}\n",line_idx, name, &previous, &current, state);

        write!(buffered_output, "{}", new_string).unwrap();

    }
    
    Ok(())
    
}


fn status(previous: i8, current: i8) -> &'static str {

    let difference: i8 = current - previous;
    let mut state = "no change";

    if difference < 0 {
        if current == 0 {
            state = "termination";
        } else {
            state = "removal"
        }            
    } 
    else if difference > 0 {
        if previous == 0 {
            state = "new";
        } else {
            state = "addition"
        }
    };

    state

}


#[cfg(test)]

mod tests {
    use super::*;
    
    #[test]        
    fn no_change() {
        let test_status = status(0,0);
        assert_eq!(test_status, "no change");
    }

    #[test]        
    fn removal() {
        let test_status = status(3,1);
        assert_eq!(test_status, "removal");
    }

    #[test]        
    fn addition() {
        let test_status = status(2,5);
        assert_eq!(test_status, "addition");
    }

    #[test]        
    fn brand_new() {
        let test_status = status(0,1);
        assert_eq!(test_status, "new");
    }

    #[test]        
    fn termination() {
        let test_status = status(3,0);
        assert_eq!(test_status, "termination");
    }
}
