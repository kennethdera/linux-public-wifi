# Linux Public WiFi

A professional, high-performance captive portal solution for Linux, featuring a voucher-based authentication system and a native management GUI.

This application allows you to transform a Linux machine into a public WiFi hotspot where users must enter a valid voucher code to gain internet access.

## 🚀 Features

- **Native Performance**: Entirely rewritten in Rust for minimal memory footprint and maximum stability.
- **Voucher System**: Generate unique codes with specific durations.
- **Captive Portal**: Fast, asynchronous web server that intercepts client traffic.
- **Management GUI**: Modern native interface to control the hotspot and manage vouchers.
- **System Integration**: Fully integrated with `systemd` for background operation and boot-time startup.
- **Firewall Automation**: Automatic MAC-based access control using `iptables`.

## 🛠 Tech Stack

- **Backend**: Rust with [Axum](https://github.com/tokio-rs/axum) and [Tokio](https://tokio.rs/)
- **Frontend**: [Slint](https://slint.dev/) (Native GUI)
- **Database**: SQLite (via `rusqlite`)
- **Network**: `hostapd` (Access Point), `dnsmasq` (DHCP/DNS), `iptables` (Firewall)

## 📦 Installation

### Prerequisites

Ensure you have the following system packages installed:
```bash
sudo pacman -S hostapd dnsmasq iptables
```

### Build and Install

1. **Clone the repository** and navigate to the project root.
2. **Build the project**:
   ```bash
   cargo build --release
   ```
3. **Run the installation script**:
   ```bash
   sudo ./install.sh
   ```

## 🖥 Usage

1. **Launch the Manager**:
   Run the compiled GUI binary:
   ```bash
   /usr/local/bin/linux-public-wifi-manager
   ```
2. **Configure the System**:
   If it is your first time running the app, click the **"Setup System"** button in the GUI to register the background service.
3. **Create Vouchers**:
   Enter a code and a duration (in minutes) and click **Add**.
4. **Start Hotspot**:
   Click **"Toggle Hotspot"**. Your machine will begin broadcasting the "FreePublicWiFi" SSID.
5. **Client Connection**:
   Clients connecting to the WiFi will be redirected to the portal page. Once they enter a valid voucher, they are granted internet access.

## 📂 Project Structure

- `daemon/`: The asynchronous portal server and authentication engine.
- `manager/`: The Slint-based administrative desktop application.
- `shared/`: Common logic for database access and firewall management.
- `templates/`: HTML/CSS for the captive portal landing page.

## 🗺 Future Roadmap

Detailed checkpoints are maintained in `PROJECT_STATE.txt`, including:
- Real-time MAC extraction from DHCP leases.
- Automated time-based access revocation.
- Per-user bandwidth limiting via `tc`.
- Multi-interface selection support.
