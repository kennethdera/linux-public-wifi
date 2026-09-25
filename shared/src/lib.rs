use rusqlite::{params, Connection, Result};
use std::process::Command;
use std::path::Path;
use std::fs;

pub struct Voucher {
    pub code: String,
    pub duration_minutes: i32,
    pub is_used: bool,
}

pub fn init_db() -> Result<()> {
    let path = "/var/lib/linux-public-wifi/vouchers.db";
    if let Some(parent) = Path::new(path).parent() {
        fs::create_dir_all(parent).ok();
    }
    
    let conn = Connection::open(path)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS vouchers (
            code TEXT PRIMARY KEY,
            duration_minutes INTEGER,
            is_used BOOLEAN DEFAULT 0,
            used_at TEXT
        )",
        [],
    )?;
    Ok(())
}

pub fn create_voucher(code: &str, duration: i32) -> Result<()> {
    let conn = Connection::open("/var/lib/linux-public-wifi/vouchers.db")?;
    conn.execute(
        "INSERT INTO vouchers (code, duration_minutes) VALUES (?, ?)",
        params![code, duration],
    )?;
    Ok(())
}

pub fn validate_voucher(code: &str) -> Result<Option<Voucher>> {
    let conn = Connection::open("/var/lib/linux-public-wifi/vouchers.db")?;
    let mut stmt = conn.prepare("SELECT code, duration_minutes, is_used FROM vouchers WHERE code = ?")?;
    let mut rows = stmt.query_map(params![code], |row| {
        Ok(Voucher {
            code: row.get(0)?,
            duration_minutes: row.get(1)?,
            is_used: row.get(2)?,
        })
    })?;

    if let Some(voucher) = rows.next() {
        let v = voucher?;
        if !v.is_used {
            return Ok(Some(v));
        }
    }
    Ok(None)
}

pub fn mark_voucher_used(code: &str) -> Result<()> {
    let conn = Connection::open("/var/lib/linux-public-wifi/vouchers.db")?;
    conn.execute(
        "UPDATE vouchers SET is_used = 1, used_at = CURRENT_TIMESTAMP WHERE code = ?",
        params![code],
    )?;
    Ok(())
}

pub fn grant_access(mac: &str) {
    let _ = Command::new("sudo")
        .args(["iptables", "-I", "FORWARD", "-m", "mac", "--mac-source", mac, "-j", "ACCEPT"])
        .output();
}

pub fn get_authorized_macs() -> Vec<String> {
    let output = Command::new("sudo")
        .args(["iptables", "-L", "FORWARD", "-n"])
        .output()
        .expect("failed to execute iptables");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut macs = Vec::new();
    for line in stdout.lines() {
        if line.contains("ACCEPT") {
            // Simple extraction for prototype
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() > 4 {
                macs.push(parts[4].to_string());
            }
        }
    }
    macs
}
