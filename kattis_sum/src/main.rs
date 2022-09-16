/***
 * Template to a Kattis solution.
 * See: https://open.kattis.com/help/rust
 * Author: Viola Söderlund <violaso@kth.se>
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

    // Select number characters seperated with a space from input line 1 (or 2, you know what I mean)
    let number_list_len = lines[0].parse::<u32>();
        //.unwrap()
        //.collect::<Vec<u32>>();

    let number_list = lines[1]
        .split(' ')
        .map(|number| number.parse::<u32>())
        .collect::<Vec<u32>>();

    number_list.sort();
     
    let mut total = 0;

    if number_list_len % 2 > 0 {
        // do the following if the list length is odd
        let half_list_len = ((number_list_len + 1) / 2) - 1;

        for current_index in half_list_len..number_list_len {
            total += number_list[current_index] //.parse::<u32>().unwrap();
        }
     }
     else {
        // otherwise do this if the list length is even
        let half_list_len = (number_list_len / 2) - 1;

        for current_index in half_list_len..number_list_len {
            total += number_list[current_index] //.parse::<u32>().unwrap();
        }
    }

    println!("{}", total);
 
    eprintln!("Kattis skips this comment!");
    //println!("Print to standard output.");
}




/*


/***
 * Template to a Kattis solution.
 * See: https://open.kattis.com/help/rust
 * Author: Viola SÃÂÃÂ¶derlund <violaso@kth.se>
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

    // Select number characters seperated with a space from input line 1 (or 2, you know what I mean)
    let number_list_len = lines[0].parse::<u32>().unwrap();
        //.collect::<Vec<u32>>();

    let number_list = lines[1]
        .split(' ')
        .map(|number| number.parse::<u32>())
        .collect::<Vec<u32>>();

    number_list.sort();
     
    let mut total = 0;

    if number_list_len % 2 != 0 {
        // do the following if the list length is odd
        let half_list_len = ((number_list_len + 1) / 2) - 1;

        for current_index in half_list_len..number_list_len {
            total += number_list[current_index] //.parse::<u32>().unwrap();
        }
     }
     else {
        // otherwise do this if the list length is even
        let half_list_len = (number_list_len / 2) - 1;

        for current_index in half_list_len..number_list_len {
            total += number_list[current_index] //.parse::<u32>().unwrap();
        }
    }

    println!("{}", total);
 
    eprintln!("Kattis skips this comment!");
    //println!("Print to standard output.");
}

*/


/* TVÅ ERRORS


/***
 * Template to a Kattis solution.
 * See: https://open.kattis.com/help/rust
 * Author: Viola SÃ¶derlund <violaso@kth.se>
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

    // Select number characters seperated with a space from input line 1 (or 2, you know what I mean)
    let number_list_len = lines[0].parse::<u32>().unwrap();

    let number_list = lines[1]
        .split(' ')
        .map(|num| num.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    number_list.sort();
     
    let mut total = 0;

    if number_list_len % 2 != 0 {
        // do the following if the list length is odd
        let half_list_len = ((number_list_len + 1) / 2) - 1;

        for current_index in half_list_len..number_list_len {
            total += number_list[current_index];
        }
     }
     else {
        // otherwise do this if the list length is even
        let half_list_len = (number_list_len / 2) - 1;

        for current_index in half_list_len..number_list_len {
            total += number_list[current_index];
        }
    }

    println!("{}", total);
 
    eprintln!("Kattis skips this comment!");
    //println!("Print to standard output.");
}

*/


/* /***
 * Template to a Kattis solution.
 * See: https://open.kattis.com/help/rust
 * Author: Viola SÃÂÃÂÃÂÃÂÃÂÃÂÃÂÃÂÃÂÃÂÃÂÃÂÃÂÃÂÃÂÃÂ¶derlund <violaso@kth.se>
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

    // Select number characters seperated with a space from input line 1 (or 2, you know what I mean)
    let number_list_len = lines[0].parse::<u32>().unwrap();

    let mut number_list = lines[1]
        .split(' ')
        .map(|num| num.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    number_list.sort();
     
    let mut total = 0;

    if number_list_len % 2 != 0 {
        // do the following if the list length is odd
        let half_list_len = ((number_list_len + 1) / 2) - 1;

        for current_index in half_list_len..number_list_len {
            total += number_list[current_index as usize];
        }
     }
     else {
        // otherwise do this if the list length is even
        let half_list_len = (number_list_len / 2) - 1;

        for current_index in half_list_len..number_list_len {
            total += number_list[current_index as usize];
        }
    }

    println!("{}", total);
 
    eprintln!("Kattis skips this comment!");
    //println!("Print to standard output.");
} success!"#!" */








/*


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

    // Select number characters seperated with a space from input line 1 (or 2, you know what I mean)
    let number_list_len = lines[0].parse::<u32>().unwrap();

    let mut number_list = lines[1]
        .split(' ')
        .map(|num| num.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    number_list.sort();
     
    let mut total = 0;

    if number_list_len % 2 > 0 {
        // do the following if the list length is odd
        let half_list_len = ((number_list_len + 1) / 2) - 1;

        for current_index in half_list_len..number_list_len {
            total += number_list[current_index as usize];
        }
     }
     else {
        // otherwise do this if the list length is even
        let half_list_len = (number_list_len / 2) - 1;

        for current_index in half_list_len..number_list_len {
            total += number_list[current_index as usize];
        }
    }

    println!("{}", total);
 
    eprintln!("Kattis skips this comment!");
    //println!("Print to standard output.");
}

*/