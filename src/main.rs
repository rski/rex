use std::process;

#[derive(Debug)]
struct Monitor {
    name: String,
    connected: bool,
    primary: bool,
    highest_res: Option<String>,
}

fn main() {
    let mut mons: Vec<Monitor> = Vec::new();

    let output = process::Command::new("xrandr")
        .output()
        .expect("could not run xrandr");
    let current_setup = std::str::from_utf8(&output.stdout).expect("could not get output");
    let mut curr_max_res: Option<String> = None;
    for line in current_setup.lines().rev() {
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
            highest_res: curr_max_res.clone(),
        };
        mons.push(d);
    }
    println!("setup: {:?}", mons);
}

fn get_res(line: &str) -> Option<String> {
    println!("{:?}", line);
    None
}
