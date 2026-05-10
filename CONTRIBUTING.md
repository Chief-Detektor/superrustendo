# Contributing zu Superrustendo

Vielen Dank für dein Interesse, zu Superrustendo beizutragen! Jeder Beitrag ist willkommen, egal ob du ein erfahrener Rust-Entwickler bist oder gerade erst anfängst.

---

## 📋 Code of Conduct

Durch das Mitwirken an diesem Projekt stimmst du zu, den [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct) einzuhalten.

---

## 🎯 Wie man helfen kann

### Für Anfänger
Diese Aufgaben sind ideal für neue Contributors:

- [ ] **Compiler-Warnungen beheben**: Führe `cargo check` aus und behebe alle Warnungen
- [ ] **Code formatieren**: Führe `cargo fmt` aus und formatiere den gesamten Codebase
- [ ] **Dokumentation hinzufügen**: Füge Modul-Dokumentation (`///` Kommentare) zu unkommentierten Funktionen hinzu
- [ ] **Unit-Tests schreiben**: Schreibe Tests für bestehende Opcodes in `src/cpu/instructions/ops/`
- [ ] **Typo-Fixes**: Korrigiere Rechtschreibfehler in Kommentaren und Dokumentation

### Für Fortgeschrittene

- [ ] **Fehlende Opcodes implementieren**: Siehe [ROADMAP.md](ROADMAP.md) für Details
- [ ] **PPU-Emulation**: Implementiere Tile-Rendering, Sprite-Handling, etc.
- [ ] **APU-Emulation**: Implementiere Sound-Channels und Audio-Mixing
- [ ] **Hardware-Register**: Implementiere fehlende Memory-Mapped Register
- [ ] **Performance-Optimierungen**: Profiling und Optimierung von Hotspots

### Für Experten

- [ ] **Architektur-Refactoring**: Verbessere Modulgrenzen, führe Traits ein
- [ ] **DMA-Implementierung**: Direct Memory Access Transfers
- [ ] **Save States**: Implementiere Speichern/Laden des Emulator-Zustands
- [ ] **JIT-Compiler**: Evaluier die Möglichkeit eines JIT-Compilers
- [ ] **Special Chips**: Super FX, SA-1, etc.

---

## 🛠️ Entwicklungsumgebung einrichten

### Voraussetzungen

- **Rust**: Neueste stabile Version (empfohlen: 1.70+)
- **Cargo**: Kommt mit Rust
- **Git**: Für Versionskontrolle

### Installation

```bash
# Rust installieren (falls nicht vorhanden)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Repository klonen
git clone https://github.com/Chief-Detektor/superrustendo.git
cd superrustendo

# Build testen
cargo check --no-default-features
```

### Empfohlene Tools

