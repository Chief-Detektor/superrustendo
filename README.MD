# Superrustendo

[![CI Status](https://github.com/Chief-Detektor/superrustendo/actions/workflows/rust.yml/badge.svg)](https://github.com/Chief-Detektor/superrustendo/actions)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)

Ein SNES (Super Nintendo Entertainment System) Emulator geschrieben in Rust.

## 📋 Projektstatus

**Aktuelle Version**: 0.1.0 (Pre-Alpha)

### ✅ Implementiert
- 65816 CPU Emulation (87/256 Opcodes)
- Grundlegende Speicherverwaltung
- Cartridge-Loading
- Disassembler (snesdis)
- Step-Debugger (snesdbg)

### 🚧 In Entwicklung
- Vollständige CPU-Opcode-Abdeckung
- PPU (Picture Processing Unit) Emulation
- APU (Audio Processing Unit) Emulation

### ❌ Nicht implementiert
- Grafikausgabe (PPU-Rendering)
- Soundausgabe (APU)
- Controller-Input
- DMA-Transfers
- Save States

**Detaillierte Analyse**: Siehe [PROJECT_ANALYSIS.md](PROJECT_ANALYSIS.md)

**Entwicklungsplan**: Siehe [ROADMAP.md](ROADMAP.md)

---

## 🚀 Schnellstart

### Voraussetzungen
- Rust 2021 Edition (empfohlen: neueste stabile Version)
- Cargo

### Installation

```bash
# Klone das Repository
git clone https://github.com/Chief-Detektor/superrustendo.git
cd superrustendo

# Build ohne GUI (empfohlen für Entwicklung)
cargo build --no-default-features

# Oder mit Make
make build
```

### Verwendung

#### Emulator starten
```bash
# Ohne GUI (nur CPU-Emulation, kein Display)
cargo run --no-default-features -- <path-to-rom.sfc>

# Mit Make
make run ROM=path/to/rom.sfc
```

#### Disassembler
```bash
cargo run --no-default-features --bin snesdis <path-to-rom.sfc>

# Mit Make
make dis ROM=path/to/rom.sfc
```

#### Debugger
```bash
cargo run --no-default-features --bin snesdbg <path-to-rom.sfc>

# Mit Make
make disdbg ROM=path/to/rom.sfc
```

**Hinweis**: Der Emulator kann derzeit keine Grafik ausgeben, da die PPU-Emulation noch nicht vollständig implementiert ist. Die CPU-Emulation läuft jedoch und kann ROMs ausführen (bis zu nicht implementierten Opcodes).

---

## 📦 Features

### CPU (65816)
- 16/8-Bit Modus (M und X Flags)
- Alle Adressmodi (26 Modi)
- 87 implementierte Opcodes
- Register: A, B, X, Y, D, S, PBR, DBR, PC, P (Status)
- Stack-Operationen
- Interrupts (teilweise)

### Speicher
- 64KB WRAM
- Cartridge-ROM Loading
- Bank-Switching (Grundlagen)
- Hardware-Register (teilweise)

### Tools

#### snesdis - Disassembler
Ein einfacher Disassembler für SNES-ROMs.

```bash
cargo run --bin snesdis rom.sfc
```

#### snesdbg - Step Debugger
Ein interaktiver Debugger mit Step-Funktionalität.

**Verfügbare Kommandos**:
- `s` oder `step` - Ein Schritt ausführen
- `s n` oder `step n` - n Schritte ausführen
- `p` oder `print cpu` - CPU-Status anzeigen

```bash
cargo run --bin snesdbg rom.sfc
```

---

## 🛠️ Entwicklung

### Projektstruktur
```
src/
├── lib.rs              # Hauptbibliothek
├── main.rs             # Haupt-Emulator
├── apu/                # Audio Processing Unit
├── cartridge/          # ROM-Loading
├── cpu/                # Central Processing Unit
│   ├── mod.rs          # CPU-State & Register
│   ├── address.rs      # Adressberechnungen
│   ├── addressmodes.rs # 26 Adressmodi
│   ├── constants.rs    # Opcode-Konstanten
│   ├── decoder.rs      # Opcode-Dekodierung
│   └── instructions/   # Instruktionsimplementierung
│       ├── mod.rs      # Instruction Struct
│       └── ops/        # 87 Opcode-Implementierungen
├── mem/                # Speicherverwaltung
├── ppu/                # Picture Processing Unit
└── tooling/            # Hilfsprogramme

bin/
├── snesdbg/            # Debugger
└── snesdis/            # Disassembler
```

### Build-Optionen

#### Features
- `gui` - Aktiviert GUI-Support (minifb)

```bash
# Ohne GUI (Standard)
cargo build --no-default-features

# Mit GUI
cargo build --features gui
```

### Makefile
Ein Makefile ist für häufige Aufgaben verfügbar:

```bash
make help      # Zeigt alle verfügbaren Kommandos
make build     # Build ohne GUI
make build-gui # Build mit GUI
make check     # cargo check
make test      # Tests ausführen
make fmt       # Code formatieren
make clippy    # Linter ausführen
make doc       # Dokumentation generieren
make clean     # Build-Artifacts bereinigen
```

### Code-Qualität

#### Formatierung
```bash
cargo fmt
# oder
make fmt
```

#### Linting
```bash
cargo clippy -- -D warnings
# oder
make clippy
```

#### Tests
```bash
cargo test
# oder
make test
```

---

## 🤝 Contributing

Contributions sind willkommen! Bitte lese [CONTRIBUTING.md](CONTRIBUTING.md) für Details.

### Good First Issues
- Compiler-Warnungen beheben
- Unit-Tests für bestehende Opcodes schreiben
- Dokumentation hinzufügen
- Code formatieren

### Wie man hilft
1. Fork das Repository
2. Erstelle einen Feature-Branch (`git checkout -b feature/amazing-feature`)
3. Commit deine Änderungen (`git commit -m 'Add amazing feature'`)
4. Push zum Branch (`git push origin feature/amazing-feature`)
5. Öffne einen Pull Request

---

## 📜 Lizenz

Dieses Projekt ist unter **MIT OR Apache-2.0** lizenziert. Siehe [LICENSE](LICENSE) für Details.

---

## 📚 Ressourcen

### Dokumentation
- [65816 CPU Reference](https://wiki.superfamicom.org/65816-reference)
- [SNES Hardware Documentation](https://problemkaputt.de/fullsnes.htm)
- [SNES Development Wiki](https://wiki.superfamicom.org/)

### Tools
- [No$SNS Debugger](https://problemkaputt.de/sns.htm)
- [Mesens Emulator](https://github.com/SourMesen/Mesen2)
- [bsnes](https://github.com/bsnes-emu/bsnes)

### Ähnliche Projekte
- [rustynes](https://github.com/EmulatorJS/rustynes) - NES Emulator in Rust
- [mgba](https://github.com/mgba-emu/mgba) - GBA Emulator (teilweise in Rust)
- [RustBoyAdvance](https://github.com/trevarj/RustBoyAdvance) - GBA Emulator in Rust
