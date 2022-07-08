use anyhow::Result;
use std::process::Command;

#[derive(Debug, PartialEq, Eq)]
struct Monitor {
    width: i32,
    height: i32,
    name: String,
}

impl Monitor {
    fn resolution(&self) -> String {
        format!("{}x{}", self.width, self.height)
    }

    fn from_line(line: &str, next: &str) -> Option<Monitor> {
        let name = line.split_whitespace().nth(0)?.to_string();
        let mut resolution = next.split_whitespace().nth(0)?.split('x');

        Some(Monitor {
            name,
            width: resolution.nth(0)?.parse().expect("ParseError"),
            height: resolution.nth(0)?.parse().expect("ParseError"),
        })
    }
}

#[derive(PartialEq, Eq)]
struct Xrandr {
    monitors: Vec<Monitor>,
    placement: Placement,
    new_primary: usize,
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Placement {
    Top,
    Bottom,
    Left,
    Right,
    Clone,
}

impl Placement {
    fn invert(&self) -> Self {
        match self {
            Self::Top => Self::Bottom,
            Self::Bottom => Self::Top,
            Self::Right => Self::Left,
            Self::Left => Self::Right,
            Self::Clone => Self::Clone,
        }
    }
}

impl std::str::FromStr for Placement {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Placement> {
        match s.to_lowercase().as_ref() {
            "top" => Ok(Self::Top),
            "bottom" => Ok(Self::Bottom),
            "left" => Ok(Self::Left),
            "right" => Ok(Self::Right),
            "clone" => Ok(Self::Clone),
            _ => anyhow::bail!("Invalid placement"),
        }
    }
}

pub struct Rect {
    width: i32,
    height: i32,
}

impl Rect {
    fn place(&self, monitor: &Monitor, at: &Placement) -> String {
        let (offset_width, offset_height) = match at {
            Placement::Top => ((self.width - monitor.width) / 2, 0),
            Placement::Bottom => (
                (self.width - monitor.width) / 2,
                self.height - monitor.height,
            ),
            Placement::Left => (0, (self.height - monitor.height) / 2),
            Placement::Right => (
                self.width - monitor.width,
                (self.height - monitor.height) / 2,
            ),
            Placement::Clone => (0,0,),
        };
        format!("{}x{}", offset_width, offset_height)
    }
}

impl Xrandr {
    fn new(placement: Placement, new_primary: usize) -> Result<Self> {
        use itertools::Itertools;

        let monitors = String::from_utf8(
            Command::new("xrandr").arg("-q").output()?.stdout,
        )?
        .lines()
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>();

        let monitors = monitors
            .iter()
            .enumerate()
            .filter(|(i, l)| {
                l.contains(" connected")
                    || (*i > 0 && monitors[*i - 1].contains(" connected"))
            })
            .map(|(_, l)| l)
            .chunks(2)
            .into_iter()
            .filter_map(|mut l| Monitor::from_line(&l.next()?, &l.next()?))
            .collect::<Vec<Monitor>>();

        Ok(Self { monitors, placement, new_primary })
    }

    fn get_rect(&self) -> Rect {
        let (width, height) = match self.placement {
            Placement::Top | Placement::Bottom => (
                self.monitors.iter().map(|m| m.width).max().unwrap(),
                self.monitors.iter().map(|m| m.height).sum::<i32>(),
            ),
            _ => (
                self.monitors.iter().map(|m| m.width).sum::<i32>(),
                self.monitors.iter().map(|m| m.height).max().unwrap(),
            ),
        };
        Rect { width, height }
    }

    fn setup(&self) -> Result<()> {
        let rect = self.get_rect();
        let mut cmd = Command::new("/usr/bin/xrandr");

        for (i, monitor) in self.monitors.iter().enumerate() {
            cmd.args(&["--output", &monitor.name])
                .args(&["--mode", &monitor.resolution()]);

            if i == 0 {
                cmd.args(&["--pos", &rect.place(monitor, &self.placement.invert())]);
            } else if self.placement == Placement::Clone {
                cmd.args(&["--same-as", &self.monitors[0].name]);
            } else {
                cmd.args(&["--pos", &rect.place(monitor, &self.placement)]);
            }

            if self.new_primary == i || self.monitors.len() == 1 {
                cmd.arg("--primary");
            }
        }

        log::info!("Command: {:?}", &cmd);

        if cmd.status()?.success() {
            Ok(())
        } else {
            anyhow::bail!("xrandr failed :(")
        }
    }
}

#[derive(structopt::StructOpt)]
#[structopt(
    name = "enact",
    about = "Easy dual-monitor setup and hotplug support for minimalistic window managers"
)]
struct Enact {
    #[structopt(short, long)]
    /// Automatically setup monitors as they're plugged in
    watch: bool,
    #[structopt(short, long)]
    /// Where to place the second monitor: top, bottom, left, right
    pos: Placement,
    #[structopt(short, long, default_value = "0")]
    /// Which monitor is the new primary
    new_primary: usize,
}

fn main() -> Result<()> {
    flexi_logger::Logger::with_env().start()?;
    use structopt::StructOpt;
    let args = Enact::from_args();

    let mut prev = Xrandr::new(args.pos, args.new_primary)?;

    if args.watch {
        loop {
            let monitors = Xrandr::new(args.pos, args.new_primary)?;
            if prev != monitors {
                log::info!("Applying: {:?}", &monitors.monitors);
                monitors.setup()?;

                prev.monitors
                    .iter()
                    .skip_while(|m| monitors.monitors.contains(m))
                    .for_each(|removed_monitor| {
                        let _ = Command::new("xrandr")
                            .args(&["--output", &removed_monitor.name, "--off"])
                            .status()
                            .map_err(|e| log::error!("{}", e));
                    });

                prev = monitors;
            }

            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    } else {
        prev.setup()?;
    }

    Ok(())
}
