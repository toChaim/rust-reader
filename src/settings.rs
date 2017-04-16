use preferences::{Preferences, AppInfo, prefs_base_dir};
use winapi::{VK_OEM_2, VK_ESCAPE, VK_OEM_PERIOD, VK_OEM_MINUS, VK_OEM_PLUS};

const APP_INFO: AppInfo = AppInfo {
    name: "rust_reader",
    author: "us",
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    pub rate: i32,
    pub hotkeys: [(u32, u32); 6],
}

impl Settings {
    pub fn new() -> Settings {
        Settings {
            rate: 6,
            hotkeys: [(2, VK_OEM_2 as u32), // ctrl-? key
                      (7, VK_ESCAPE as u32), // ctrl-alt-shift-esk
                      (3, VK_OEM_2 as u32), // ctrl-alt-?
                      (2, VK_OEM_PERIOD as u32), // ctrl-.
                      (3, VK_OEM_MINUS as u32), // ctrl-alt--
                      (3, VK_OEM_PLUS as u32) /* ctrl-alt-= */],
        }
    }
    pub fn get_dir(&self) -> ::std::path::PathBuf {
        prefs_base_dir()
            .map(|mut p| {
                     p.push("us");
                     p.push("rust_reader");
                     p.push("setings.prefs.json");
                     p
                 })
            .unwrap_or(::std::path::PathBuf::new())
    }
    pub fn from_file() -> Settings {
        Settings::load(&APP_INFO, "setings").unwrap_or_else(|_| {
            println!("failed to lode settings.");
            Settings::new()
        })
    }
    pub fn to_file(&self) {
        if self.save(&APP_INFO, "setings").is_err() {
            println!("failed to save settings.");
        }
    }
}

impl Drop for Settings {
    fn drop(&mut self) {
        self.to_file()
    }
}
