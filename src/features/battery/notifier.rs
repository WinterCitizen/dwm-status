use io;
use libnotify;
use std::time;

const LEVELS: &[f32] = &[0.02, 0.05, 0.1, 0.15, 0.2];
const CRITICAL: f32 = 0.1;

#[derive(Debug)]
pub struct BatteryNotifier {
    capacity: Option<f32>,
    name: String,
}

impl BatteryNotifier {
    pub fn new(name: &str) -> Self {
        BatteryNotifier {
            capacity: None,
            name: String::from(name),
        }
    }

    pub fn reset(&mut self) {
        self.capacity = None;
    }

    pub fn update(&mut self, capacity: f32, estimation: &time::Duration) {
        for level in LEVELS {
            if level >= &capacity {
                if match self.capacity {
                    Some(value) if level >= &value => false,
                    _ => true,
                } {
                    let minutes = estimation.as_secs() / 60;
                    io::show_notification(
                        &format!("Battery {} under {:.0}%", self.name, level * 100.),
                        &format!("{:02}:{:02} remaining", minutes / 60, minutes % 60),
                        if level <= &CRITICAL {
                            libnotify::Urgency::Critical
                        } else {
                            libnotify::Urgency::Normal
                        },
                    );
                }

                break;
            }
        }

        self.capacity = Some(capacity);
    }
}
