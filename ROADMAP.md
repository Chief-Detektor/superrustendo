# Superrustendo - Entwicklungsroadmap

## 📋 Übersicht

Superrustendo ist ein SNES-Emulator in Rust, der sich derzeit in einem frühen Entwicklungsstadium befindet. Dieses Dokument beschreibt den aktuellen Zustand, die geplanten Verbesserungen und die langfristige Vision.

## 🎯 Aktueller Zustand (Stand: Mai 2025)

### ✅ Funktionierende Komponenten
- **CPU-Emulation**: Grundlegende 65816-CPU-Emulation mit vielen implementierten Opcodes
- **Speicherverwaltung**: Bus-System mit WRAM, Cartridge-Speicher
- **Disassembler (snesdis)**: Einfacher Disassembler für SNES-ROMs
- **Debugger (snesdbg)**: Schrittweiser Debugger mit Grundfunktionen
- **APU**: Grundgerüst vorhanden (noch nicht voll funktionsfähig)
- **PPU**: Grundgerüst vorhanden (noch nicht voll funktionsfähig)

### ⚠️ Bekannte Einschränkungen
- Viele Opcodes noch nicht implementiert
- APU und PPU nur als Platzhalter
- Keine Grafikausgabe
- Keine Soundausgabe
- Keine vollständige Speicherabbildung
- Keine Interrupt-Behandlung

### 📊 Code-Qualität
- **Warnungen**: ~100+ Compiler-Warnungen (hauptsächlich Naming-Konventionen)
- **Testabdeckung**: Grundlegende Opcode-Tests vorhanden
- **Dokumentation**: Teilweise vorhanden, viele TODO-Kommentare
- **Code-Stil**: Inkonsistent (Mischung aus snake_case und CamelCase)

---

## 🚀 Entwicklungsphasen

### Phase 1: Code-Qualität & Stabilität (Priorität: HOCH)
**Ziel**: Saubere, wartbare Codebasis mit guter Testabdeckung

#### 📌 Aufgaben
- [x] Unused imports und borrows bereinigen
- [ ] Naming-Konventionen vereinheitlichen (snake_case für Variablen/Funktionen, PascalCase für Typen)
- [ ] Code-Dokumentation verbessern (Rustdoc-Kommentare)
- [ ] Konsistente Fehlerbehandlung implementieren
- [ ] Logging-System einführen (statt println!)
- [ ] CI/CD-Pipeline optimieren (GitHub Actions statt GitLab CI)

#### 📊 Erfolgsmetriken
- < 20 Compiler-Warnungen
- 100% Testabdeckung für Decoder
- Dokumentation für alle öffentlichen APIs

#### ⏱️ Zeitrahmen
- **Geschätzt**: 2-4 Wochen

---

### Phase 2: CPU-Vollständigkeit (Priorität: HOCH)
**Ziel**: Vollständige 65816-CPU-Emulation

#### 📌 Aufgaben
- [ ] Alle fehlenden Opcodes implementieren
- [ ] Interrupt-Behandlung implementieren
- [ ] Adressierungsmodi vervollständigen
- [ ] Cycle-accurate Timing implementieren
- [ ] Emulationsmodus vs. Native-Modus testen

#### 📊 Erfolgsmetriken
- Alle 256 Opcodes implementiert
- Alle Adressierungsmodi funktionieren
- CPU-Tests für alle Opcodes

#### ⏱️ Zeitrahmen
- **Geschätzt**: 4-8 Wochen

---

### Phase 3: Speichersystem & Cartridge (Priorität: MITTEL)
**Ziel**: Vollständige Speicherabbildung und Cartridge-Unterstützung

#### 📌 Aufgaben
- [ ] Vollständige Speicherabbildung (WRAM, SRAM, ROM, etc.)
- [ ] Bank-Switching für verschiedene Cartridge-Typen
- [ ] Save-State-Unterstützung
- [ ] ROM-Header-Parsing verbessern

#### 📊 Erfolgsmetriken
- Unterstützung für HiROM und LoROM
- Speicherzugriffstests
- Save-State-Funktionalität

#### ⏱️ Zeitrahmen
- **Geschätzt**: 3-6 Wochen

---

### Phase 4: PPU-Implementierung (Priorität: MITTEL)
**Ziel**: Grundlegende Grafikausgabe

#### 📌 Aufgaben
- [ ] PPU-Register implementieren
- [ ] Tile-Rendering
- [ ] Sprite-Rendering
- [ ] Palettenverwaltung
- [ ] Framebuffer-Integration (minifb)

#### 📊 Erfolgsmetriken
- Einfache Grafikdemos laufen
- Tile- und Sprite-Tests
- Framebuffer-Ausgabe funktioniert

#### ⏱️ Zeitrahmen
- **Geschätzt**: 6-12 Wochen

---

### Phase 5: APU-Implementierung (Priorität: NIEDRIG)
**Ziel**: Soundausgabe

#### 📌 Aufgaben
- [ ] SPC700-Emulation oder Integration mit snes-apu
- [ ] Audio-Buffer-Management
- [ ] Sound-Ausgabe (cpal oder SDL2)

#### 📊 Erfolgsmetriken
- Einfache Soundtests funktionieren
- Audio-Synchronisation mit CPU

#### ⏱️ Zeitrahmen
- **Geschätzt**: 4-8 Wochen

---

### Phase 6: Benutzeroberfläche & Tools (Priorität: NIEDRIG)
**Ziel**: Benutzerfreundliche Tools und UI

#### 📌 Aufgaben
- [ ] GUI mit SDL2 oder egui
- [ ] Debugger-Oberfläche verbessern
- [ ] Disassembler verbessern
- [ ] ROM-Informationen anzeigen
- [ ] Breakpoints und Watchpoints

