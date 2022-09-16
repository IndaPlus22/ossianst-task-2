/***
 * Template to a Kattis solution.
 * See: https://open.kattis.com/help/rust
 * Author: Viola Sderlund <violaso@kth.se>
 */

 use std::io;
 use std::io::prelude::*;
 
 /// Kattis calls main function to run your solution.
 fn main() {
     // get standard input stream
     let input = io::stdin();
 
     // get input lines as strings
     // see: https://doc.rust-lang.org/std/io/trait.BufRead.html
     let mut lines = input
         .lock()
         .lines()
         .map(|_line| _line.ok().unwrap())
         .collect::<Vec<String>>(); // Vector not necessary, see example solution.
     
    // split each line up in a pair of i64 numbers to handle the size
     for line_index in 0..lines.len() {
         let number_pair = lines[line_index as usize]
            .split(' ')
            .map(|number| number.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
            
        // get the difference between the pair
        let difference = number_pair[0] - number_pair[1];
        
        // print the absolute value of the difference to std.output
        println!("{}", difference.abs());
     }
 
     eprintln!("Kattis skips this comment!");
 }