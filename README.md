# powe_rs

A simple web UI to power off or reboot your Linux machine in 5-4-3-2-1 seconds, by default on port 54321 — because, obviously.

---

<p align="center">
  <img src="https://github.com/omid/powe_rs/raw/refs/heads/master/assets/screen.apng" alt="Web UI Screenshot" width="400" />
</p>

---

**Security Warning:**

- Do **not** expose this script to the internet or any public network. It is intended for use on trusted, private networks only.
- This tool can power off or reboot your host. Use with care and consider restricting access (e.g., firewall, VPN).

---

## Features

- Minimal HTTP server with a web UI for power off and reboot the host
- Confirmation modal for actions
- Systemd user service install option
- Customizable ip and port

## Installation

Choose one of these ways

### 1. Pre-built packages (Recommended / Probably Easier and Faster)

Go to https://github.com/omid/powe_rs/releases/latest and choose the amd64 or aarch64 version, based on your CPU architecture, and download the file.

### 2. Build locally

Ensure you have Git and Cargo installed.

Then run the following command:

```sh
cargo build --release
```

## Usage

### Install the service

Just run this:

```sh
sudo powe_rs install
```

It will install the service on the default port, 54321. Then you can open http://127.0.0.1:54321 or use your machine's IP address.

Or if you want to listen to a custom ip and/or port, you can pass them like below:

```sh
sudo powe_rs install -l 127.0.0.1:8080
sudo powe_rs install -l 8080
```

The command above will also install the binary on your system. After that, you can run the command manually as shown below:

```sh
powe_rs serve
```

Or

```sh
powe_rs serve -l 127.0.0.1:8080
powe_rs serve -l 8080
```

You can set the `DRY=true` environment variable to test the script without actually running the commands.

```sh
DRY=true powe_rs serve
```

### Uninstall the service

To remove the systemd service:

```sh
sudo powe_rs uninstall
```

To also remove the installed binary:

```sh
sudo powe_rs uninstall -a
```

### Help

You can always see the help message with:

```sh
powe_rs -h
```

**Note:**

This project was written about 80-90% by AI. The main motivation was my personal need to easily and remotely turn off my Raspberry Pi.

## License

MIT
