use std::process::Command;
use crate::batteries::Urgency;

pub fn send_notif(level: i32, urgency : &Urgency ) {
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
    Command::new("brightnessctl")
        .arg("s")
        .arg(if dim { "10%" } else { "90%" })
        .spawn()
        .expect("Failed to change brightness");
}
