use std::process;

#[derive(Debug)]
struct Monitor {
    name: String,
    connected: bool,
    primary: bool,
}

fn main() {
    let output = process::Command::new("xrandr")
        .output()
        .expect("could not run xrandr");
    let current_setup = std::str::from_utf8(&output.stdout).expect("could not get output");
    for line in current_setup.lines() {
        if line.starts_with(" ") {
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
        };
        println!("{:?}", d);
    }
    // println!("setup: {}", current_setup);
}
