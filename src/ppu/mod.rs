use self::registers::{
    addr::AddrRegister, ctrl::CtrlRegister, mask::MaskRegister, scroll::ScrollRegister,
    status::StatusRegister,
};
use crate::cartridge::Mirroring;

pub mod registers;

pub struct NesPPU {
    pub chr_rom: Vec<u8>,
    pub palette_table: [u8; 32],
    pub vram: [u8; 2048],
    pub oam: [u8; 256],
    pub mirroring: Mirroring,
    internal_data_buf: u8,

    // Registers
    pub ctrl: CtrlRegister,
    pub mask: MaskRegister,
    pub status: StatusRegister,
    pub oam_addr: u8,
    pub scroll: ScrollRegister,
    pub addr: AddrRegister,
}

pub trait PPU {
    fn write_to_ppu_addr(&mut self, value: u8);
    fn write_to_ctrl(&mut self, value: u8);
    fn read_data(&mut self) -> u8;
    fn write_to_mask(&mut self, value: u8);
    fn read_status(&mut self) -> u8;
    fn write_to_oam_addr(&mut self, value: u8);
    fn write_to_oam_data(&mut self, value: u8);
    fn read_oam_data(&self) -> u8;
    fn write_to_scroll(&mut self, value: u8);
    fn write_to_data(&mut self, value: u8);
    fn write_oam_dma(&mut self, value: &[u8; 256]);
}

impl NesPPU {
    pub fn new(chr_rom: Vec<u8>, mirroring: Mirroring) -> Self {
        Self {
            chr_rom,
            palette_table: [0; 32],
            vram: [0; 2048],
            oam: [0; 64 * 4],
            mirroring,
            internal_data_buf: 0,
            addr: AddrRegister::new(),
            ctrl: CtrlRegister::new(),
            mask: MaskRegister::new(),
            status: StatusRegister::new(),
            scroll: ScrollRegister::new(),
            oam_addr: 0,
        }
    }

    fn increment_vram_addr(&mut self) {
        self.addr.increment(self.ctrl.vram_addr_increment());
    }

    // Horizontal:
    // A a
    // B a
    // Vertical:
    // A B
    // a b
    pub fn mirror_vram_addr(&self, addr: u16) -> u16 {
        // Mirror 0x3000..0x3eff to 0x2000..0x2eff
        let mirrored_addr = addr & 0b101111_11111111;
        let vram_idx = mirrored_addr - 0x2000;
        let nametable = vram_idx / 0x0400;

        match (&self.mirroring, nametable) {
            (Mirroring::Vertical, 2) | (Mirroring::Vertical, 3) => vram_idx - 0x800,
            (Mirroring::Horizontal, 2) => vram_idx - 0x400,
            (Mirroring::Horizontal, 1) => vram_idx - 0x400,
            (Mirroring::Horizontal, 3) => vram_idx - 0x800,
            _ => vram_idx,
        }
    }
}

impl PPU for NesPPU {
    fn write_to_ppu_addr(&mut self, data: u8) {
        self.addr.update(data);
    }

    fn write_to_ctrl(&mut self, data: u8) {
        self.ctrl.update(data);
    }

    fn read_data(&mut self) -> u8 {
        let addr = self.addr.get();
        self.increment_vram_addr();

        match addr {
            0x0000..=0x1FFF => {
                // Return buffered value first and then return data on next read
                let result = self.internal_data_buf;
                self.internal_data_buf = self.chr_rom[addr as usize];
                result
            }
            0x2000..=0x2FFF => {
                // Return buffered value first and then return data on next read
                let result = self.internal_data_buf;
                self.internal_data_buf = self.vram[self.mirror_vram_addr(addr) as usize];
                result
            }
            0x3000..=0x3EFF => panic!(
                "Address space 0x3000..0x3eff is not expected to be used, requested = {:#X}",
                addr
            ),
            // Instant Access to Palette
            0x3F00..=0x3FFF => self.palette_table[(addr - 0x3f00) as usize],
            _ => panic!("unexpected access to mirrored space {}", addr),
        }
    }

    fn write_to_mask(&mut self, data: u8) {
        self.mask.update(data);
    }

    fn read_status(&mut self) -> u8 {
        let result = self.status.snapshot();
        self.status.reset_vblank_status();
        self.addr.reset_latch();
        self.scroll.reset_latch();
        result
    }

    fn write_to_oam_addr(&mut self, value: u8) {
        self.oam_addr = value;
    }

    fn write_to_oam_data(&mut self, value: u8) {
        self.oam[self.oam_addr as usize] = value;
        self.oam_addr = self.oam_addr.wrapping_add(1);
    }

    fn read_oam_data(&self) -> u8 {
        self.oam[self.oam_addr as usize]
    }

    fn write_to_scroll(&mut self, value: u8) {
        self.scroll.write(value);
    }

    fn write_oam_dma(&mut self, data: &[u8; 256]) {
        for x in data.iter() {
            self.oam[self.oam_addr as usize] = *x;
            self.oam_addr = self.oam_addr.wrapping_add(1);
        }
    }

    fn write_to_data(&mut self, value: u8) {
        let addr = self.addr.get();
        self.increment_vram_addr();

        match addr {
            0x0000..=0x1FFF => println!("Cannot write to CHR ROM"),
            0x2000..=0x2FFF => {
                self.vram[self.mirror_vram_addr(addr) as usize] = value;
            }
            0x3000..=0x3EFF => panic!(
                "Address space 0x3000..0x3eff is not expected to be used, requested = {:#X}",
                addr
            ),
            // Mirrors of $3F00/$3F04/$3F08/$3F0C
            0x3f10 | 0x3f14 | 0x3f18 | 0x3f1c => {
                let add_mirror = addr - 0x10;
                self.palette_table[(add_mirror - 0x3f00) as usize] = value;
            }
            0x3f00..=0x3fff => {
                self.palette_table[(addr - 0x3f00) as usize] = value;
            }
            _ => panic!("unexpected access to mirrored space {}", addr),
        }
    }
}
