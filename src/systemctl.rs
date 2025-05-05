pub struct SystemCtl<'a> {
    service: &'a str,
}

const SERVICE_PATH: &str = "/etc/systemd/system";

impl<'a> SystemCtl<'a> {
    pub fn new(service: &'a str) -> Self {
        Self { service }
    }
    pub fn install_service(service: &'a str, path: &str, ip: &str, port: u16) -> Self {
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
        std::fs::write(format!("{SERVICE_PATH}/{service}"), service_content).expect("Failed to write service file");
        SystemCtl::daemon_reload();
        Self::new(service)
    }

    pub fn uninstall_service(service: &'a str) {
        let sysctl = SystemCtl::new(service);
        sysctl.stop();
        sysctl.disable();
        std::fs::remove_file(format!("{SERVICE_PATH}/{service}")).ok();
        SystemCtl::daemon_reload();
    }

    pub fn daemon_reload() {
        let mut cmd = Self::systemctl_cmd();
        cmd.arg("daemon-reload");
        Self::run_cmd(cmd);
    }
    pub fn enable(&self) {
        let mut cmd = Self::systemctl_cmd();
        cmd.args(["enable", self.service]);
        Self::run_cmd(cmd);
    }
    pub fn start(&self) {
        let mut cmd = Self::systemctl_cmd();
        cmd.args(["start", self.service]);
        Self::run_cmd(cmd);
    }
    pub fn stop(&self) {
        let mut cmd = Self::systemctl_cmd();
        cmd.args(["stop", self.service]);
        Self::run_cmd(cmd);
    }
    pub fn disable(&self) {
        let mut cmd = Self::systemctl_cmd();
        cmd.args(["disable", self.service]);
        Self::run_cmd(cmd);
    }

    pub fn poweroff(when: Option<&str>) {
        Self::do_power_action("poweroff", when, "power off");
    }
    pub fn reboot(when: Option<&str>) {
        Self::do_power_action("reboot", when, "reboot");
    }

    fn do_power_action(action: &str, when: Option<&str>, log_label: &str) {
        let mut cmd = Self::systemctl_cmd();
        cmd.arg(action);
        if let Some(when_val) = when {
            cmd.arg("--when");
            cmd.arg(when_val);
        }
        if when == Some("cancel") {
            SystemCtl::log(&format!("cancel scheduled {}", log_label));
        } else {
            SystemCtl::log(log_label);
        }
        Self::run_cmd(cmd);
    }

    fn run_cmd(mut cmd: std::process::Command) {
        if std::env::var("DRY").unwrap_or("false".to_string()) == "false" {
            cmd.output().ok();
        }
    }

    fn systemctl_cmd() -> std::process::Command {
        std::process::Command::new("systemctl")
    }

    fn log(command: &str) {
        println!("User asked to {} via web ui of powe_rs", command);
    }
}
