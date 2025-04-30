pub struct SystemCtl<'a> {
    service: &'a str,
}

impl<'a> SystemCtl<'a> {
    pub fn new(service: &'a str) -> Self {
        Self { service }
    }
    pub fn daemon_reload() {
        std::process::Command::new("systemctl").args(["daemon-reload"]).output().ok();
    }
    pub fn enable(&self) {
        std::process::Command::new("systemctl").args(["enable", self.service]).output().ok();
    }
    pub fn start(&self) {
        std::process::Command::new("systemctl").args(["start", self.service]).output().ok();
    }
    pub fn stop(&self) {
        std::process::Command::new("systemctl").args(["stop", self.service]).output().ok();
    }
    pub fn disable(&self) {
        std::process::Command::new("systemctl").args(["disable", self.service]).output().ok();
    }
}