- **Editor**: VS Code mit [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
- **Alternative**: JetBrains RustRover oder IntelliJ IDEA mit Rust-Plugin
- **Debugger**: `gdb` oder `lldb` mit Rust-Support

---

## 🚀 Beitrags-Workflow

### 1. Issue finden oder erstellen

- Schau in den [Issues](https://github.com/Chief-Detektor/superrustendo/issues) nach offenen Aufgaben
- Filtere nach "good first issue" für Anfänger-freundliche Aufgaben
- Erstelle einen neuen Issue, wenn du eine Idee oder einen Bug gefunden hast

### 2. Branch erstellen

```bash
# Hauptbranch aktualisieren
git checkout main
git pull origin main

# Neuen Feature-Branch erstellen
git checkout -b feature/dein-feature-name
# oder für Bugfixes:
git checkout -b fix/bug-beschreibung
```

**Branch-Namenskonvention**:
- `feature/` - Neue Funktionalität
- `fix/` - Bugfixes
- `refactor/` - Code-Refactoring
- `docs/` - Dokumentation
- `test/` - Tests

### 3. Änderungen implementieren

- Halte deine Commits klein und fokussiert
- Jeder Commit sollte eine logische Einheit sein
- Schreibe klare Commit-Messages

**Commit-Message Format**:
```
Typ: kurze Beschreibung

Längere Beschreibung (optional)

- Bullet point 1
- Bullet point 2
```

**Typen**: `feat`, `fix`, `refactor`, `docs`, `test`, `chore`, `style`

Beispiele:
```bash
git commit -m "feat: implement ADC opcode"
git commit -m "fix: correct stack pointer wrap-around"
git commit -m "docs: add module documentation for CPU"
git commit -m "test: add unit tests for LDA instruction"
```

### 4. Code-Qualität sicherstellen

Vor dem Push:

```bash
# Code formatieren
cargo fmt

# Linter ausführen
cargo clippy -- -D warnings

# Tests ausführen
cargo test --no-default-features

# Build testen
cargo build --no-default-features
```

### 5. Pull Request erstellen

```bash
# Änderungen pushen
git push origin feature/dein-feature-name
```

Dann:
1. Gehe zu [GitHub Pull Requests](https://github.com/Chief-Detektor/superrustendo/pulls)
2. Klicke auf "New Pull Request"
3. Wähle deinen Branch als "compare" Branch
4. Fülle die PR-Beschreibung aus (siehe unten)
5. Erstelle den Pull Request

---

## 📝 Pull Request Template

Bitte fülle die folgende Vorlage für deine Pull Requests aus:

```markdown
## Beschreibung

Kurze Beschreibung der Änderungen.

## Motivation

Warum werden diese Änderungen benötigt? Welches Problem wird gelöst?

## Änderungen

- [ ] Neue Funktionalität
- [ ] Bugfix
- [ ] Refactoring
- [ ] Dokumentation
- [ ] Tests
- [ ] Sonstiges

## Verknüpfte Issues

Fixes #123
oder
Related to #456

## Checkliste

- [ ] Code formatiert (`cargo fmt`)
- [ ] Linter läuft sauber (`cargo clippy -- -D warnings`)
- [ ] Alle Tests bestehen (`cargo test`)
- [ ] Build erfolgreich (`cargo build`)
- [ ] Dokumentation aktualisiert (falls nötig)
- [ ] Commit-Messages sind klar und beschreibend
```

---

## 📜 Code-Standards

### Rust-Stil

- Folge dem [Rust Style Guide](https://doc.rust-lang.org/style-guide/)
- Verwende `rustfmt` für Formatierung
- Verwende `clippy` für Linting

### Spezifische Richtlinien

#### Naming
- Verwende `snake_case` für Funktionen und Variablen
- Verwende `PascalCase` für Typen
- Verwende `SCREAMING_SNAKE_CASE` für Konstanten

#### Fehlerbehandlung
- Verwende `Result` und `Option` statt `unwrap()` und `expect()`
- Verwende `?` Operator für Fehler-Propagation
- Definiere eigene Error-Typen für komplexe Fehler

#### Dokumentation
- Dokumentiere alle öffentlichen (`pub`) Items
- Verwende `///` für Dok-Kommentare
- Verwende `//` für Code-Kommentare
- Dokumentiere Panics, Errors und Safety

Beispiel:
```rust
/// Adds two numbers and returns the result.
///
/// # Examples
///
/// ```
/// let result = add(2, 3);
/// assert_eq!(result, 5);
/// ```
///
/// # Panics
///
/// This function will panic if the result overflows.
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

#### Tests
- Platziere Unit-Tests im selben Modul wie der getestete Code
- Verwende `#[cfg(test)]` für Test-Module
- Test-Dateien sollten `_test` oder `tests` im Namen haben
- Schreibe klare Test-Namen

Beispiel:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lda_immediate() {
        // Test-Setup
        let mut cpu = CPU::new();
        
        // Test-Execution
        cpu.execute(LDA_IMMEDIATE, 0x42);
        
        // Assertions
        assert_eq!(cpu.regs.A, 0x42);
    }
}
```

#### Performance
- Vermeide unnötige Allokationen
- Verwende Referenzen statt Klonen wo möglich
- Verwende `Copy` und `Clone` Typen wo angemessen
- Profiling vor Optimierung

---

## 🔍 Code Review Prozess

1. **PR Erstellen**: Erstelle einen Pull Request mit klarer Beschreibung
2. **Automatisierte Checks**: CI-Pipeline läuft automatisch
3. **Manueller Review**: Mindestens ein Maintainer reviewt den Code
4. **Feedback**: Addressiere alle Kommentare und Fragen
5. **Merge**: Nach Approval wird der PR gemerged

### Review-Kriterien

- [ ] Code folgt den Stil-Richtlinien
- [ ] Alle Tests bestehen
- [ ] Keine neuen Warnungen
- [ ] Dokumentation ist vollständig
- [ ] Änderungen sind gut strukturiert
- [ ] Keine Breaking Changes (oder dokumentiert)

---

## 📊 Projekt-Struktur

### Wichtige Dateien und Verzeichnisse

```
.
├── Cargo.toml           # Projekt-Konfiguration
├── README.MD            # Haupt-Dokumentation
├── ROADMAP.md           # Entwicklungsplan
├── CONTRIBUTING.md      # Diese Datei
├── PROJECT_ANALYSIS.md  # Projektanalyse
├── Makefile             # Häufige Aufgaben
├── src/                 # Haupt-Quellcode
│   ├── lib.rs           # Bibliothekseinstieg
│   ├── main.rs          # Haupt-Emulator
│   ├── apu/             # Audio Processing Unit
│   ├── cartridge/       # ROM-Loading
│   ├── cpu/             # Central Processing Unit
│   │   ├── mod.rs       # CPU-State
│   │   ├── decoder.rs   # Opcode-Dekodierung
│   │   └── instructions/ # Instruktionen
│   │       └── ops/     # Opcode-Implementierungen
│   ├── mem/             # Speicherverwaltung
│   ├── ppu/             # Picture Processing Unit
│   └── tooling/         # Hilfsprogramme
├── bin/                 # Binäre Programme
│   ├── snesdbg/         # Debugger
│   └── snesdis/         # Disassembler
├── tests/               # Integrationstests
└── snes-test-roms/      # Test-ROMs (gitignored)
```

---

## 💬 Kommunikation

- **Issues**: Für Bug-Reports und Feature-Requests
- **Pull Requests**: Für Code-Contributions
- **Discussions**: Für allgemeine Diskussionen

---

## 🙏 Anerkennung

Vielen Dank an alle, die zu diesem Projekt beitragen! Deine Hilfe ist sehr geschätzt.

---

## 📜 Lizenz

Durch das Mitwirken an diesem Projekt stimmst du zu, deine Contributions unter der [MIT OR Apache-2.0](LICENSE) Lizenz zu stellen.
