# Superrustendo - Entwicklungs-Roadmap

## Vision
Ein vollständiger, genauer SNES-Emulator in Rust, der:
- Alle offiziellen SNES-Spiele unterstützt
- Hohe Kompatibilität und Genauigkeit bietet
- Als Bibliothek und Standalone-Emulator nutzbar ist
- Entwickler-freundlich mit Debugging-Tools ist

---

## Meilensteine

### 🟢 Phase 0: Projektstabilisierung (HOCH - 1-2 Wochen)
**Ziel**: Build-Probleme beheben, Code-Qualität verbessern, CI/CD stabilisieren

#### Aufgaben
- [x] Projektanalyse durchführen
- [x] Build-Probleme mit minifb identifizieren
- [ ] `minifb` als optional feature markieren ✅
- [ ] Alle Compiler-Warnungen beheben
- [ ] `cargo clippy` Warnungen beheben
- [ ] `cargo fmt` auf gesamten Codebase anwenden
- [ ] CI/CD Pipeline anpassen (kein GUI-Build standardmäßig)
- [ ] Unit-Tests für bestehende Funktionalität hinzufügen
- [ ] Code-Coverage verbessern

#### Akzeptanzkriterien
- ✅ `cargo check --no-default-features` läuft ohne Warnungen
- ✅ `cargo build --no-default-features` erfolgreich
- ✅ `cargo test --no-default-features` erfolgreich
- ✅ CI-Pipeline läuft grün

---

### 🟡 Phase 1: CPU-Vollständigkeit (HOCH - 2-3 Wochen)
**Ziel**: Alle 65816 Opcodes implementieren und testen

#### Aufgaben
- [ ] Fehlende Opcodes identifizieren (derzeit 87/256 implementiert)
- [ ] Alle fehlenden Opcodes implementieren
- [ ] Unit-Tests für jeden Opcode hinzufügen
- [ ] CPU-State Tests hinzufügen
- [ ] Interrupt-Handling implementieren (NMI, IRQ, BRK)
- [ ] CPU-Timing und Cycle-Counting verbessern

#### Fehlende Opcodes (Auswahl)
- JSL, JML (Long Jumps)
- Complete flag operations
- All stack operations
- All transfer operations
- All block move operations

#### Akzeptanzkriterien
- ✅ Alle 256 Opcodes implementiert
- ✅ Jeder Opcode hat mindestens einen Unit-Test
- ✅ CPU kann einfache Test-ROMs ausführen

---

### 🟡 Phase 2: Speicher und Hardware (MITTEL - 2 Wochen)
**Ziel**: Vollständige Speicherkarte und Hardware-Register

#### Aufgaben
- [ ] Komplette SNES Memory Map implementieren
- [ ] Alle Hardware-Register implementieren
- [ ] Bank-Switching für Cartridges
- [ ] WRAM- und SRAM-Handling
- [ ] ROM-Mirroring unterstützen
- [ ] Save-RAM (Batterie-backup) implementieren

#### Akzeptanzkriterien
- ✅ Alle Speicherbereiche korrekt adressiert
- ✅ Cartridge-Loading für verschiedene Mapper
- ✅ Save-RAM funktioniert

---

### 🟡 Phase 3: PPU-Grundlagen (HOCH - 3-4 Wochen)
**Ziel**: Grundlegende Grafikausgabe

#### Aufgaben
- [ ] PPU-Register implementieren
- [ ] Tile- und Tilemap-Rendering
- [ ] Paletten-Handling
- [ ] Background-Layer (BG1-BG4)
- [ ] Sprite-Rendering (Grundlagen)
- [ ] Simple Framebuffer-Ausgabe (ohne GUI)

#### Akzeptanzkriterien
- ✅ Einfache Grafik wird korrekt gerendert
- ✅ Sprites werden angezeigt
- ✅ Background-Layer funktionieren

---

### 🟠 Phase 4: Tools und Debugging (MITTEL - 2 Wochen)
**Ziel**: Entwickler-Tools verbessern

#### Aufgaben
- [ ] snesdis: Vollständiger Disassembler
- [ ] snesdbg: Verbesserte Debugging-Funktionen
  - Breakpoints
  - Memory Inspection
  - Register Display
  - Step Over/Into
  - Call Stack
- [ ] Logging-System für CPU/PPU/APU
- [ ] Trace-Mode (Record & Replay)

#### Akzeptanzkriterien
- ✅ snesdis kann ROMs vollständig disassemblieren
- ✅ snesdbg bietet vollwertiges Debugging

---

### 🟠 Phase 5: APU und Sound (NIEDRIG - 2-3 Wochen)
**Ziel**: Sound-Emulation

#### Aufgaben
- [ ] SPC700 CPU emulieren
- [ ] APU-Register implementieren
- [ ] Sound-Channels (Square, Triangle, Noise, DPCM)
- [ ] Audio-Mixing
- [ ] Audio-Ausgabe (optional, mit Feature-Flag)

#### Akzeptanzkriterien
- ✅ Sound wird generiert (auch wenn nicht ausgegeben)
- ✅ APU-Register reagieren korrekt

---

### 🟠 Phase 6: Input und Controller (NIEDRIG - 1 Woche)
**Ziel**: Controller-Eingabe

#### Aufgaben
- [ ] Joypad-Register implementieren
- [ ] Controller-Input Handling
- [ ] Multiplayer-Support (2 Controller)
- [ ] Input-Mapping

