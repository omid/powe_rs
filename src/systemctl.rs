use std::fs;

pub struct SystemCtl<'a> {
    service: &'a str,
}

const SERVICE_PATH: &str = "/etc/systemd/system";

impl<'a> SystemCtl<'a> {
    pub fn new(service: &'a str) -> Self {
        Self { service }
    }
    pub fn install_service(service: &'a str, path: &str, ip: &str, port: u16) -> Self {
        let service_path = format!("{SERVICE_PATH}/{service}");
        let service_content = format!(
            r"[Unit]
Description=Power Control Web UI
After=network.target

[Service]
ExecStart={} serve -l {}:{}
Restart=always

[Install]
WantedBy=multi-user.target
",
            path, ip, port,
        );
        std::fs::write(&service_path, service_content).expect("Failed to write service file");
        SystemCtl::daemon_reload();
        Self::new(service)
    }

    pub fn uninstall_service(service: &'a str) {
        let sysctl = SystemCtl::new(service);
        sysctl.stop();
        sysctl.disable();
        fs::remove_file(&format!("{SERVICE_PATH}/{service}")).ok();
        SystemCtl::daemon_reload();
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
    pub fn poweroff(when: Option<&str>) {
        SystemCtl::log("power off");
        let mut cmd = std::process::Command::new("systemctl");
        cmd.arg("poweroff");
        if let Some(when_val) = when {
            cmd.arg("--when");
            cmd.arg(when_val);
        }
        cmd.output().ok();
    }
    pub fn reboot() {
        SystemCtl::log("reboot");
        std::process::Command::new("systemctl").args(["reboot"]).output().ok();
    }

    fn log(command: &str) {
        println!("User asked to {} via web ui of powe_rs", command);
    }
}
