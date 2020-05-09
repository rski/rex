use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::process;
use std::time;
use structopt::StructOpt;
use toml;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct Monitor {
    name: String,
    connected: bool,
    primary: bool,
    on: bool,
    highest_res: Option<String>,
}

#[derive(StructOpt)]
struct Cli {
    #[structopt(short, long)]
    dry_run: bool,
}

#[derive(Debug, Deserialize)]
struct Predicate {
    name: String,
    connected: bool,
    res: String,
}

#[derive(Debug, Deserialize)]
struct Setup {
    exec: String,
    predicates: Option<Vec<Predicate>>,
}

#[derive(Debug, Deserialize)]
struct Config {
    setup: Vec<Setup>,
}

fn main() {
    let args = Cli::from_args();
    let sleep_time = time::Duration::from_secs(1);
    let config = get_config();
    let mut prev_setup: Box<HashMap<String, Monitor>> = Box::from(HashMap::new());
    loop {
        let output = process::Command::new("xrandr")
            .output()
            .expect("could not run xrandr");
        let current_setup = std::str::from_utf8(&output.stdout).expect("could not get output");
        let displays = Box::from(parse_xrandr(current_setup));

        if displays == prev_setup {
            println!("Steady state");
            std::thread::sleep(sleep_time);
            continue;
        }

        let mut proc = select_command(&displays, &config);
        if args.dry_run {
            println!("would have executed {:?}", proc);
            return;
        } else {
            match proc.output() {
                Ok(_) => (),
                Err(e) => println! {"{}", e},
            }
            proc.status().expect("failed to run");
            prev_setup = Box::from(parse_xrandr(current_setup));
        }

        std::thread::sleep(sleep_time)
    }
}

fn select_command(displays: &HashMap<String, Monitor>, cfg: &Config) -> Box<process::Command> {
    let mut proc = Box::new(process::Command::new("xrandr"));
    for setup in cfg.setup.iter() {
        if predicate_matches(&setup.predicates, displays) {
            for i in setup.exec.split_ascii_whitespace() {
                proc.arg(i);
            }
            return proc;
        }
    }
    proc.arg("--auto");
    proc
}

fn predicate_matches(
    predicates: &Option<Vec<Predicate>>,
    displays: &HashMap<String, Monitor>,
) -> bool {
    predicates.as_ref().map_or(true, |preds| {
        for pred in preds.iter() {
            if let Some(display) = displays.get(pred.name.as_str()) {
                if let Some(res) = display.highest_res.as_ref() {
                    if !res.eq(pred.res.as_str()) {
                        return false;
                    }
                    if !display.connected == pred.connected {
                        return false;
                    }
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    })
}

fn parse_xrandr(xrandr: &str) -> HashMap<String, Monitor> {
    let mut mons: HashMap<String, Monitor> = HashMap::new();
    let mut max_res: Option<String> = None;
    for line in xrandr.lines().rev() {
        if line.starts_with(' ') {
            max_res = Some(line.split_ascii_whitespace().next().unwrap().to_owned());
            continue;
        }
        if line.starts_with("Screen ") {
            continue;
        }
        let d = parse_monitor(line, max_res.clone());
        mons.insert(d.name.clone(), d);
    }
    mons
}

// a display looks like
// <name> (dis)connected [primary] [resolution+offset] (normal left inverted right x axis y axis) 527mm x 296mm
// a display that is on will have a resolution.
// Oddly enough, a display that is off can still be primary.
fn parse_monitor<'a>(line: &'a str, max_res: Option<String>) -> Monitor {
    let v: Vec<&str> = line.split_ascii_whitespace().collect();
    if v.len() < 3 {
        panic!("input was long enough, cannot parse {:?}", v)
    }
    let prim = v[2] == "primary";
    let res_offset = if prim { 3 } else { 2 };
    let m = Monitor {
        name: v[0].to_owned(),
        connected: v[1] == "connected",
        primary: prim,
        highest_res: max_res,
        on: !v[res_offset].starts_with('('), // crude approximation
    };

    println!("{:?}", m);
    m
}

fn get_config() -> Config {
    let home = "/home/rski/.config/rex/config.toml";
    println!("{:?}", home);
    let contents = fs::read_to_string(home).expect("Something went wrong reading the file");

    toml::from_str::<Config>(contents.as_str()).unwrap()
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
                on: false,
            },
        );
        expected_displays_map.insert(
            String::from("eDP1"),
            Monitor {
                name: String::from("eDP1"),
                connected: true,
                primary: false,
                highest_res: Some("1920x1080"),
                on: true,
            },
        );
        assert_eq! {displays, expected_displays_map};
    }
    #[test]

    fn test_parse_predicate() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("testdata/config.toml");
        let contents =
            fs::read_to_string(d.to_str().unwrap()).expect("Something went wrong reading the file");
        toml::from_str::<Config>(contents.as_str()).unwrap();
    }
}
