use tokio::sync::Mutex;
use std::{sync::Arc, os::windows::process::CommandExt, process::Command};

const CREATE_NO_WINDOW: u32 = 0x08000000;

pub struct BrowserManagerState {
    browser_manager_mutex: Arc<Mutex<BrowserManager>>,
}

impl BrowserManagerState {
    pub fn new() -> Self {
        Self {
            browser_manager_mutex: Arc::new(Mutex::new(BrowserManager::new()))
        }
    }
}

pub struct BrowserManager {
    processes: Vec<u32>,
}

impl BrowserManager {
    pub const fn new() -> Self {
        Self {
            processes: Vec::new(),
        }
    }

    pub fn push(&mut self, process: u32) {
        self.processes.push(process);
    }

    pub fn pop(&mut self, process: u32) {
        let i = self.processes.iter().position(|p| *p == process);

        if let Some(i) = i {
            self.processes.remove(i);
        }
    }

    pub fn clear(&mut self) {
        for pid in self.processes.iter() {
            let mut kill = Command::new("taskkill.exe");

            let pid = *pid;
            let pid = pid.to_string();

            kill.args(["/PID", &pid, "/F", "/T"]);
            kill.creation_flags(CREATE_NO_WINDOW);

            _ = kill.output();
        }

        self.processes.clear();
        _ = BrowserManager::clear_dir();
    }

    fn clear_dir() -> std::io::Result<()> {
        let dir = std::fs::read_dir(r".\download")?;

        for entry in dir.filter_map(|x| x.ok()) {
            let metadata = entry.metadata()?;

            if !metadata.is_dir() {
                continue;
            }

            let name = entry.file_name();
            let name = name.to_str().unwrap();
            let path = entry.path();

            if name.starts_with("data-dir-") {
                _ = std::fs::remove_dir_all(path);
            }
        }

        Ok(())
    }
}

impl Default for BrowserManager {
    fn default() -> Self {
        Self::new()
    }
}
