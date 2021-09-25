use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env::args;
use std::fs;
use std::path::Path;
use std::process;
use std::time;
use toml;
use xdg::BaseDirectories;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct Monitor {
    name: String,
    connected: bool,
    primary: bool,
    on: bool,
    highest_res: Option<String>,
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
    sleep_time: Option<time::Duration>,
    setup: Vec<Setup>,
}

fn main() {
    let args: Vec<String> = args().collect();
    let dry_run = args.contains(&String::from("--dry-run"));
    if args.len() > 2 || (args.len() == 2 && !dry_run) {
        panic!("unexpected arguments {:?}", args);
    }
    let config = get_config();
    let sleep_time = config.sleep_time.unwrap_or(time::Duration::from_secs(1));
    let mut prev_setup: Box<HashMap<String, Monitor>> = Box::from(HashMap::new());
    let mut logged: bool = false;
    loop {
        let output = process::Command::new("xrandr")
            .output()
            .expect("could not run xrandr");
        let current_setup = std::str::from_utf8(&output.stdout).expect("could not get output");
        let displays = Box::from(parse_xrandr(current_setup));

        if displays == prev_setup {
            if !logged {
                println!("Steady state");
            };
            logged = true;
            std::thread::sleep(sleep_time);
            continue;
        }

        logged = false;
        println!("{:?}", &displays);
        let mut proc = select_command(&displays, &config);
        if args.contains(&String::from("--dry-run")) {
            println!("would have executed {:?}", proc);
            return;
        } else {
            match proc.output() {
                Err(e) => println! {"{}", e},
                _ => {}
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
        preds.iter().for_each(|pred| {
            let ok = match displays.get(pred.name.as_str()) {
                Some(display) => match &display.highest_res {
                    Some(res) => res.eq(pred.res.as_str()) && display.connected == pred.connected,
                    None => false,
                },
                _ => false,
            };
            if !ok {
                false;
            }
        });
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
    m
}

fn get_config() -> Config {
    let p = Path::new("rex/config.toml");
    let dirs = BaseDirectories::new().unwrap();
    let cfg = dirs.find_config_file(p).unwrap();
    println!("using config {}", cfg.to_str().unwrap());
    let contents = fs::read_to_string(cfg).expect("Something went wrong reading the file");
    toml::from_str::<Config>(contents.as_str()).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::{fs::read_to_string, path::PathBuf};

    #[test]
    fn test_parse() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("testdata/laptop_simple.txt");
        let contents =
            read_to_string(d.to_str().unwrap()).expect("Something went wrong reading the file");
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
                highest_res: Some(String::from("1920x1080")),
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
            read_to_string(d.to_str().unwrap()).expect("Something went wrong reading the file");
        let c = toml::from_str::<Config>(contents.as_str()).unwrap();
        assert_eq!(c.sleep_time, Some(time::Duration::from_secs(2)));
    }
}