#### Akzeptanzkriterien
- ✅ Controller-Input wird erkannt
- ✅ Spiele reagieren auf Eingaben

---

### 🟠 Phase 7: DMA und Special Features (NIEDRIG - 2 Wochen)
**Ziel**: Fortgeschrittene Hardware-Features

#### Aufgaben
- [ ] DMA-Transfers implementieren
- [ ] HDMA (H-Blank DMA)
- [ ] Mode 7 Grafik
- [ ] Super FX Chip (optional)
- [ ] SA-1 Chip (optional)

#### Akzeptanzkriterien
- ✅ DMA-Transfers funktionieren
- ✅ Mode 7 Spiele laufen

---

### 🔴 Phase 8: GUI und Benutzerinterface (NIEDRIG - 2-3 Wochen)
**Ziel**: Grafische Oberfläche

#### Aufgaben
- [ ] GUI-Framework auswählen (egui, iced, oder SDL2)
- [ ] Hauptfenster mit Emulation
- [ ] Menü-System
- [ ] Konfiguration (Settings Dialog)
- [ ] Fullscreen-Modus
- [ ] Screenshot-Funktion

#### Akzeptanzkriterien
- ✅ GUI ist funktional
- ✅ Einstellungen können gespeichert werden

---

### 🔴 Phase 9: Performance und Optimierung (NIEDRIG - fortlaufend)
**Ziel**: Emulationsgeschwindigkeit verbessern

#### Aufgaben
- [ ] Performance-Profiling
- [ ] Hotspots identifizieren und optimieren
- [ ] JIT-Compiler evaluieren (optional)
- [ ] Multi-Threading für PPU/APU
- [ ] Frame-Skipping

#### Akzeptanzkriterien
- ✅ Volle Geschwindigkeit auf mittelmäßiger Hardware
- ✅ Keine Frame-Drops bei einfachen Spielen

---

### 🔴 Phase 10: Kompatibilitätstests (MITTEL - fortlaufend)
**Ziel**: Spiele-Kompatibilität

#### Aufgaben
- [ ] Test-ROMs sammeln
- [ ] Kompatibilitäts-Datenbank erstellen
- [ ] Automatisierte Tests mit bekannten ROMs
- [ ] Bug-Tracking für spezifische Spiele

#### Akzeptanzkriterien
- ✅ 80% der getesteten Spiele laufen
- ✅ 50% der getesteten Spiele laufen fehlerfrei

---

## Prioritäten-Matrix

| Priorität | Bereich | Zeitrahmen |
|----------|---------|------------|
| HOCH | Build-Stabilisierung | 1-2 Wochen |
| HOCH | CPU-Vollständigkeit | 2-3 Wochen |
| HOCH | PPU-Grundlagen | 3-4 Wochen |
| MITTEL | Speicher/Hardware | 2 Wochen |
| MITTEL | Tools & Debugging | 2 Wochen |
| NIEDRIG | APU & Sound | 2-3 Wochen |
| NIEDRIG | Input | 1 Woche |
| NIEDRIG | DMA | 2 Wochen |
| NIEDRIG | GUI | 2-3 Wochen |
| NIEDRIG | Performance | fortlaufend |
| MITTEL | Kompatibilität | fortlaufend |

---

## Release-Plan

### v0.1.0 - Stabilisierung
- Build-Probleme behoben
- Code-Qualität verbessert
- Grundlegende CPU-Funktionalität

### v0.2.0 - CPU-Vollständigkeit
- Alle Opcodes implementiert
- Interrupts funktionieren
- Unit-Tests für CPU

### v0.3.0 - Grafik-Grundlagen
- PPU-Rendering
- Einfache Spiele laufen

### v0.4.0 - Tools
- Vollständiger Disassembler
- Debugger mit Breakpoints

### v0.5.0 - Sound
- APU-Emulation
- Audio-Ausgabe

### v1.0.0 - Vollständiger Emulator
- Alle Hauptfeatures
- Hohe Kompatibilität
- Stabil und performant

---

## Contributing

### Wie man helfen kann
1. **Code-Qualität**: Warnungen beheben, Tests schreiben
2. **Dokumentation**: Code kommentieren, Anleitungen schreiben
3. **Testing**: Spiele testen, Bugs melden
4. **Features**: Neue Funktionalität implementieren

### Good First Issues
- [ ] Compiler-Warnungen beheben
- [ ] Unit-Tests für bestehende Opcodes schreiben
- [ ] Dokumentation hinzufügen
- [ ] Code formatieren

### Mentoring
Für neue Contributors:
- Einfache Aufgaben sind mit "Good First Issue" markiert
- Komplexere Aufgaben haben detaillierte Beschreibungen
- Code-Reviews werden schnell durchgeführt

---

## Ressourcen

### Dokumentation
- [65816 CPU Reference](https://wiki.superfamicom.org/65816-reference)
- [SNES Hardware Documentation](https://problemkaputt.de/fullsnes.htm)
- [SNES Development Wiki](https://wiki.superfamicom.org/)

### Tools
- [No$SNS Debugger](https://problemkaputt.de/sns.htm)
- [Mesens Emulator](https://github.com/SourMesen/Mesen2)
- [bsnes](https://github.com/bsnes-emu/bsnes)

### Test-ROMs
- [snes9x Test ROMs](https://github.com/snes9xgit/snes9x/tree/master/test)
- [ZSNES Test ROMs](https://github.com/zsnes-dev/zsnes/tree/master/test)
