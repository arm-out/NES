bitflags! {
    // 7  bit  0
    // ---- ----
    // VPHB SINN
    // |||| ||||
    // |||| ||++- Base nametable address
    // |||| ||    (0 = $2000; 1 = $2400; 2 = $2800; 3 = $2C00)
    // |||| |+--- VRAM address increment per CPU read/write of PPUDATA
    // |||| |     (0: add 1, going across; 1: add 32, going down)
    // |||| +---- Sprite pattern table address for 8x8 sprites
    // ||||       (0: $0000; 1: $1000; ignored in 8x16 mode)
    // |||+------ Background pattern table address (0: $0000; 1: $1000)
    // ||+------- Sprite size (0: 8x8 pixels; 1: 8x16 pixels – see PPU OAM#Byte 1)
    // |+-------- PPU master/slave select
    // |          (0: read backdrop from EXT pins; 1: output color on EXT pins)
    // +--------- Generate an NMI at the start of the
    //            vertical blanking interval (0: off; 1: on)

    pub struct CtrlRegister: u8 {
        const Nametable1            = 0b00000001;
        const Nametable2            = 0b00000010;
        const VramInc               = 0b00000100;
        const SpritePatternTable    = 0b00001000;
        const BgPatternTable        = 0b00010000;
        const SpriteSize            = 0b00100000;
        const MasterSlave           = 0b01000000;
        const NMI                   = 0b10000000;
    }
}

impl CtrlRegister {
    pub fn new() -> Self {
        Self::from_bits_truncate(0b00000000)
    }

    pub fn nametable_addr(&self) -> u16 {
        match self.bits() & 0b11 {
            0 => 0x2000,
            1 => 0x2400,
            2 => 0x2800,
            3 => 0x2c00,
            _ => panic!("not possible"),
        }
    }

    pub fn vram_addr_increment(&self) -> u8 {
        if !self.contains(CtrlRegister::VramInc) {
            1
        } else {
            32
        }
    }

    pub fn sprt_pattern_addr(&self) -> u16 {
        if !self.contains(CtrlRegister::SpritePatternTable) {
            0
        } else {
            0x1000
        }
    }

    pub fn bknd_pattern_addr(&self) -> u16 {
        if !self.contains(CtrlRegister::BgPatternTable) {
            0
        } else {
            0x1000
        }
    }

    pub fn sprite_size(&self) -> u8 {
        if !self.contains(CtrlRegister::SpriteSize) {
            8
        } else {
            16
        }
    }

    pub fn master_slave_select(&self) -> u8 {
        if !self.contains(CtrlRegister::SpriteSize) {
            0
        } else {
            1
        }
    }

    pub fn generate_vblank_nmi(&self) -> bool {
        return self.contains(CtrlRegister::NMI);
    }

    pub fn update(&mut self, data: u8) {
        self.set(CtrlRegister::Nametable1, data & 0b00000001 != 0);
        self.set(CtrlRegister::Nametable2, data & 0b00000010 != 0);
        self.set(CtrlRegister::VramInc, data & 0b00000100 != 0);
        self.set(CtrlRegister::SpritePatternTable, data & 0b00001000 != 0);
        self.set(CtrlRegister::BgPatternTable, data & 0b00010000 != 0);
        self.set(CtrlRegister::SpriteSize, data & 0b00100000 != 0);
        self.set(CtrlRegister::MasterSlave, data & 0b01000000 != 0);
        self.set(CtrlRegister::NMI, data & 0b10000000 != 0);
    }
}
