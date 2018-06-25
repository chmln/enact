#![feature(nll)]
use std::process::Command;

fn run_command(command: &mut Command) -> String {
    String::from_utf8_lossy(&command.output().unwrap().stdout).into_owned()
}

#[derive(Debug)]
struct Monitor {
    width: i32,
    height: i32,
    name: String,
}

impl Default for Monitor {
    fn default() -> Self {
        Monitor {
            width: 1920,
            height: 1080,
            name: "eDP1".to_string(),
        }
    }
}

impl Monitor {
    fn resolution(&self) -> String {
        format!("{}x{}", self.width, self.height)
    }
}

fn get_secondary_monitor() -> Option<Monitor> {
    let monitors = run_command(Command::new("xrandr").arg("-q"));
    let mut lines = monitors.lines();

    while let Some(line) = lines.next() {
        if line.contains(" connected") && line.contains("HDMI") {
            let monitor_name = line.split_whitespace().nth(0)?;

            if let Some(resolution_line) = lines.next() {
                let mut resolution =
                    resolution_line.split_whitespace().nth(0)?.split("x");

                return Some(Monitor {
                    width: resolution.nth(0)?.parse().expect("ParseError"),
                    height: resolution.nth(0)?.parse().expect("ParseError"),
                    name: monitor_name.to_string(),
                });
            }
        }
    }
    return None;
}

fn main() {
    let primary_monitor = Monitor::default();
    let mut cmd = Command::new("xrandr");

    if let Some(secondary_monitor) = get_secondary_monitor() {
        let xrandr_args = format!(
            "--output {} --mode {} --pos 0x0 --output {} --primary --mode {} \
             --pos {}x{}",
            secondary_monitor.name,
            secondary_monitor.resolution(),
            primary_monitor.name,
            primary_monitor.resolution(),
            (secondary_monitor.width - primary_monitor.width) / 2,
            secondary_monitor.height
        );

        cmd.args(xrandr_args.split_whitespace());
    }
    else {
        cmd.args(&["--output", &primary_monitor.name])
            .args(&["--mode", &primary_monitor.resolution()])
            .args(&["--pos", "0x0"])
            .arg("--primary");
    }

    println!(
        "{}",
        match cmd.spawn() {
            Ok(_res) => "Ok",
            Err(_e) => "Error",
        }
    )

    // println!("{:?}", randr);
}
