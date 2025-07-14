use ordinal::Ordinal;

fn main() {
    let days = [
        "A Partridge in a Pear Tree",
        "Two Turtle Doves,",
        "Three French Hens,",
        "Four Calling Birds,",
        "Five Golden Rings,",
        "Six Geese a-laying,",
        "Seven Swans a-swimming,",
        "Eight Maids a-milking,",
        "Nine Ladies dancing,",
        "Ten Lords a-leaping,",
        "Eleven Pipers piping,",
        "Twelve Drummers Drumming,",
    ];

    for (i, d) in days.iter().enumerate() {
        println!("\nOn the {} day of Christmas", Ordinal(i + 1));

        println!("My true love gave to me");

        println!("{d}");

        for n in (0..i).rev() {
            if n == 0 {
                println!("and {}", days[n]);
                continue;
            }
            println!("{}", days[n]);
        }
    }
}
