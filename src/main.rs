use std::process;
use std::time;

#[derive(Debug, PartialEq)]
struct Monitor<'a> {
    name: String,
    connected: bool,
    primary: bool,
    highest_res: Option<&'a str>,
}

fn main() {
    let sleep_time = time::Duration::from_secs(1);
    loop {
        let output = process::Command::new("xrandr")
            .output()
            .expect("could not run xrandr");
        let current_setup = std::str::from_utf8(&output.stdout).expect("could not get output");
        let displays = parse_xrandr(current_setup);
        // println!("setup: {:?}", displays);
        let mut proc = displays_to_command(displays);

        match proc.output() {
            Ok(_) => (),
            Err(e) => println! {"{}", e},
        }
        proc.status().expect("failed to run");

        std::thread::sleep(sleep_time)
    }
}

fn displays_to_command(displays: Vec<Monitor>) -> Box<process::Command> {
    let mut proc = process::Command::new("xrandr");
    for d in displays.iter() {
        if d.name == "DP2-2-8" && d.connected {
            for i in String::from("--output eDP1 --off --output DP2-1 --primary --mode 2560x1440 --pos 0x0 --rotate left --output DP2-2-8 --mode 2560x1440 --pos 1440x560 --rotate normal").split_ascii_whitespace() {
                proc.arg(i);
            }
            return Box::new(proc);
        }
    }
    proc.arg("--auto");
    return Box::new(proc);
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
    use std::fs;
    use std::path::PathBuf;

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
