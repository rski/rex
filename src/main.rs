use std::collections::HashMap;
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
    let sleep_time = time::Duration::from_secs(10);
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

fn displays_to_command(displays: HashMap<String, Monitor>) -> Box<process::Command> {
    let mut proc = process::Command::new("xrandr");
    let dp = displays.get("DP2-2-8");
    if let Some(d) = dp {
        if d.connected {
            for i in String::from("--output eDP1 --off --output DP2-1 --primary --mode 2560x1440 --pos 0x0 --rotate left --crtc 0 --output DP2-2-8 --mode 2560x1440 --pos 1440x560 --rotate normal --crtc 1").split_ascii_whitespace() {
                proc.arg(i);
            }
            return Box::new(proc);
        }
    }
    proc.arg("--auto");
    return Box::new(proc);
}

fn parse_xrandr(xrandr: &str) -> HashMap<String, Monitor> {
    let mut mons: HashMap<String, Monitor> = HashMap::new();
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
        mons.insert(d.name.clone(), d);
    }
    mons
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn test_parse() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("testdata/laptop_simple.txt");
        let contents =
            fs::read_to_string(d.to_str().unwrap()).expect("Something went wrong reading the file");
        let displays = parse_xrandr(&contents);
        let mut expected_displays_map: HashMap<String, Monitor> = HashMap::new();
        expected_displays_map.insert(
            String::from("DP1"),
            Monitor {
                name: String::from("DP1"),
                connected: false,
                primary: false,
                highest_res: None,
            },
        );
        expected_displays_map.insert(
            String::from("eDP1"),
            Monitor {
                name: String::from("eDP1"),
                connected: true,
                primary: false,
                highest_res: Some("1920x1080"),
            },
        );
        assert_eq! {displays, expected_displays_map};
    }
}
