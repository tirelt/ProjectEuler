use std::collections::HashMap;

fn write_number(n: u32,map: &HashMap<u32,&str>)->String{
    assert!(n<=1000,"Not implemented for >=1000");
    if n == 1000{
        let res = String::from("one thousand");
        return res; 
    }
    let number_of_hundred = n/100;
    let rest_hundred = n % 100;
    let mut res = String::new();
    if number_of_hundred > 0{
        let add = format!("{} hundred",map.get(&number_of_hundred).unwrap());
        res.push_str(&add);
        if rest_hundred > 0 {
            res.push_str(" and ");
        } else {
            return res;
        }
    }
    if let Some(name) = map.get(&rest_hundred){
        res.push_str(name);
        return res 
    }
    let number_of_ten = rest_hundred / 10;
    let rest_ten = rest_hundred % 10;
    let ten = number_of_ten * 10;
    if ten > 0{
        res.push_str(map.get(&ten).unwrap());
        res.push(' ');
    }
    res.push_str(map.get(&rest_ten).unwrap());
    res
}
fn main() {
    let map: HashMap<u32, &str> = HashMap::from([
        (0, "zero"),
        (1, "one"),
        (2, "two"),
        (3, "three"),
        (4, "four"),
        (5, "five"),
        (6, "six"),
        (7, "seven"),
        (8, "eight"),
        (9, "nine"),
        (10, "ten"),
        (11, "eleven"),
        (12, "twelve"),
        (13, "thirteen"),
        (14, "fourteen"),
        (15, "fifteen"),
        (16, "sixteen"),
        (17, "seventeen"),
        (18, "eighteen"),
        (19, "nineteen"),
        (20, "twenty"),
        (30, "thirty"),
        (40, "forty"),
        (50, "fifty"),
        (60, "sixty"),
        (70, "seventy"),
        (80, "eighty"),
        (90, "ninety"),
    ]);
    //println!("test: {}",write_number(516,&map));
    let res: u32 = (1..=1000)
                    .map(|x| write_number(x,&map)
                        .chars()
                        .filter(|x| !x.is_whitespace())
                        .collect())
                    .map(|x: String| x.len() as u32)
                    .sum();
    println!("Result: {res}");
}
