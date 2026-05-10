# Entwicklung - Superrustendo

Dieses Dokument fasst die aktuellen Entwicklungsaktivitäten und den Fortschritt des Projekts zusammen.

## 📋 Aktueller Stand

### Letzte Commits
```
52e7b93 docs: Update README.md with comprehensive project documentation
13c751f docs: Add project documentation and CI/CD pipeline
04f784c Code cleanup: Fix unused imports, borrows, and unreachable code
```

### Code-Qualität
- **Compiler-Warnungen**: ~100 (von ursprünglich ~128 reduziert)
- **Code-Bereinigung**: 40 Zeilen netto entfernt
- **Dokumentation**: 4 neue Dokumentationsdateien

## 🎯 Offene Issues (14 insgesamt)

### Phase 1: Code-Qualität & Stabilität 🟡
- **#2** - Naming-Konventionen vereinheitlichen
- **#3** - Code-Dokumentation verbessern
- **#4** - Logging-System einführen

### Phase 2: CPU-Vollständigkeit 🔴
- **#5** - Alle fehlenden CPU Opcodes implementieren
- **#6** - Interrupt-Behandlung implementieren
- **#7** - Cycle-accurate Timing implementieren

### Phase 3: Speichersystem & Cartridge 🔴
- **#8** - Vollständige Speicherabbildung implementieren
- **#9** - Bank-Switching für Cartridge-Typen implementieren
- **#10** - Save-State-Unterstützung implementieren

### Phase 4: PPU-Implementierung 🟡
- **#11** - PPU-Implementierung

### Phase 5: APU-Implementierung 🟢
- **#12** - APU-Implementierung

### Phase 6: Benutzeroberfläche & Tools 🟢
- **#13** - GUI implementieren
- **#14** - Debugger verbessern

## 🚀 Wie du helfen kannst

1. **Wähle ein Issue** aus der Liste oben
2. **Fork das Repository** und erstelle einen Branch
3. **Arbeite an der Lösung** und erstelle einen Pull Request
4. **Verweise auf das Issue** in deinem Pull Request

### Gute erste Issues (für Neueinsteiger)
- #2: Naming-Konventionen vereinheitlichen
- #3: Code-Dokumentation verbessern
- #4: Logging-System einführen

### Fortgeschrittene Issues
- #5: Alle fehlenden CPU Opcodes implementieren
- #6: Interrupt-Behandlung implementieren
- #8: Vollständige Speicherabbildung implementieren

## 📊 Entwicklungsmetriken

### Code-Statistiken
- **Gesamtzeilen Code**: ~8600 (src/)
- **Anzahl Dateien**: 106 Rust-Dateien
- **CPU-Opcodes**: 87 implementierte Opcodes
- **Tests**: 1143 Zeilen Testcode

### Abdeckung
- **Decoder-Tests**: Vollständig
- **CPU-Tests**: Teilweise
- **PPU/APU-Tests**: Noch nicht vorhanden

## 🔧 Entwicklungsumgebung

### Voraussetzungen
- Rust (stable)
- Git
- Cargo

### Setup
```bash
git clone https://github.com/Chief-Detektor/superrustendo.git
cd superrustendo
cargo build
```

### Tests ausführen
```bash
cargo test
```

### Code-Qualität prüfen
```bash
cargo check
cargo fmt --all -- --check
cargo clippy -- -D warnings
```

## 📞 Kommunikation

- **Issues**: https://github.com/Chief-Detektor/superrustendo/issues
- **Discussions**: https://github.com/Chief-Detektor/superrustendo/discussions
- **E-Mail**: andre.rosenhammer@gmail.com

## 📅 Meilensteine

| Version | Geplant | Status |
|---------|---------|--------|
| v0.1.0 | Januar 2024 | ✅ Abgeschlossen |
| v0.2.0 | Q3 2025 | 🟡 In Arbeit |
| v0.3.0 | Q4 2025 | ⏳ Geplant |
| v0.4.0 | Q1 2026 | ⏳ Geplant |
| v1.0.0 | TBA | ⏳ Geplant |

---

*Letzte Aktualisierung: Mai 2025*
