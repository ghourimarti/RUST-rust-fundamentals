
fn split_string(s: String, delimiter: char, field: usize) -> String 
{
    let parts: Vec<&str> = s.split(delimiter).collect();

    // This will not compile!
    let result = parts.get(field);
    result.expect("Index out of bounds").to_string()


    
}



// write a function that returns the nth character of a string
fn nth_char(s: String, n: usize) -> char {
    s.chars().nth(n).unwrap()
}


// write a decorator function like flask or fastapi in python
fn main() 
{
    println!("\n<=====================================>\n");

    let chunk = split_string("hello,world".to_string(), ',', 1);
    println!("Split string: {}", chunk);

    let char = nth_char("hello".to_string(), 1);
    println!("Nth character: {}", char);
}
