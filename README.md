
# PIC64GX Zephyr + Rust Examples (West Manifest)

This repository provides **Microchip PIC64GX SoC support** for the Zephyr RTOS ecosystem via a **west manifest repository**, including:

* A `west.yml` manifest that fetches:

  * a Zephyr fork with PIC64GX support
  * the Zephyr Rust module (`zephyr-lang-rust`)
  * additional modules required by the PIC64GX platform
* Sample Zephyr applications for Microchip’s PIC64GX SoC (currently: **Rust blinky**)
* Extended **west commands** to generate a payload compatible with the **HSS boot flow**
* Optional blob fetching via `west blobs fetch` (e.g., payload generator binaries)

> **Note:** This repository is **not** a full Zephyr tree. Zephyr and modules are downloaded into your workspace by `west update`.

---

## Prerequisites

This section describes the requirements needed before building and flashing a Zephyr application.

### Supported build hosts

This setup was tested on:

* **Ubuntu 24.04.3 LTS** (Release: 24.04)

### Install Zephyr SDK and build system on host PC

To build Zephyr applications on a host PC, install the Zephyr SDK and host build dependencies according to the official Zephyr installation documentation.

Known-good toolchain used during development:

* **Zephyr SDK 0.17.4**

Zephyr version used in this setup:

* **4.3.99** (as reported by Zephyr `VERSION`)

---

## Build Instructions

The following commands create a workspace and clone this repository into a directory called `pic64gx-soc`.

### Create a workspace and clone this repository

```bash
mkdir -p zephyr_workspace && cd zephyr_workspace
git clone https://github.com/luphiax/pic64gx-zephyr-examples-rust.git pic64gx-soc
```

### Initialize the west workspace

> Run `west init -l` from the **workspace root**, pointing to the manifest repository directory.

```bash
west init -l pic64gx-soc
```

### Fetch Zephyr + modules

```bash
west update
west zephyr-export
source zephyr/zephyr-env.sh
```

### If `west` is missing (recommended: Python venv)

```bash
python3 -m venv .venv
source .venv/bin/activate
pip install -U pip west
```

If you get Python dependency errors during configuration/build:

```bash
pip install -r zephyr/scripts/requirements.txt
```

---

## Build a Zephyr application

To build a Zephyr application, use:

```bash
west build -p -b pic64gx_curiosity_kit <application>
```

Where:

* `pic64gx_curiosity_kit` is the supported board for the PIC64GX Curiosity Kit
* `<application>` is the path to the application

Example (Rust blinky):

```bash
west build -p -b pic64gx_curiosity_kit pic64gx-soc/apps/rust_blinky
```

### (Optional) Generate an Eclipse CDT-4 based project

```bash
west build -p -b pic64gx_curiosity_kit pic64gx-soc/apps/rust_blinky -G "Eclipse CDT4 - Unix Makefiles"
```

---

## Applications

The table below lists the applications available for PIC64GX Curiosity Kit:

| application        | description                                |
| ------------------ | ------------------------------------------ |
| `apps/rust_blinky` | Rust LED blinky using Zephyr Rust bindings |

---

## Using West Commands for PIC64GX SoC

This repository includes scripts that extend Zephyr’s build system (**west**) via `scripts/`.

### Python requirements

If required by the scripts:

```bash
pip install -r scripts/requirements.txt
```

### List extension commands

```bash
west help
```

You should see extension commands from this manifest repository (path: `pic64gx-soc`), such as:

* `generate-payload` – invoke the HSS payload generator to create a bootable image
* `flash-payload` – flash a payload onto the target storage (if supported by your setup)

---

## Fetching Blobs with West

West supports fetching binary “blobs” from source control. This repository can fetch required binaries (e.g., payload generator executable) via:

```bash
west blobs fetch
```

This retrieves the payload generator binary (if configured as a blob in the manifest).
**Note:** This is required to use `west generate-payload` if your setup depends on a fetched payload generator.

---

## Generating a payload (HSS)

The `generate-payload` west command uses the HSS payload generator to create a binary suitable for booting on PIC64GX.

To generate a payload you typically need:

* a compiled Zephyr application (`zephyr.elf`)
* a YAML configuration file (examples under `payload-configs/`)

Example:

```bash
# Build the Rust blinky application
west build -p -b pic64gx_curiosity_kit pic64gx-soc/apps/rust_blinky

# Generate a payload (example config)
west generate-payload pic64gx-soc/payload-configs/single_hart_ddr.yaml output.bin
```

