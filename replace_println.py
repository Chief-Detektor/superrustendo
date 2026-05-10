#!/usr/bin/env python3
"""
Skript zum Ersetzen von println!-Aufrufen durch log-Macros in Rust-Dateien.
"""

import os
import re
import sys
from pathlib import Path

# Verzeichnis, in dem gesucht wird
ROOT_DIR = Path("/workspace/superrustendo/src")

# Dateien, die bereits bearbeitet wurden
ALREADY_PROCESSED = {
    "main.rs",
    "bin/snesdbg/main.rs",
    "bin/snesdis/main.rs",
    "cartridge/mod.rs",
}

# Log-Level-Mapping basierend auf dem Kontext
# Standard: debug für die meisten println!-Aufrufe
DEFAULT_LOG_LEVEL = "debug"

# Spezifische Muster für verschiedene Log-Levels
LOG_LEVEL_PATTERNS = {
    r'error|fail|invalid|wrong|problem': 'error',
    r'warn|warning|caution': 'warn',
    r'info|status|loaded|success|done|complete': 'info',
    r'trace|detail|verbose': 'trace',
}

def get_log_level(line: str) -> str:
    """Bestimmt das Log-Level basierend auf dem Inhalt der Zeile."""
    line_lower = line.lower()
    for pattern, level in LOG_LEVEL_PATTERNS.items():
        if re.search(pattern, line_lower):
            return level
    return DEFAULT_LOG_LEVEL

def add_log_import(content: str) -> str:
    """Fügt den log-Import hinzu, falls nicht bereits vorhanden."""
    if 'use log;' in content or 'use log::' in content:
        return content
    
    # Finde die letzte use-Deklaration
    lines = content.split('\n')
    last_use_idx = -1
    for i, line in enumerate(lines):
        if line.strip().startswith('use ') and not line.strip().startswith('use super::'):
            last_use_idx = i
    
    if last_use_idx >= 0:
        # Füge nach der letzten use-Deklaration ein
        lines.insert(last_use_idx + 1, 'use log;')
    else:
        # Füge am Anfang nach den Kommentaren ein
        for i, line in enumerate(lines):
            if line.strip() and not line.strip().startswith('//') and not line.strip().startswith('#!'):
                lines.insert(i, 'use log;')
                break
        else:
            lines.append('use log;')
    
    return '\n'.join(lines)

def replace_println_in_line(line: str) -> str:
    """Ersetzt println!-Aufrufe in einer einzelnen Zeile."""
    # Überspringe kommentierte println!-Aufrufe
    if line.strip().startswith('//'):
        return line
    
    # Ersetze println! durch log::debug!
    # Match println! mit verschiedenen Argumenten
    pattern = r'println!\s*\((.*?)\)'
    
    def replace_match(match):
        args = match.group(1)
        # Bestimme das Log-Level
        level = get_log_level(args)
        return f'log::{level}!({args})'
    
    return re.sub(pattern, replace_match, line)

def process_file(file_path: Path) -> bool:
    """Verarbeitet eine einzelne Datei."""
    with open(file_path, 'r', encoding='utf-8') as f:
        content = f.read()
    
    # Prüfe, ob die Datei println!-Aufrufe enthält
    if 'println!' not in content:
        return False
    
    # Prüfe, ob alle println!-Aufrufe kommentiert sind
    non_commented_println = False
    for line in content.split('\n'):
        stripped = line.strip()
        if 'println!' in stripped and not stripped.startswith('//'):
            non_commented_println = True
            break
    
    if not non_commented_println:
        return False
    
    # Füge log-Import hinzu
    content = add_log_import(content)
    
    # Ersetze alle println!-Aufrufe
    lines = content.split('\n')
    new_lines = []
    for line in lines:
        new_line = replace_println_in_line(line)
        new_lines.append(new_line)
    
    new_content = '\n'.join(new_lines)
    
    # Schreibe nur, wenn sich etwas geändert hat
    if new_content != content:
        with open(file_path, 'w', encoding='utf-8') as f:
            f.write(new_content)
        return True
    
    return False

def main():
    """Hauptfunktion."""
    processed_files = []
    
    # Finde alle Rust-Dateien
    rust_files = []
    for root, dirs, files in os.walk(ROOT_DIR):
        for file in files:
            if file.endswith('.rs'):
                rust_files.append(Path(root) / file)
    
    # Verarbeite jede Datei
    for file_path in rust_files:
        rel_path = file_path.relative_to(ROOT_DIR)
        file_name = str(rel_path)
        
        # Überspringe bereits bearbeitete Dateien
        if file_name in ALREADY_PROCESSED or 'bin/' in file_name:
            continue
        
        if process_file(file_path):
            processed_files.append(file_name)
            print(f"✓ Bearbeitet: {file_name}")
    
    print(f"\nInsgesamt bearbeitet: {len(processed_files)} Dateien")
    if processed_files:
        print("Dateien:")
        for f in processed_files:
            print(f"  - {f}")

if __name__ == '__main__':
    main()
