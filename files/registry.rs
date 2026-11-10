//! Windows registry read helpers for Marktext.

pub const APP_KEY: &str = "Software\\Marktext\\Marktext";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Marktext";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Marktext") },
        RegValue { name: "Version",     value: String::from("v3.1.7") },
        RegValue { name: "Channel",     value: String::from("lts") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Marktext") },
        RegValue { name: "Publisher",       value: String::from("Marktext") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Marktext\\uninstall.exe\"") },
    ]
}