#### 📊 Erfolgsmetriken
- Funktionierende GUI
- Verbesserte Debugging-Erfahrung

#### ⏱️ Zeitrahmen
- **Geschätzt**: 4-8 Wochen

---

## 📁 Projektstruktur

```
superrustendo/
├── src/
│   ├── main.rs                 # Haupt-Emulator
│   ├── lib.rs                  # Bibliotheksexports
│   ├── cpu/                    # CPU-Emulation
│   │   ├── mod.rs              # CPU-Hauptmodul
│   │   ├── decoder.rs          # Opcode-Decoder
│   │   ├── address.rs          # Adressierungsmodi
│   │   ├── constants.rs        # CPU-Konstanten
│   │   └── instructions/       # Opcode-Implementierungen
│   │       ├── mod.rs          # Instuktions-Modul
│   │       └── ops/            # Individuelle Opcodes
│   ├── mem/                    # Speicherverwaltung
│   │   ├── mod.rs              # Bus-System
│   │   ├── wram.rs             # WRAM
│   │   └── hw_registers.rs     # Hardware-Register
│   ├── ppu/                    # PPU-Emulation
│   │   ├── mod.rs              # PPU-Hauptmodul
│   │   └── sprite.rs           # Sprite-Handling
│   ├── apu/                    # APU-Emulation
│   │   └── mod.rs              # APU-Hauptmodul
│   ├── cartridge/              # Cartridge-Handling
│   │   └── mod.rs              # Cartridge-Lader
│   └── tooling/                # Hilfsprogramme
│       ├── mod.rs              # Tooling-Modul
│       └── disassembler.rs     # Disassembler
├── tests/                      # Tests
│   ├── mod.rs                  # Test-Module
│   └── decoder.rs              # Decoder-Tests
├── Cargo.toml                  # Projektkonfiguration
├── README.md                   # Projektbeschreibung
└── ROADMAP.md                  # Diese Datei
```

---

## 🔧 Technische Entscheidungen

### Architektur
- **Single-Threaded**: Der Emulator läuft derzeit single-threaded
- **Event-Driven**: Zukunft: Event-basierte Architektur für bessere Performance
- **Modular**: Klare Trennung der Komponenten (CPU, PPU, APU, Memory)

### Abhängigkeiten
- **Aktuell**: minifb (Framebuffer), rustyline (CLI), byteorder, hex
- **Zukünftig**: SDL2 (Input/Audio), cpal (Audio), serde (Save-States)

### Performance
- **Ziel**: 60 FPS auf modernen Systemen
- **Optimierungen**: Cycle-accurate Emulation, JIT-Compilation (langfristig)

---

## 📊 Qualitätsmetriken

### Code-Qualität
- [ ] < 20 Compiler-Warnungen
- [ ] 100% Rustdoc-Dokumentation für öffentliche APIs
- [ ] Konsistente Naming-Konventionen
- [ ] Keine unsafe-Blöcke (wo möglich)

### Tests
- [ ] Unit-Tests für alle CPU-Opcodes
- [ ] Integrationstests für Subsysteme
- [ ] ROM-Kompatibilitätstests

### Performance
- [ ] CPU: < 10% Overhead pro Opcode
- [ ] PPU: 60 FPS bei einfachen Spielen
- [ ] Speicher: < 100MB RAM-Nutzung

---

## 🤝 Beitragsrichtlinien

### Code-Stil
- **Rust**: Folge [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- **Naming**: snake_case für Variablen/Funktionen, PascalCase für Typen
- **Dokumentation**: Rustdoc-Kommentare für alle öffentlichen Items
- **Fehlerbehandlung**: Verwende Result und Option statt panic!

### Commit-Nachrichten
- **Format**: `<typ>: <beschreibung>` (feat, fix, docs, refactor, etc.)
- **Beispiele**:
  - `feat: Implement ADC opcode`
  - `fix: Remove unused borrow in disassembler`
  - `docs: Add CPU architecture documentation`
  - `refactor: Clean up address mode handling`

### Pull Requests
- Kleine, fokussierte Änderungen
- Alle Tests müssen passieren
- Code-Review erforderlich
- Aktualisiere Dokumentation bei API-Änderungen

---

## 📞 Kommunikation

- **Issues**: GitHub Issues für Bugs und Feature-Requests
- **Discussions**: GitHub Discussions für allgemeine Fragen
- **E-Mail**: andre.rosenhammer@gmail.com (für private Anfragen)

---

## 🎉 Meilensteine

### v0.1.0 (Aktuell)
- Grundlegende CPU-Emulation
- Einfacher Disassembler und Debugger

### v0.2.0 (Geplant: Q3 2025)
- Vollständige CPU-Emulation
- Code-Qualitätsverbesserungen
- Bessere Testabdeckung

### v0.3.0 (Geplant: Q4 2025)
- Grundlegende PPU-Implementierung
- Grafikausgabe
- Speichersystem vervollständigt

### v0.4.0 (Geplant: Q1 2026)
- APU-Implementierung
- Soundausgabe
- GUI-Basis

### v1.0.0 (Langfristig)
- Voll funktionsfähiger SNES-Emulator
- Gute Performance
- Benutzerfreundliche Oberfläche

---

## 📝 Changelog

### Unreleased
- Code-Bereinigung: Unused imports, borrows und unreachable code entfernt
- Warnungen von ~128 auf ~100 reduziert

### v0.1.0 (2024-01-30)
- Initialer Release
- Grundlegende CPU-Emulation
- Einfacher Disassembler (snesdis)
- Schrittweiser Debugger (snesdbg)

---

*Letzte Aktualisierung: Mai 2025*
