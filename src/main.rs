// 12 days of xmahs
//
// loop incrementing for days
// match on days
// loop decrementing gifts

fn main() {
    println!("days of xmas!");
    let mut days = 1;
    while days < 13 {
        println!("on day {} of xmas, somebody gave to me {:?}", days, gifts(days));
        days += 1;
    }
}

fn gifts(x: i32) {
    let gift = match x {
        12 => println!("12 drummers drumming,"),
        11 => println!("11 pipers piping,"),
        10 => println!("10 lords a-leaping,"),
        9 => println!("9 ladies dancing,"),
        8 => println!("8 maids a-milking,"),
        7 => println!("7 swans a-swimming,"),
        6 => println!("6 geese a-laying,"),
        5 => println!("FIVE! GOLDEN! RINGS!"),
        4 => println!("4 calling birds,"),
        3 => println!("3 french hens,"),
        2 => println!("2 turtle doves,"),
        _ => println!("a partridge in a pear tree"),
    };
    return gift
}
