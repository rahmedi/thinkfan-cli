# thinkfan-cli

**A minimalist, fast, and terminal-native fan control tool for ThinkPads.**
Directly interacts with `/proc/acpi/ibm/fan` for setting fan levels, no dependency on `thinkfan`.

---

## Features

* Set manual fan levels instantly
* Reads and writes directly to IBM ACPI fan interface
* No daemons or background services required
* Reading fan status instantly

---

## Installation

### IMPORTANT!!!

Please enable thinkpad acpi module to make thinkfan-cli work properly

```bash
sudo modprobe thinkpad_acpi fan_control=1
```

For libreboot/coreboot devices:

```bash
sudo modprobe thinkpad_acpi force_load=1 fan_control=1
```

### Arch Linux (via AUR)

```bash
yay -S thinkfan-cli
```

### Manual

```bash
git clone https://github.com/rahmedi/thinkfan-cli.git
cd thinkfan-cli
cargo build --release
sudo cp target/release/thinkfan-cli /usr/bin/
```

---

## Usage

```bash
sudo thinkfan-cli -h            # Help message
sudo thinkfan-cli -s auto       # Switch to Auto controlled fan mode
sudo thinkfan-cli -s 3          # Set fan to level 3 manually (0–7)
```

Note: Requires root privileges to write to `/proc/acpi/ibm/fan`.

---

## Feedbacks

Open an issue or suggest a feature.
Tested on T480 | X220 | R61i - reports for other models welcome.

---

## Author

Maintained by [rahmed](https://github.com/rahmedi)
Licensed under the GPL v3 License.
