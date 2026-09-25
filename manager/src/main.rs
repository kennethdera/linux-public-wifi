slint::include_modules!();

use std::process::Command;
use shared::{create_voucher};
use slint::Color;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak();
    ui.on_toggle_hotspot(move || {
        let ui = ui_handle.unwrap();
        // Check the AP service instead of the portal daemon
        let status = Command::new("sudo")
            .args(["systemctl", "is-active", "linux-public-wifi-ap.service"])
            .output();

        if let Ok(out) = status {
            if String::from_utf8_lossy(&out.stdout).trim() == "active" {
                Command::new("sudo").args(["systemctl", "stop", "linux-public-wifi-ap.service"]).output().ok();
                Command::new("sudo").args(["systemctl", "stop", "linux-public-wifi.service"]).output().ok();
                ui.set_status_text("Status: Stopped".into());
                ui.set_status_color(Color::from_rgb_u8(255, 0, 0).into());
            } else {
                // Start the AP and the Portal
                Command::new("sudo").args(["systemctl", "start", "linux-public-wifi-ap.service"]).output().ok();
                Command::new("sudo").args(["systemctl", "start", "linux-public-wifi.service"]).output().ok();
                ui.set_status_text("Status: Running".into());
                ui.set_status_color(Color::from_rgb_u8(0, 255, 0).into());
            }
        }
    });

    let ui_handle_s = ui.as_weak();
    ui.on_setup_system(move || {
        let _ui = ui_handle_s.unwrap();
        
        // 1. DB Setup
        let user = std::env::var("USER").unwrap_or_else(|_| "caesar".to_string());
        Command::new("sudo").args(["mkdir", "-p", "/var/lib/linux-public-wifi"]).output().ok();
        Command::new("sudo").args(["chown", &format!("{}:users", user), "/var/lib/linux-public-wifi"]).output().ok();

        // 2. Configs Installation
        sudo_cp("/home/caesar/Documents/Projects/linux-public-wifi/hostapd.conf", "/etc/linux-public-wifi/hostapd.conf");
        sudo_cp("/home/caesar/Documents/Projects/linux-public-wifi/dnsmasq.conf", "/etc/linux-public-wifi/dnsmasq.conf");

        // 3. Service Installation
        sudo_cp("/home/caesar/Documents/Projects/linux-public-wifi/linux-public-wifi.service", "/etc/systemd/system/linux-public-wifi.service");
        sudo_cp("/home/caesar/Documents/Projects/linux-public-wifi/linux-public-wifi-ap.service", "/etc/systemd/system/linux-public-wifi-ap.service");
        
        // Fix paths
        let binary_path = "/home/caesar/Documents/Projects/linux-public-wifi/target/release/daemon";
        let sed_cmd = format!("s|/usr/bin/python3 /usr/lib/linux-public-wifi/app/daemon/main.py|{}|g", binary_path);
        Command::new("sudo").args(["sed", "-i", &sed_cmd, "/etc/systemd/system/linux-public-wifi.service"]).output().ok();
        
        // 4. Routing & Internet Sharing (The Fix for Issue 5)
        Command::new("sudo").args(["sysctl", "-w", "net.ipv4.ip_forward=1"]).output().ok();
        // Use enp2s0 as the WAN interface (based on earlier ip link show)
        Command::new("sudo").args(["iptables", "-t", "nat", "-A", "POSTROUTING", "-o", "enp2s0", "-j", "MASQUERADE"]).output().ok();
        Command::new("sudo").args(["iptables", "-A", "FORWARD", "-i", "wlp3s0", "-o", "enp2s0", "-j", "ACCEPT"]).output().ok();
        Command::new("sudo").args(["iptables", "-A", "FORWARD", "-i", "enp2s0", "-o", "wlp3s0", "-m", "state", "--state", "RELATED,ESTABLISHED", "-j", "ACCEPT"]).output().ok();
        
        Command::new("sudo").args(["systemctl", "daemon-reload"]).output().ok();
        Command::new("sudo").args(["systemctl", "enable", "linux-public-wifi.service"]).output().ok();
        Command::new("sudo").args(["systemctl", "enable", "linux-public-wifi-ap.service"]).output().ok();
        
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

fn sudo_cp(src: &str, dst: &str) {
    // Ensure destination directory exists
    let dst_parent = std::path::Path::new(dst).parent().unwrap();
    Command::new("sudo").args(["mkdir", "-p", dst_parent.to_str().unwrap()]).output().ok();
    Command::new("sudo").args(["cp", src, dst]).output().ok();
}
