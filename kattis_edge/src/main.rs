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
 
     /* add code here ... */
     let number_pair = lines[0]
         .split(' ')
         .map(|number| number.parse::<u32>().unwrap())
         .collect::<Vec<u32>>();
         
     for row in 0..number_pair[0] {
         
         for column in 0..number_pair[1] {

             if (column > 8) && (row > 8) {
                 print!(".");
                 continue;
             }
     
             if column <= row {
                print!("{}", column + 1);
                continue;
             }
             else {
                 print!("{}", row + 1);
                 continue;
             }
         }
         println!("");
     }

 
     eprintln!("Kattis skips this comment!");
     //println!("Print to standard output.");
 }