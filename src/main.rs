use std::process::Command;

#[derive(Debug)]
struct Monitor {
    width: i32,
    height: i32,
    name: String,
}

impl Monitor {
    fn resolution(&self) -> String {
        format!("{}x{}", self.width, self.height)
    }

    fn from_line(line: &str, next: Option<&str>) -> Option<Monitor> {
        let name = line.split_whitespace().nth(0)?.to_string();
        let mut resolution = next?.split_whitespace().nth(0)?.split("x");

        return Some(Monitor {
            name,
            width: resolution.nth(0)?.parse().expect("ParseError"),
            height: resolution.nth(0)?.parse().expect("ParseError"),
        });
    }
}

struct Xrandr;

impl Xrandr {
    fn is_monitor_line(str: &str) -> bool {
        str.contains(" connected") 
    }

    fn query() -> String {
        let mut cmd = Command::new("xrandr");
        let output = &cmd.arg("-q").output().unwrap().stdout;
        String::from_utf8_lossy(output).into_owned()
    }

    fn get_monitors() -> Vec<Monitor> {
        let mut monitors = Vec::new();

        let xrandr = Self::query();
        let mut lines = xrandr.lines();

        while let Some(line) = lines.next() {
          if Self::is_monitor_line(line) {
            if let Some(monitor) = Monitor::from_line(line, lines.next()) {
              monitors.push(monitor);
            }
          }
        }
        monitors
    }

    fn setup() -> std::io::Result<()> {
        let monitors = Self::get_monitors();
        if monitors.len() == 0 {
            return Ok(());
        }

        let mut cmd = Command::new("xrandr");
        let primary = &monitors[0];

        let total_width = monitors.iter().fold(-primary.width,|acc, m| acc + m.width);
        let max_height = monitors.iter().fold(0,|acc, m| if m.height > acc { m.height } else { acc });
        let mut non_primary_pos = 0;

        for (i, monitor) in monitors.iter().enumerate() {
            cmd
                .arg("--output").arg(&monitor.name)
                .arg("--mode").arg(&monitor.resolution())
                .arg("--pos");

            if i == 0 {
                cmd
                    .arg(format!("{}x{}", (total_width - primary.width)/2, max_height))
                    .arg("--primary");
            }
            else {
                cmd.arg(format!("{}x{}", non_primary_pos, (monitor.height - max_height)/2));
                non_primary_pos += monitor.width;
            }
        }

        cmd.spawn().map(|_r| ())
    }
}




fn main() -> std::io::Result<()> {
    Xrandr::setup()
}
