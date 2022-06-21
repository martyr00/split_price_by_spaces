fn main() {
    //test split_price_by_spaces
    println!("{}", split_price_by_spaces(10));
    println!("{}", split_price_by_spaces(100));
    println!("{}", split_price_by_spaces(1000));
    println!("{}", split_price_by_spaces(10000));
    println!("{}", split_price_by_spaces(100000));
    println!("{}", split_price_by_spaces(1000000));
    println!("{}", split_price_by_spaces(10000000));
}

//adds a space after every 3 elements
fn split_price_by_spaces(numbers: usize) -> String {
    let numb_str = numbers.to_string();
    let rev_numb = numb_str.chars().rev().collect::<String>();
    let mut vec_numbers: Vec<&str> = rev_numb.split("").collect();

    let mut res_vec = Vec::new();
    for x in 0..vec_numbers.len() {
        if (x) % 3 == 0 {
            res_vec.push(vec_numbers[x].to_string() + &*" ".to_string())
        } else {
            res_vec.push(vec_numbers[x].to_string())
        };
    }

    let mut res = "".to_string();

    for i in res_vec {
        res += &*i
    }
    res.trim().chars().rev().collect::<String>()
}
