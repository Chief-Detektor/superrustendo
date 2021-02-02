pub const PAL_RES: (u16, u8) = (256, 240);
pub const NTSC_RES: (u16, u8) = (256, 224);

#[derive(Debug)]
pub enum SystemType {
    PAL,
    NTSC,
}

#[derive(Debug)]
struct PPUMem<T> {
    ram: Vec<T>,
    address: usize,
}

impl<T> PPUMem<T> {
    pub fn new(size: usize) -> Self {
        PPUMem {
            ram: Vec::with_capacity(size),
            address: 0,
        }
    }
}

// TODO: store current oam, cgram and vram addresses!
#[derive(Debug)]
pub struct PPU {
    vram: PPUMem<u8>,   //[u8; 0x10000], // 64 KB vram
    cgram: PPUMem<u16>, //[u16; 256], // 512 byte cgram
    oam: PPUMem<u8>,    //[u8; 544],    // 544 byte oam
    sys_hw: SystemType,
}

// TODO: VRAM
impl PPU {
    pub fn new(sys_hw: SystemType) -> Self {
        PPU {
            vram: PPUMem::new(0x10000),
            cgram: PPUMem::new(256),
            oam: PPUMem::new(544),
            sys_hw,
        }
    }

    pub fn init_display(&self, byte: u8) {
        /*
        2100  wb++++ INIDISP - Screen Display
                x---bbbb

                x    = Force blank on when set.
                bbbb = Screen brightness, F=max, 0="off".

                Note that force blank CAN be disabled mid-scanline. However, this can
                result in glitched graphics on that scanline, as the internal rendering
                buffers will not have been updated during force blank. Current theory
                is that BGs will be glitched for a few tiles (depending on how far in
                advance the PPU operates), and OBJ will be glitched for the entire
                scanline.

                Also, writing this register on the first line of V-Blank (225 or 240,
                depending on overscan) when force blank is currently active causes the
                OAM Address Reset to occur.
                */
        let force_blank = (byte & 0b10000000) >> 7 != 0;
        let screen_brightness = byte & 0x0f;
        println!(
            "[PPU]: INIDISP (force blank: {}, brightness: {:x}",
            force_blank, screen_brightness
        );
    }
}
