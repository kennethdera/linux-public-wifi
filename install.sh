#!/bin/bash
set -e

PROJECT_DIR="/home/caesar/Documents/Projects/linux-public-wifi"
BINARY_MANAGER="$PROJECT_DIR/target/release/manager"
BINARY_DAEMON="$PROJECT_DIR/target/release/daemon"
SERVICE_FILE="$PROJECT_DIR/linux-public-wifi.service"
DESKTOP_FILE="$PROJECT_DIR/linux-public-wifi.desktop"
ICON_FILE="$PROJECT_DIR/logo.png"

echo "Installing Linux Public WiFi..."

# 1. Install Binaries
sudo cp "$BINARY_MANAGER" /usr/local/bin/linux-public-wifi-manager
sudo cp "$BINARY_DAEMON" /usr/local/bin/linux-public-wifi-daemon

# 2. Install Systemd Service
sudo cp "$SERVICE_FILE" /etc/systemd/system/
# Update service file to use the installed path
sudo sed -i "s|/home/caesar/Documents/Projects/linux-public-wifi/target/release/daemon|/usr/local/bin/linux-public-wifi-daemon|g" /etc/systemd/system/linux-public-wifi.service
sudo systemctl daemon-reload
sudo systemctl enable linux-public-wifi.service

# 3. Install Icon
if [ -f "$ICON_FILE" ]; then
    sudo mkdir -p /usr/share/icons/hicolor/256x256/apps
    sudo cp "$ICON_FILE" /usr/share/icons/hicolor/256x256/apps/linux-public-wifi.png
    echo "Icon installed."
else
    echo "Warning: logo.png not found in project directory. Skipping icon installation."
fi

# 4. Install Desktop Entry
sudo mkdir -p /usr/share/applications
sudo cp "$DESKTOP_FILE" /usr/share/applications/

echo "Installation complete! You can now find 'Linux Public WiFi' in your app menu."