---

## Flashing a payload to an SD card

Once the payload is generated, one simple way to flash is using `dd`.

**WARNING:** Be extremely careful with `dd`. If you choose the wrong device, you can wipe your system disk.

Check the device name before flashing:

```bash
lsblk
```

Then flash (example assumes the SD card is `/dev/sda` — **verify on your system**):

```bash
sudo dd if=output.bin of=/dev/sda bs=4M status=progress conv=fsync
```

---

## Debugging (OpenOCD + GDB)

### OpenOCD (terminal 1)

Download and unpack the PIC64GX OpenOCD release (example: xPack OpenOCD), then run:

```bash
openocd --command "set DEVICE pic64gx" -f board/microchip_riscv_efp5.cfg
```

If you run from an unpacked directory:

```bash
cd xpack-openocd-0.12.0-3
./bin/openocd --command "set DEVICE pic64gx" -f board/microchip_riscv_efp5.cfg
```

### GDB (terminal 2)

Use a RISC-V GDB (either from xPacks or from Zephyr SDK). Example:

```bash
riscv64-zephyr-elf-gdb
```

In GDB:

```gdb
target extended-remote localhost:3333
# load symbols (example):
# file build/zephyr/zephyr.elf
```

---

## UART / udev rules (Linux)

On Linux you may need udev rules to access the embedded JTAG/UART without root.
Create:

`/etc/udev/rules.d/70-microchip.rules`

```udev
# Update rights for FlashPro 6
ACTION=="add", ATTRS{idVendor}=="1514", ATTRS{idProduct}=="200b", \
GROUP="dialout", MODE="0666"

# Bind ftdi_sio driver to all input
ACTION=="add", ATTRS{idVendor}=="1514", ATTRS{idProduct}=="2008", \
ATTRS{product}=="Embedded FlashPro5", ATTR{bInterfaceNumber}!="00", \
RUN+="/sbin/modprobe ftdi_sio", RUN+="/bin/sh -c 'echo 1514 2008 > /sys/bus/usb-serial/drivers/ftdi_sio/new_id'"

# Unbind ftdi_sio driver for channel A which should be the JTAG
SUBSYSTEM=="usb", DRIVER=="ftdi_sio", ATTR{bInterfaceNumber}=="00", ATTR{interface}=="Embedded FlashPro5", \
RUN+="/bin/sh -c 'echo $kernel > /sys/bus/usb/drivers/ftdi_sio/unbind'"

# Helper symlinks (optional)
KERNEL=="ttyUSB[0-9]*", SUBSYSTEM=="tty", SUBSYSTEMS=="usb", \
ATTRS{interface}=="Embedded FlashPro5", ATTRS{bInterfaceNumber}=="01", \
SYMLINK+="ttyUSB-FlashPro5B", GROUP="dialout", MODE="0666"

KERNEL=="ttyUSB[0-9]*", SUBSYSTEM=="tty", SUBSYSTEMS=="usb", \
ATTRS{interface}=="Embedded FlashPro5", ATTRS{bInterfaceNumber}=="02", \
SYMLINK+="ttyUSB-FlashPro5C", GROUP="dialout", MODE="0666"

KERNEL=="ttyUSB[0-9]*", SUBSYSTEM=="tty", SUBSYSTEMS=="usb", \
ATTRS{interface}=="Embedded FlashPro5", ATTRS{bInterfaceNumber}=="03", \
SYMLINK+="ttyUSB-FlashPro5D", GROUP="dialout", MODE="0666"
```

Reload udev rules:

```bash
sudo udevadm control --reload-rules
sudo udevadm trigger
```

---

## Reproducibility (pinned revisions)

This repository pins component revisions in `west.yml` (commit SHAs) to avoid breakages from moving branches and Rust API changes.

To inspect the exact revisions used in a workspace:

```bash
west list
git -C zephyr rev-parse --short HEAD
git -C modules/lang/rust rev-parse --short HEAD
```

To update versions:

1. edit `west.yml` (`revision:` fields)
2. run `west update`
3. rebuild (update application code if APIs changed)

---

## Additional Reading

* Zephyr Project documentation (Getting Started, Build system, Kconfig)
* Zephyr Rust documentation (Rust language support)
* Microchip PIC64GX documentation / board resources

---

## License

See `LICENSE` (if present). Zephyr-related work typically follows Apache-2.0 conventions.

---
