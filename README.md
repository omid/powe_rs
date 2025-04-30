# powe_rs

A simple web UI to shutdown or restart your Linux machine, with systemd integration.

---

**Security Warning:**

- Do **not** expose this script to the internet or any public network. It is intended for use on trusted, private networks only.
- This tool can shutdown or restart your host. Use with care and consider restricting access (e.g., firewall, VPN).

---

## Features

- Minimal HTTP server with a web UI for shutdown and restart the host
- Confirmation modal for actions
- Systemd user service install option
- Customizable ip and port

## Usage

### Build locally

```sh
cargo build --release
```

### Install as a systemd service

```sh
sudo powe_rs install
```

Or if you want to listen to a custom ip and/or port, you can pass them like below:

```sh
sudo powe_rs install -l 127.0.0.1:8080
sudo powe_rs install -l 8080
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
