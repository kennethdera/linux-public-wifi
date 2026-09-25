slint::include_modules!();

use std::process::Command;
use shared::{create_voucher, get_authorized_macs};
use std::fs;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak();
    ui.on_toggle_hotspot(move || {
        let ui = ui_handle.unwrap();
        let status = Command::new("sudo")
            .args(["systemctl", "is-active", "linux-public-wifi.service"])
            .output();

        if let Ok(out) = status {
            if String::from_utf8_lossy(&out.stdout).trim() == "active" {
                Command::new("sudo").args(["systemctl", "stop", "linux-public-wifi.service"]).output().ok();
                ui.set_status_text("Status: Stopped".into());
                ui.set_status_color("red".into());
            } else {
                Command::new("sudo").args(["systemctl", "start", "linux-public-wifi.service"]).output().ok();
                ui.set_status_text("Status: Running".into());
                ui.set_status_color("green".into());
            }
        }
    });

    let ui_handle_s = ui.as_weak();
    ui.on_setup_system(move || {
        let _ui = ui_handle_s.unwrap();
        
        // 1. Copy service file
        let src = "/home/caesar/Documents/Projects/linux-public-wifi/linux-public-wifi.service";
        let dst = "/etc/systemd/system/linux-public-wifi.service";
        
        // Since we are in a GUI, we use sudo cp via Command
        Command::new("sudo").args(["cp", src, dst]).output().ok();
        
        // 2. Fix path in service file to point to current release binary
        let binary_path = "/home/caesar/Documents/Projects/linux-public-wifi/target/release/daemon";
        let sed_cmd = format!("s|/usr/bin/python3 /usr/lib/linux-public-wifi/app/daemon/main.py|{}|g", binary_path);
        Command::new("sudo").args(["sed", "-i", &sed_cmd, dst]).output().ok();
        
        // 3. Reload and Enable
        Command::new("sudo").args(["systemctl", "daemon-reload"]).output().ok();
        Command::new("sudo").args(["systemctl", "enable", "linux-public-wifi.service"]).output().ok();
        
        println!("System setup complete.");
    });

    let ui_handle_v = ui.as_weak();
    ui.on_add_voucher(move |code, dur| {
        let _ui = ui_handle_v.unwrap();
        let duration: i32 = dur.parse().unwrap_or(60);
        match create_voucher(&code, duration) {
            Ok(_) => println!("Voucher {} created!", code),
            Err(e) => println!("Error: {}", e),
        }
    });

    ui.run()
}
