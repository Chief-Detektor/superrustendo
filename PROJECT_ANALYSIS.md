# Superrustendo - Projektanalyse

## Aktueller Stand (10. Mai 2025)

### Projektübersicht
- **Name**: Superrustendo
- **Ziel**: SNES Emulator in Rust
- **Autor**: Andre Rosenhammer
- **Sprache**: Rust 2021 Edition
- **Lizenz**: Nicht angegeben

### Code-Statistik
- **Gesamtzeilen Rust-Code**: ~9.751 Zeilen (src + tests)
- **Hauptmodule**: 6 (apu, cartridge, cpu, mem, ppu, tooling)
- **CPU-Instruktionen**: 87 implementierte Opcodes in `src/cpu/instructions/ops/`
- **Binaries**: 3 (superrustendo, snesdbg, snesdis)

### Architektur

#### Module Struktur
```
src/
├── lib.rs              # Hauptbibliothek
├── main.rs             # Haupt-Emulator Binary
├── apu/                # Audio Processing Unit (nur mod.rs - Platzhalter)
│   └── mod.rs
├── cartridge/          # ROM-Ladefunktionalität
│   └── mod.rs
├── cpu/                # Central Processing Unit (65816)
│   ├── mod.rs          # Register, CPU-State
│   ├── address.rs      # Adressberechnungen
│   ├── addressmodes.rs # Adressmodi (26 Modi)
│   ├── constants.rs    # Opcode-Konstanten
│   ├── decoder.rs      # Opcode-Dekodierung
│   └── instructions/   # Instruktionsimplementierung
│       ├── mod.rs      # Instruction Struct + Execute
│       └── ops/        # 87 Opcode-Implementierungen
├── mem/                # Speicherverwaltung
│   ├── mod.rs          # Bus, Memory Map
│   ├── hw_registers.rs # Hardware-Register
│   └── wram.rs         # Work RAM
├── ppu/                # Picture Processing Unit
│   ├── mod.rs          # PPU Hauptlogik
│   └── sprite.rs       # Sprite-Handling
└── tooling/            # Hilfsprogramme
    ├── mod.rs
    └── disassembler.rs # Disassembler-Logik

bin/
├── snesdbg/            # Step-Debugger
│   └── main.rs
└── snesdis/            # Disassembler
    └── main.rs

tests/
├── decoder.rs          # Decoder-Tests
└── mod.rs              # Test-Module
```

### Implementierungsstatus

#### ✅ Implementiert
- **CPU-Kern**: Register (A, B, X, Y, D, S, PBR, DBR, PC, P)
- **Status-Register**: N, V, M, X, D, I, Z, C Flags
- **Adressmodi**: 26 verschiedene Modi (Immediate, Absolute, DirectPage, etc.)
- **Instruktionen**: 87 Opcodes implementiert
  - Arithmetik: ADC, SBC, INC, DEC, INX, INY, DEX, DEY
  - Logisch: AND, ORA, EOR, BIT, TRB, TSB
  - Lade/Speichere: LDA, LDX, LDY, STA, STX, STY, STZ
  - Stack: PHA, PHB, PHD, PHK, PHP, PHX, PHY, PLA, PLB, PLD, PLP, PLX, PLY
  - Sprünge: JMP, JSR, RTS, RTI, RTL, BRA, BRL, BCC, BCS, BEQ, BNE, BMI, BPL, BVC, BVS
  - Flag-Operationen: CLC, CLD, CLI, CLV, SEC, SED, SEI, REP, SEP
  - Transfers: TAX, TAY, TXA, TXY, TXS, TSX, TAY, TYA, TCD, TCS, TDC, TSC, XBA, XCE
  - Block-Moves: MVN, MVP
  - Sonstige: NOP, BRK, COP, STP, WAI, WDM, PER, PEA, PEI
- **Speicher**: Bus-Implementierung mit Cartridge-Loading
- **Cartridge**: ROM-Ladefunktionalität
- **Tools**: snesdis (Disassembler), snesdbg (Step-Debugger)

#### 🚧 Teilweise Implementiert
- **PPU**: Grundstruktur vorhanden, aber keine vollständige Emulation
- **APU**: Nur Platzhalter, keine Sound-Emulation
- **Memory Map**: Grundlegend, aber nicht alle Hardware-Register

#### ❌ Nicht Implementiert
- **PPU-Rendering**: Keine Grafikausgabe
- **APU-Sound**: Keine Audioausgabe
- **Input-Handling**: Keine Controller-Eingabe
- **Interrupts**: Nicht vollständig (NMI, IRQ)
- **DMA**: Direct Memory Access nicht implementiert
- **Save States**: Kein Speichern/Laden des Zustands
- **GUI**: Keine grafische Oberfläche

### Build-Status
- **`cargo check`**: ✅ Erfolgreich (mit 128 Warnungen)
- **`cargo build`**: ❌ Speicherprobleme mit minifb/wayland (OOM Error)
- **`cargo test`**: ❌ Speicherprobleme beim Kompilieren

### Hauptprobleme

1. **Build-Probleme**: 
   - `minifb` Dependency verursacht OOM beim Kompilieren
   - Wayland-Protokolle benötigen viel Speicher
   - Lösung: Optional Feature machen oder alternative Crate

2. **Code-Qualität**:
   - 128 Warnungen (unbenutzte Imports, Dead Code, etc.)
   - Keine konsistente Fehlerbehandlung
   - Teilweise unidiomatischer Rust-Code

3. **Architektur**:
   - Starke Kopplung zwischen CPU und Bus
   - Keine klare Trennung von Emulation und Tools
   - Keine Unit-Tests für viele Instruktionen

4. **Dokumentation**:
   - Keine Modul-Dokumentation
   - Keine API-Dokumentation
   - README veraltet

### Abhängigkeiten
```toml
[dependencies]
byte_struct = "0.7.1"    # Binary struct parsing
byteorder = "1.4.3"      # Byte order handling
hex = "0.4.3"           # Hex encoding/decoding
minifb = "0.25.0"       # Framebuffer für GUI (PROBLEMATISCH)
raw-string = "0.3.5"    # Raw string literals
rustyline = "9.0.0"     # CLI für Debugger
```

### Test-Roms
- Verzeichnis `snes-test-roms/` existiert aber ist **leer**
- Keine Test-ROMs für Regressionstests

---

## Empfehlungen für die weitere Entwicklung

### 1. Build-Probleme beheben (Priorität: HOCH)
- `minifb` als optional feature markieren
- Alternative: `pixels` oder `sdl2` Crate evaluieren
- CI-Umgebung anpassen

### 2. Code-Qualität verbessern (Priorität: HOCH)
- Alle Warnungen beheben
- `cargo clippy` einführen
- `cargo fmt` konsistent anwenden
- Unit-Tests erweitern

### 3. Architektur refaktorieren (Priorität: MITTEL)
- Klare Modulgrenzen definieren
- Trait-basierte Schnittstellen
- Fehlerbehandlung standardisieren

### 4. Dokumentation (Priorität: MITTEL)
- Modul-Dokumentation hinzufügen
- README aktualisieren
- Entwicklungs-Roadmap erstellen

### 5. Features implementieren (Priorität: NIEDRIG)
- PPU-Emulation vervollständigen
- APU-Emulation hinzufügen
- Input-Handling implementieren
- Save States
