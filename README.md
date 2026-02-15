# Orbital: Redox Display Server

Orbital is the modern windowing system and compositor for Redox OS, providing a secure and performant environment for graphical applications.

## 🏛️ Architecture

Orbital follows the microkernel philosophy, running as a user-space daemon and leveraging the system's scheme architecture for resource management.

- **[Architecture Deep Dive](doc/ARCHITECTURE.md)** - Details on the compositor and scheme interface.
- **[Client Development](https://gitlab.redox-os.org/redox-os/orbclient)** - How to write apps for Orbital.

## ✨ Features

- **Double-Buffered Rendering**: Smooth, flicker-free window management.
- **Modular Compositor**: Independent of the kernel for increased stability.
- **Unified Input**: Integrated with `inputd` for cross-device support.
- **Modern Frameworks**: First-class support for **Slint**, **Iced**, and **Egui**.

## 🛠️ Components

| Component | Purpose |
| --- | --- |
| **orbital** | The display server and background compositor. |
| **file manager** | Native file exploration tool. |
| **terminal** | High-performance terminal emulator. |
| **launcher** | Quick access to system applications. |

## 🚀 Getting Started

To build Orbital for testing on Linux:

```bash
cargo build --release
```

To run in the Redox build system, follow the instructions in the [Redox Book](https://doc.redox-os.org/book/podman-build.html).

---
*Orbital - The Window to Redox OS.*
