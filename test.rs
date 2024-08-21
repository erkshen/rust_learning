// learning basic rust functions

fn main() {

    //hello_world();

    //ranges();

    
    // adding from 0 to 4
    // note that 'let' creates an immutable variable, 
    // so we need to use 'mut' as well to specify mutable
    /*
    let mut total = 0;
    for i in 0..5 {
        total += i;
    }
    println!("sum = {}", total);
    */

    // we have to explicitly cast types in rust
    // e.g. 0.0 cannot add with an integer value
    // we have to use 0.0 + 1 as f64
    println!("{}", 0.0 + 1 as f64)
}

fn hello_world() {
    // declaring variables is done with 'let'
    let answer = 42;

    // test case assertion
    assert_eq!(answer, 41);

    // prints need an !
    println!("Hello {}", answer);
}

fn ranges() {
    // ranges are done with x..y to get x to y-1
    for i in 0..5 {
        if i % 2 == 0 {
            println!("even {}", i);
        } else {
            println!("odd {}", i);
        }
    }
    
    // we can also write:
    for i in 0..5 {
        let even_odd = if i % 2 == 0 {"even"} else {"odd"};
        println!("{} {}", even_odd, i);
    }
}

