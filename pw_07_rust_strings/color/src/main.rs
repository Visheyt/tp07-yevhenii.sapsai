use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Color",
    about = "Print the RGB values of a color from its name"
)]
struct Arg {
    ///Print RGB values in decimal
    #[structopt(short, long)]
    decimal: bool,

    ///Prints RGB values in percent
    #[structopt(short, long)]
    percent: bool,

    ///Name of the color
    name: String,
}

fn main() {
    let arg = Arg::from_args();
    if get_hex(&arg.name).is_empty() {
        eprintln!("The '{}' color is undefined", &arg.name);
        std::process::exit(1)
    } else {
        print!("{}: {}", &arg.name, get_hex(&arg.name));
        if arg.decimal == true {
            print!("; {}", get_dec(&get_hex(&arg.name)))
        }
        if arg.percent == true {
            print!("; {}", get_per(&get_hex(&arg.name)))
        }
	print!("\n");
    }
}

fn get_hex(name: &str) -> String {
    let contents = std::fs::read_to_string("colors.csv").unwrap();
    let mut words = String::new();
    for line in contents.lines() {
        line.split(';');

        if name == line.split(';').next().unwrap() {
            words = String::from(line.split(';').skip(1).next().unwrap())
        }
    }
    words
}

fn get_dec(hex: &str) -> String {
    format!(
        "RGB({}, {}, {})",
        u8::from_str_radix(&hex[1..3], 16).unwrap(),
        u8::from_str_radix(&hex[3..5], 16).unwrap(),
        u8::from_str_radix(&hex[5..7], 16).unwrap()
    )
}

fn get_per(hex: &str) -> String {
    format!(
        "RGB({}%, {}%, {}%)",
        to_per(u8::from_str_radix(&hex[1..3], 16).unwrap()),
        to_per(u8::from_str_radix(&hex[3..5], 16).unwrap()),
        to_per(u8::from_str_radix(&hex[5..7], 16).unwrap())
    )
}
//comparison fix potom
fn to_per(c: u8) -> u8 {
    let k = c as u16;
    let r = k * 100 / 255;
    r as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_hex() {
        assert_eq!(get_hex("black"), "#000000");
        assert_eq!(get_hex("navy"), "#000080");
        assert_eq!(get_hex("green"), "#008000");
        assert_eq!(get_hex("teal"), "#008080");
        assert_eq!(get_hex("maroon"), "#800000");
        assert_eq!(get_hex("purple"), "#800080");
        assert_eq!(get_hex("olive"), "#808000");
        assert_eq!(get_hex("silver"), "#C0C0C0");
        assert_eq!(get_hex("gray"), "#808080");
        assert_eq!(get_hex("blue"), "#0000FF");
        assert_eq!(get_hex("lime"), "#00FF00");
        assert_eq!(get_hex("aqua"), "#00FFFF");
        assert_eq!(get_hex("red"), "#FF0000");
        assert_eq!(get_hex("fuchsia"), "#FF00FF");
        assert_eq!(get_hex("yellow"), "#FFFF00");
        assert_eq!(get_hex("white"), "#FFFFFF");
        assert_eq!(get_hex("no_color"), "");
    }

    #[test]
    fn test_get_dec() {
        assert_eq!(get_dec("#000000"), "RGB(0, 0, 0)");
        assert_eq!(get_dec("#000080"), "RGB(0, 0, 128)");
        assert_eq!(get_dec("#008000"), "RGB(0, 128, 0)");
        assert_eq!(get_dec("#008080"), "RGB(0, 128, 128)");
        assert_eq!(get_dec("#800000"), "RGB(128, 0, 0)");
        assert_eq!(get_dec("#800080"), "RGB(128, 0, 128)");
        assert_eq!(get_dec("#808000"), "RGB(128, 128, 0)");
        assert_eq!(get_dec("#C0C0C0"), "RGB(192, 192, 192)");
        assert_eq!(get_dec("#808080"), "RGB(128, 128, 128)");
        assert_eq!(get_dec("#0000FF"), "RGB(0, 0, 255)");
        assert_eq!(get_dec("#00FF00"), "RGB(0, 255, 0)");
        assert_eq!(get_dec("#00FFFF"), "RGB(0, 255, 255)");
        assert_eq!(get_dec("#FF0000"), "RGB(255, 0, 0)");
        assert_eq!(get_dec("#FF00FF"), "RGB(255, 0, 255)");
        assert_eq!(get_dec("#FFFF00"), "RGB(255, 255, 0)");
        assert_eq!(get_dec("#FFFFFF"), "RGB(255, 255, 255)");
        assert_eq!(get_dec("#6495ED"), "RGB(100, 149, 237)");
    }

    #[test]
    fn test_get_per() {
        assert_eq!(get_per("#000000"), "RGB(0%, 0%, 0%)");
        assert_eq!(get_per("#000080"), "RGB(0%, 0%, 50%)");
        assert_eq!(get_per("#008000"), "RGB(0%, 50%, 0%)");
        assert_eq!(get_per("#008080"), "RGB(0%, 50%, 50%)");
        assert_eq!(get_per("#800000"), "RGB(50%, 0%, 0%)");
        assert_eq!(get_per("#800080"), "RGB(50%, 0%, 50%)");
        assert_eq!(get_per("#808000"), "RGB(50%, 50%, 0%)");
        assert_eq!(get_per("#C0C0C0"), "RGB(75%, 75%, 75%)");
        assert_eq!(get_per("#808080"), "RGB(50%, 50%, 50%)");
        assert_eq!(get_per("#0000FF"), "RGB(0%, 0%, 100%)");
        assert_eq!(get_per("#00FF00"), "RGB(0%, 100%, 0%)");
        assert_eq!(get_per("#00FFFF"), "RGB(0%, 100%, 100%)");
        assert_eq!(get_per("#FF0000"), "RGB(100%, 0%, 0%)");
        assert_eq!(get_per("#FF00FF"), "RGB(100%, 0%, 100%)");
        assert_eq!(get_per("#FFFF00"), "RGB(100%, 100%, 0%)");
        assert_eq!(get_per("#FFFFFF"), "RGB(100%, 100%, 100%)");
        assert_eq!(get_per("#6495ED"), "RGB(39%, 58%, 92%)");
    }

    #[test]
    fn test_to_per() {
        assert_eq!(to_per(0), 0);
        assert_eq!(to_per(50), 19);
        assert_eq!(to_per(64), 25);
        assert_eq!(to_per(128), 50);
        assert_eq!(to_per(192), 75);
        assert_eq!(to_per(255), 100);
    }
}
