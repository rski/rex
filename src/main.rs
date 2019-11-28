use std::fs;
use std::path::PathBuf;
use std::process;

#[derive(Debug, PartialEq)]
struct Monitor<'a> {
    name: String,
    connected: bool,
    primary: bool,
    highest_res: Option<&'a str>,
}

fn main() {
    let output = process::Command::new("xrandr")
        .output()
        .expect("could not run xrandr");
    let current_setup = std::str::from_utf8(&output.stdout).expect("could not get output");
    let displays = parse_xrandr(current_setup);
    println!("setup: {:?}", displays);
}

fn parse_xrandr(xrandr: &str) -> Vec<Monitor> {
    let mut mons: Vec<Monitor> = Vec::new();
    let mut curr_max_res: Option<&str> = None;
    for line in xrandr.lines().rev() {
        if line.starts_with(" ") {
            curr_max_res = line.split_ascii_whitespace().nth(0);
            continue;
        }
        if line.starts_with("Screen ") {
            continue;
        }
        let v: Vec<&str> = line.split_ascii_whitespace().collect();
        let d = Monitor {
            name: v[0].to_owned(),
            connected: v[1] == "connected",
            primary: v[2] == "primary",
            highest_res: curr_max_res,
        };
        mons.push(d);
    }
    mons
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("testdata/laptop_simple.txt");
        let contents =
            fs::read_to_string(d.to_str().unwrap()).expect("Something went wrong reading the file");
        let displays = parse_xrandr(&contents);
        let expected_displays = vec![
            Monitor {
                name: String::from("DP1"),
                connected: false,
                primary: false,
                highest_res: None,
            },
            Monitor {
                name: String::from("eDP1"),
                connected: true,
                primary: false,
                highest_res: Some("1920x1080"),
            },
        ];
        assert_eq! {displays, expected_displays};
    }
}
