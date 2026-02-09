````md
# PIC64GX Zephyr + Rust Examples (West Manifest)

This repository provides a **west manifest + example applications** to build Zephyr RTOS for the
**Microchip PIC64GX Curiosity Kit** with **Rust support**.

It contains:
- A `west.yml` manifest that fetches:
  - a Zephyr fork with PIC64GX support
  - the Zephyr Rust module (`zephyr-lang-rust`)
  - additional modules required by the PIC64GX platform
- Example applications under `apps/` (currently: `apps/rust_blinky`)
- (Optional) west extension commands via `scripts/` (if enabled in `west.yml`)

> **Note:** This repository is **not** a full Zephyr tree. Zephyr and modules are downloaded into
> your workspace by `west update`.

---

## What’s included

- **Board:** `pic64gx_curiosity_kit` (PIC64GX Curiosity Kit)
- **Zephyr version:** 4.3.99 (as reported by `VERSION`)
- **Host tested:** Ubuntu 24.04.3 LTS (Release 24.04)
- **Rust support:** via `modules/lang/rust` (`zephyr-lang-rust`)

---

## Prerequisites

### Supported build hosts
This setup was tested on:
- **Ubuntu 24.04.3 LTS** (Release 24.04)

Other modern Linux distributions should work with the same toolchain.

### Toolchain and build tools
You need:
- Python 3 + `pip`
- CMake + Ninja (recommended)
- Zephyr SDK (toolchain + host tools)

Known-good toolchain used during development:
- **Zephyr SDK 0.17.4**

---

## Build instructions

### 1) Create a workspace and install `west` (recommended: Python venv)

```bash
mkdir -p zephyr_workspace && cd zephyr_workspace

python3 -m venv .venv
source .venv/bin/activate
pip install -U pip west
````

### 2) Clone this repository and initialize the west workspace

```bash
git clone https://github.com/luphiax/pic64gx-zephyr-examples-rust.git pic64gx-soc

west init -l pic64gx-soc
west update
west zephyr-export
source zephyr/zephyr-env.sh
```

If you get Python dependency errors during configuration/build, install Zephyr’s Python requirements:

```bash
pip install -r zephyr/scripts/requirements.txt
```

### 3) Build an application

General form:

```bash
west build -p -b pic64gx_curiosity_kit <application>
```

Example (Rust blinky):

```bash
west build -p -b pic64gx_curiosity_kit pic64gx-soc/apps/rust_blinky
```

Optional: generate an Eclipse CDT project:

```bash
west build -p -b pic64gx_curiosity_kit pic64gx-soc/apps/rust_blinky -G "Eclipse CDT4 - Unix Makefiles"
```

### 4) Useful checks

List PIC64GX boards available in the current workspace:

```bash
west boards | grep -i pic64gx
```

Show the exact revisions fetched by `west`:

```bash
west list
git -C zephyr rev-parse --short HEAD
git -C modules/lang/rust rev-parse --short HEAD
```

---

## Available applications

| application        | description                                        |
| ------------------ | -------------------------------------------------- |
| `apps/rust_blinky` | Rust LED blinky example using Zephyr Rust bindings |

---

## Reproducibility (pinned revisions)

This repository pins component revisions in `west.yml` (commit SHAs) to avoid breakages caused by
moving branches and Rust API changes.

Updating to newer versions:

1. edit the `revision:` fields in `west.yml`
2. run `west update`
3. rebuild (and update app code if APIs changed)

---

## Optional: west extensions (scripts)

If `west.yml` enables extended west commands via:

```yaml
manifest:
  self:
    west-commands: scripts/west-commands.yml
```

you can list available commands with:

```bash
west help
```

If the scripts require additional Python packages:

```bash
pip install -r scripts/requirements.txt
```

---

## License

See `LICENSE` (if present). This repository typically follows Zephyr’s Apache-2.0 conventions for
Zephyr-related work.

```
::contentReference[oaicite:0]{index=0}
```
