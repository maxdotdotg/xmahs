// 12 days of xmahs
//
// loop incrementing for days
// match on days
// loop decrementing gifts

fn main() {
    println!("days of xmas!");
    let mut days = 1;
    while days < 13 {
        println!("on day {} of xmas, somebody gave to me {}", days, gifts(days));
        days += 1;
    }
}

fn gifts(x: i32) -> &'static str {
    let gift = match x {
        12 => "12 drummers drumming,",
        11 => "11 pipers piping,",
        10 => "10 lords a-leaping,",
        9 => "9 ladies dancing,",
        8 => "8 maids a-milking,",
        7 => "7 swans a-swimming,",
        6 => "6 geese a-laying,",
        5 => "FIVE! GOLDEN! RINGS!",
        4 => "4 calling birds,",
        3 => "3 french hens,",
        2 => "2 turtle doves,",
        _ => "a partridge in a pear tree",
    };
    return gift;
}
