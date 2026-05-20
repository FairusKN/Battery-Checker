use std::process::Command;
use crate::batteries::Urgency;

const CHANGE_BRIGHTNESS_LEVEL: i8 = 80; // Use this to change just *-1 to dim it

pub fn send_notif(level: i32, urgency : &Urgency ) {
    println!("level {}", level);
    println!("Urgency in send {}", urgency.as_str());

    Command::new("notify-send")
        // Urgency
        .arg("-u")
        .arg(urgency.as_str())
        //Expire
        .arg("-t")
        .arg("5000")
        //Level
        .arg(&format!("🔋 {} Battery {}%)",
            if *urgency == Urgency::Low {"Low"} else {"High"},
            level))
        .arg(if *urgency == Urgency::Low {
            "Plug in your charger!"
        } else {
            "Unplug your charger!"
        })
        .spawn()
        .expect("Failed to send notification");
}

pub fn change_brightness(dim: bool) {
    // Sub Process will make smth like  `swayosd-client --brightness=+80`
    Command::new("swayosd-client")
        .arg("--brightness")
        .arg(format!(
            "{}{}",
            if dim { "-" } else { "+" },
            CHANGE_BRIGHTNESS_LEVEL
        ))
        .spawn()
        .expect("Failed to change brightness level");
}
