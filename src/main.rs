use tokio_udev::{AsyncMonitorSocket, MonitorBuilder, Event};
use tokio_stream::StreamExt;
use tokio::time::{sleep, Duration};
use std::io;

use batteries::{ChargingState, Urgency, get_battery_percentage, get_battery_state};
use subprocessing::{send_notif, change_brightness};

mod batteries;
mod subprocessing;

const LOW_BATTERY_THRESHOLD: i32 = 20;
const FULL_BATTERY_THRESHOLD: i32 = 98;

#[tokio::main(flavor = "current_thread")]
async fn main() -> io::Result<()> {
    let (mut dimmed, mut notified) = (false, false);

    let monitor = MonitorBuilder::new()?
        .match_subsystem("power_supply")?
        .listen()?;

    let mut socket = AsyncMonitorSocket::try_from(monitor)?;

    println!("Watching Batteries");

    loop {
        let res = is_event(&mut socket).await;

        if let Ok(_) = res {
            let _ = runner(&mut dimmed, &mut notified);
        }
    }
}

fn runner(dimmed: &mut bool, notified: &mut bool) -> Result<(), Box<dyn std::error::Error>> {
    let battery_level = get_battery_percentage()?;
    let state = get_battery_state()?;
    let urgency = if battery_level <= LOW_BATTERY_THRESHOLD { Urgency::Low } else if battery_level >= FULL_BATTERY_THRESHOLD {Urgency::Full} else {Urgency::Normal};

    println!("Battery {}", battery_level);
    println!("state {}", state.as_str());
    println!("urgency {}", urgency.as_str());


    if urgency == Urgency::Low && state == ChargingState::Discharging { // Low and not charging
        if !*notified {
            send_notif(battery_level, &urgency);
            *notified = true;
        };

        if !*dimmed {
            change_brightness(true);
            *dimmed = true;
        };


    } else if urgency == Urgency::Full && state == ChargingState::Charging {
        println!("hit high");
        if !*notified {
            send_notif(battery_level, &urgency);
            *notified = true;
            println!("hit notif");
        };
    }

    if urgency != Urgency::Low && *dimmed {
        change_brightness(false);
        *dimmed = false;
    }

    if urgency != Urgency::Low {
        *notified = false
    }

    Ok(())

}

async fn is_event(socket: &mut AsyncMonitorSocket) -> io::Result<Event> {
    loop {
        let _ = match socket.next().await {
            Some(res) => {
                return res
            },
            None => {
                sleep(Duration::from_millis(100)).await;
                continue;
            }
        };

    }

}

