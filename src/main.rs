use std::process;

#[derive(Debug)]
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
            curr_max_res = get_res(line);
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

fn get_res(line: &str) -> Option<&str> {
    let mut parts = line.split_ascii_whitespace();
    parts.nth(0)
}
