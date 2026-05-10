# Contributing to Superrustendo

Vielen Dank für dein Interesse, zu Superrustendo beizutragen! 🎉

Dieses Dokument beschreibt, wie du zum Projekt beitragen kannst, sei es durch Code, Dokumentation, Bug-Reports oder andere Formen der Unterstützung.

---

## 📋 Inhaltsverzeichnis

- [Wie du helfen kannst](#-wie-du-helfen-kannst)
- [Erste Schritte](#-erste-schritte)
- [Entwicklungsumgebung einrichten](#-entwicklungsumgebung-einrichten)
- [Code-Stil und Richtlinien](#-code-stil-und-richtlinien)
- [Commit-Nachrichten](#-commit-nachrichten)
- [Pull Requests](#-pull-requests)
- [Code Review Prozess](#-code-review-prozess)
- [Bug Reports](#-bug-reports)
- [Feature Requests](#-feature-requests)
- [Verhaltenskodex](#-verhaltenskodex)

---

## 🤝 Wie du helfen kannst

Es gibt viele Möglichkeiten, zu Superrustendo beizutragen:

### 🐛 Bugs melden
- Melde Bugs über [GitHub Issues](https://github.com/Chief-Detektor/superrustendo/issues)
- Gib so viele Details wie möglich an (Schritte zur Reproduktion, erwartetes vs. tatsächliches Verhalten)

### 💡 Feature Requests
- Teile deine Ideen für neue Features über [GitHub Issues](https://github.com/Chief-Detektor/superrustendo/issues)
- Diskutiere Feature-Ideen in [GitHub Discussions](https://github.com/Chief-Detektor/superrustendo/discussions)

### 📝 Dokumentation verbessern
- Verbessere bestehende Dokumentation
- Füge Beispiele und Tutorials hinzu
- Übersetze Dokumentation in andere Sprachen

### 🔧 Code beitragen
- Implementiere fehlende CPU-Opcodes
- Verbessere bestehende Implementierungen
- Füge Tests hinzu
- Optimiere Performance
- Behebe Bugs

### 🧪 Tests schreiben
- Schreibe Unit-Tests für CPU-Opcodes
- Erstelle Integrationstests
- Teste ROM-Kompatibilität

### 📊 Code-Qualität verbessern
- Behebe Compiler-Warnungen
- Verbessere Code-Dokumentation
- Refaktoriere unübersichtlichen Code

---

## 🚀 Erste Schritte

### Voraussetzungen

- [Rust](https://www.rust-lang.org/tools/install) (stabile Version)
- [Git](https://git-scm.com/downloads)
- Ein Texteditor oder IDE deiner Wahl (VS Code, IntelliJ IDEA, etc.)

### Projekt klonen

```bash
# Repository klonen
git clone https://github.com/Chief-Detektor/superrustendo.git
cd superrustendo

# Abhängigkeiten installieren
cargo build
```

### Projekt struktur

```
superrustendo/
├── src/                    # Haupt-Quellcode
│   ├── cpu/               # CPU-Emulation
│   ├── mem/               # Speicherverwaltung
│   ├── ppu/               # PPU-Emulation
│   ├── apu/               # APU-Emulation
│   ├── cartridge/         # Cartridge-Handling
│   └── tooling/           # Hilfsprogramme
├── tests/                 # Tests
├── Cargo.toml             # Projektkonfiguration
├── README.md              # Projektbeschreibung
├── ROADMAP.md             # Entwicklungsroadmap
└── CONTRIBUTING.md        # Diese Datei
```

---

## 🛠️ Entwicklungsumgebung einrichten

### 1. Repository klonen

```bash
git clone https://github.com/Chief-Detektor/superrustendo.git
cd superrustendo
```

### 2. Abhängigkeiten installieren

```bash
# Alle Abhängigkeiten herunterladen und kompilieren
cargo build

# Oder für Release-Build
cargo build --release
```

### 3. Tests ausführen

```bash
# Alle Tests ausführen
cargo test

# Nur Unit-Tests
cargo test --lib

# Nur Integrationstests
cargo test --test decoder

# Mit Code-Abdeckung (erfordert grcov)
cargo test -- --coverage
```

### 4. Tools ausführen

```bash
# Emulator ausführen
cargo run --bin superrustendo path/to/rom.sfc

# Disassembler ausführen
cargo run --bin snesdis path/to/rom.sfc

# Debugger ausführen
cargo run --bin snesdbg path/to/rom.sfc
```

### 5. Code-Qualität prüfen

```bash
# Compiler-Warnungen anzeigen
cargo check

# Code formatieren
cargo fmt

# Linting (erfordert clippy)
cargo clippy
```

---

## 📖 Code-Stil und Richtlinien

### Rust-Standards

Folge den [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/) und dem [Rust Style Guide](https://doc.rust-lang.org/style-guide/).

### Naming-Konventionen

| Element | Konvention | Beispiel |
|---------|-------------|----------|
| Variablen | snake_case | `let cpu_state = ...` |
| Funktionen | snake_case | `fn decode_opcode() {}` |
| Typen | PascalCase | `struct Cpu {}` |
| Enums | PascalCase | `enum AddressMode {}` |
| Konstanten | SCREAMING_SNAKE_CASE | `const MAX_MEMORY: usize = ...` |
| Module | snake_case | `mod cpu {}` |

### Code-Formatierung

- Verwende `cargo fmt` für automatische Formatierung
- Zeilenlänge: 100 Zeichen (Standard)
- Einrückung: 4 Leerzeichen

### Dokumentation

- Alle öffentlichen Items (structs, enums, functions, etc.) müssen Rustdoc-Kommentare haben
- Verwende `///` für Dokumentationskommentare
- Verwende `//` für Implementierungsnotizen

**Beispiel:**
```rust
/// Adds two numbers and returns the result.
///
/// # Arguments
///
/// * `a` - The first number
/// * `b` - The second number
///
/// # Returns
///
/// The sum of `a` and `b`
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

### Fehlerbehandlung

- Verwende `Result` und `Option` statt `panic!` oder `unwrap()`
- Verwende `?` Operator für Fehlerpropagierung
- Definiere eigene Error-Typen für komplexe Fehlerfälle

**Gut:**
```rust
fn read_memory(address: u16) -> Result<u8, MemoryError> {
    // ...
}
```

**Schlecht:**
```rust
fn read_memory(address: u16) -> u8 {
    // ...
    value.unwrap()  // ❌ Vermeide unwrap()
}
```

### Logging

- Verwende das `log` Crate für Logging (noch nicht implementiert, aber geplant)
- Vermeide `println!` für Debug-Ausgaben in Produktionscode

### Tests

- Schreibe Unit-Tests für alle öffentlichen Funktionen
- Verwende `#[test]` Attribute
- Teste Edge-Cases

**Beispiel:**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_add_overflow() {
        assert_eq!(add(u8::MAX, 1), 0);
    }
}
```

---

## 📝 Commit-Nachrichten

Verwende [Conventional Commits](https://www.conventionalcommits.org/) Format:

```
<typ>([scope]): <beschreibung>

[body]

[footer]
```

### Typen

| Typ | Beschreibung | Beispiel |
|-----|--------------|----------|
| `feat` | Neue Feature | `feat: Implement ADC opcode` |
| `fix` | Bugfix | `fix: Correct memory mirroring` |
| `docs` | Dokumentation | `docs: Add CPU architecture docs` |
| `refactor` | Code-Refactoring | `refactor: Clean up address modes` |
| `test` | Tests | `test: Add decoder unit tests` |
| `chore` | Wartungsaufgaben | `chore: Update dependencies` |
| `style` | Code-Stil | `style: Format code with cargo fmt` |
| `perf` | Performance | `perf: Optimize CPU cycle counting` |

### Beispiele

```bash
# Einfacher Commit
git commit -m "feat: Implement ADC opcode"

# Commit mit Body
git commit -m "fix: Correct memory mirroring

Fixes issue where HiROM addresses were not mirrored correctly.
This resolves #123"

# Commit mit Footer
git commit -m "feat: Add PPU tile rendering

Implements basic tile rendering for background layers.

Closes #45"
```

---

## 🔀 Pull Requests

### Vor dem Erstellen eines Pull Requests

1. **Fork das Repository** und arbeite in deinem eigenen Fork
2. **Erstelle einen Feature-Branch** von `main`:
   ```bash
   git checkout -b feat/my-feature
   ```
3. **Mache kleine, fokussierte Commits**
4. **Führe alle Tests aus**:
   ```bash
   cargo test
   ```
5. **Prüfe Code-Qualität**:
   ```bash
   cargo check
   cargo fmt
   ```

### Pull Request erstellen

1. Push deinen Branch zu deinem Fork:
   ```bash
   git push origin feat/my-feature
   ```

2. Erstelle einen Pull Request auf [GitHub](https://github.com/Chief-Detektor/superrustendo/pulls)

3. Fülle die PR-Vorlage aus:
   - Klare Beschreibung der Änderungen
   - Verlinkung zu relevanten Issues
   - Screenshots (falls zutreffend)

### Pull Request Checkliste

- [ ] Alle Tests passieren
- [ ] Code ist formatiert (`cargo fmt`)
- [ ] Keine neuen Compiler-Warnungen
- [ ] Dokumentation ist aktualisiert
- [ ] Commits folgen den Richtlinien
- [ ] PR-Beschreibung ist klar und vollständig

---

## 👀 Code Review Prozess

### Für Contributors

1. **Sei geduldig**: Code Reviews können Zeit brauchen
2. **Sei offen für Feedback**: Nimm Kritik konstruktiv auf
3. **Antworte auf Kommentare**: Kläre Fragen und implementiere vorgeschlagene Änderungen
4. **Aktualisiere deinen PR**: Push weitere Commits, um Feedback umzusetzen

### Für Reviewer

1. **Sei respektvoll**: Gib konstruktives Feedback
2. **Sei spezifisch**: Vermeide vage Kommentare wie "das gefällt mir nicht"
3. **Biete Lösungen an**: Nicht nur Kritik, sondern auch Vorschläge
4. **Reagiere zeitnah**: Versuche, innerhalb von 2-3 Tagen zu antworten

### Review-Kriterien

- [ ] Code folgt den Stilrichtlinien
- [ ] Code ist gut dokumentiert
- [ ] Tests decken die Änderungen ab
- [ ] Keine neuen Bugs eingeführt
- [ ] Performance-Impact ist akzeptabel
- [ ] Code ist wartbar und verständlich

---

## 🐛 Bug Reports

### Wie man einen guten Bug Report schreibt

1. **Suche nach bestehenden Issues**: Prüfe, ob der Bug bereits gemeldet wurde
2. **Verwende eine klare Überschrift**: Beschreibe das Problem kurz und präzise
3. **Gib so viele Details wie möglich an**:
   - Schritte zur Reproduktion
   - Erwartetes Verhalten
   - Tatsächliches Verhalten
   - Screenshots oder Logs (falls zutreffend)
   - Systeminformationen (OS, Rust-Version, etc.)

### Bug Report Vorlage

```markdown
## Beschreibung

[Klare Beschreibung des Bugs]

## Schritte zur Reproduktion

1. [Schritt 1]
2. [Schritt 2]
3. [Schritt 3]

## Erwartetes Verhalten

[Was sollte passieren?]

## Tatsächliches Verhalten

[Was passiert tatsächlich?]

## Zusätzliche Informationen

- OS: [Windows/Linux/macOS]
- Rust-Version: [Ausgabe von `rustc --version`]
- ROM: [Name der ROM, falls zutreffend]
- Logs:
  ```
  [Relevante Log-Ausgaben]
  ```
```

---

## 🚀 Feature Requests

### Wie man einen guten Feature Request schreibt

1. **Suche nach bestehenden Requests**: Prüfe, ob das Feature bereits vorgeschlagen wurde
2. **Verwende eine klare Überschrift**: Beschreibe das Feature kurz
3. **Erkläre das Problem**: Welches Problem löst das Feature?
4. **Beschreibe die Lösung**: Wie sollte das Feature funktionieren?
5. **Gib Beispiele an**: Mockups, Code-Beispiele, etc.

### Feature Request Vorlage

```markdown
## Feature Request

### Problem

[Welches Problem soll gelöst werden?]

### Vorschlagene Lösung

[Wie sollte das Feature funktionieren?]

### Alternativen

[Gibt es alternative Lösungen?]

### Zusätzliche Informationen

- Priorität: [Hoch/Mittel/Niedrig]
- Komplexität: [Einfach/Mittel/Komplex]
- Abhängigkeiten: [Welche neuen Abhängigkeiten wären nötig?]
```

---

## 🤝 Verhaltenskodex

Dieses Projekt folgt dem [Contributor Covenant Code of Conduct](https://www.contributor-covenant.org/version/2/0/code_of_conduct/).

### Kurzfassung

- Sei freundlich und respektvoll
- Sei inklusiv und einladend
- Vermeide beleidigende Sprache
- Akzeptiere konstruktive Kritik
- Respektiere die Entscheidungen der Maintainer

### Vollständiger Verhaltenskodex

Der vollständige Verhaltenskodex ist unter [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) zu finden.

---

## 📞 Kontakt

- **Issues**: [GitHub Issues](https://github.com/Chief-Detektor/superrustendo/issues)
- **Discussions**: [GitHub Discussions](https://github.com/Chief-Detektor/superrustendo/discussions)
- **E-Mail**: andre.rosenhammer@gmail.com

---

## 🙏 Dank

Vielen Dank an alle, die zu Superrustendo beitragen! Jeder Beitrag, egal wie klein, hilft dabei, dieses Projekt besser zu machen.

---

*Letzte Aktualisierung: Mai 2025*
