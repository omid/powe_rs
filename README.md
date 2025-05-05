# powe_rs

A simple web UI to power off or reboot your Linux machine, with systemd integration.

---

<p align="center">
  <video src="assets/screen.mp4" alt="Web UI Screenshot" width="400" loop autoplay muted></video>
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

## Usage

### Build locally

```sh
cargo build --release
```

### Install as a systemd service

Just run this:

```sh
sudo powe_rs install
```

It will install it on the default port, which is 54321. Then you can open it http://127.0.0.1:54321 or on the machine's IP.

Or if you want to listen to a custom ip and/or port, you can pass them like below:

```sh
sudo powe_rs install -l 127.0.0.1:8080
sudo powe_rs install -l 8080
```

The command above will install a binary on your system also. It means after that you can run the command manually like below:

```sh
powe_rs serve
```

Or

```sh
powe_rs serve -l 127.0.0.1:8080
powe_rs serve -l 8080
```

You can set `DRY=true` env var to test the script without running the commands.

```sh
DRY=true powe_rs serve
```

## Uninstall

To remove the systemd service:

```sh
sudo powe_rs uninstall
```

To also remove the installed powe_rs binary:

```sh
sudo powe_rs uninstall -a
```

## Help

You can always see the help message with:

```sh
powe_rs -h
```

**Note:**

This project was written about 80-90% by AI, with the main motivation being my personal need to easily and remotely turn off my Raspberry Pi.

## License

MIT
