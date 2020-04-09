use english_numbers::Formatting;

const GIFTS: [&str; 12] = [
    "partridge in a pear tree",
    "turtle doves",
    "French hens",
    "calling birds",
    "gold rings",
    "geese a-laying",
    "swans a-swimming",
    "maids a-milking",
    "ladies dancing",
    "lord a-leaping",
    "pipers piping",
    "drummers drumming"
];

fn main() {
    for day in 1..=12 {
        println!("On the {} day of Christmas my true love sent to me",
            ordinal::Ordinal(day));

        if day == 1 {
            println!("A {}.", GIFTS[0]);
        } else {
            for (i, gift) in GIFTS[1..day].iter().enumerate().rev() {
                println!("{} {},",
                    english_numbers::convert(i as i64 + 2, Formatting::all()),
                    gift
                );
            }
            println!("And a {}.", GIFTS[0]);
        }
        println!("");
    }
}
