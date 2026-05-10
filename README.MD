# Superrustendo 🎮

[![GitHub Actions](https://github.com/Chief-Detektor/superrustendo/actions/workflows/rust.yml/badge.svg)](https://github.com/Chief-Detektor/superrustendo/actions)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://opensource.org/licenses/MIT)

**Superrustendo** ist ein SNES-Emulator in Rust, der sich derzeit in der aktiven Entwicklung befindet.

## 📖 Dokumentation

- [🚀 Roadmap](ROADMAP.md) - Entwicklungsplan und Meilensteine
- [🤝 Contributing](CONTRIBUTING.md) - Wie du beitragen kannst
- [📜 Code of Conduct](CODE_OF_CONDUCT.md) - Verhaltensregeln

## 🎯 Was ist Superrustendo?

Superrustendo ist ein SNES (Super Nintendo Entertainment System) Emulator, der in Rust geschrieben ist. Das Projekt befindet sich noch in einem frühen Stadium, aber es bietet bereits:

- **CPU-Emulation**: Grundlegende 65816-CPU-Emulation mit vielen implementierten Opcodes
- **Speicherverwaltung**: Bus-System mit WRAM und Cartridge-Speicher
- **Tools**:
  - `snesdis` - Einfacher Disassembler für SNES-ROMs
  - `snesdbg` - Schrittweiser Debugger

### ⚠️ Aktueller Status

Das Projekt ist **noch nicht spielbar** - es kann ROMs laden und Code ausführen, aber viele Komponenten sind noch nicht implementiert:

- ✅ Grundlegende CPU-Emulation
- ✅ Speichersystem
- ⏳ PPU (Grafik) - Grundgerüst vorhanden
- ⏳ APU (Sound) - Grundgerüst vorhanden
- ❌ Keine vollständige Opcode-Unterstützung
- ❌ Keine Grafikausgabe
- ❌ Keine Soundausgabe

## 🎭 Warum?

Das SNES ist meine absolute Lieblingskonsole! 🎮

Ich wollte verstehen, wie das System funktioniert und wie man Software (Spiele/Demos) dafür schreibt. Gleichzeitig wollte ich meine Rust-Kenntnisse verbessern. Also habe ich beschlossen, diesen Emulator zu implementieren.

## 🚀 Schnellstart

### Voraussetzungen

- [Rust](https://www.rust-lang.org/tools/install) (stabile Version)
- [Cargo](https://doc.rust-lang.org/cargo/) (wird mit Rust installiert)

### Installation

```bash
# Repository klonen
git clone https://github.com/Chief-Detektor/superrustendo.git
cd superrustendo

# Abhängigkeiten installieren und kompilieren
cargo build --release
```

### Verwendung

#### Emulator ausführen
```bash
cargo run --bin superrustendo -- path/to/rom.sfc
```

#### Disassembler verwenden
```bash
cargo run --bin snesdis -- path/to/rom.sfc
```

#### Debugger verwenden
```bash
cargo run --bin snesdbg -- path/to/rom.sfc
```

## 🔍 snesdbg - Debugger Befehle

Sobald der Debugger mit einer SFC-Datei gestartet wurde, stehen folgende Befehle zur Verfügung:

| Befehl | Beschreibung |
|--------|--------------|
| `s` oder `step` | Ein Schritt (eine Instuktion ausführen) |
| `s n` oder `step n` | n Schritte ausführen |
| `p` oder `print cpu` | CPU-Zustand anzeigen |

## 📦 Releases

Sieh dir die [Releases-Seite](https://github.com/Chief-Detektor/superrustendo/releases) für vorgefertigte Binaries an.

## 🤝 Mitwirken

Jeder Beitrag ist willkommen! Bitte lies [CONTRIBUTING.md](CONTRIBUTING.md) für Details, wie du beitragen kannst.

### Wie du helfen kannst

- 🐛 **Bugs melden** - Melde Probleme über [GitHub Issues](https://github.com/Chief-Detektor/superrustendo/issues)
- 💡 **Feature-Ideen** - Teile deine Ideen in [GitHub Discussions](https://github.com/Chief-Detektor/superrustendo/discussions)
- 📝 **Dokumentation** - Verbessere die Dokumentation
- 🔧 **Code** - Implementiere fehlende Opcodes oder Features
- 🧪 **Tests** - Schreibe Tests für bestehende Funktionalität

## 📄 Lizenz

Dieses Projekt steht unter der [MIT-Lizenz](LICENSE).

## 🙏 Dank

Vielen Dank an alle, die zu diesem Projekt beitragen! 🎉

---

*Letzte Aktualisierung: Mai 2025*
