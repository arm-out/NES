use crate::cartridge::Rom;
use crate::cpu::Mem;
use crate::ppu::{NesPPU, PPU};

const RAM: u16 = 0x0000;
const RAM_MIRROR_END: u16 = 0x1FFF;
const PPU_REGISTERS_MIRROR_START: u16 = 0x2008;
const PPU_REGISTERS_MIRROR_END: u16 = 0x3FFF;
const PRG_ROM_START: u16 = 0x8000;
const PRG_ROM_END: u16 = 0xFFFF;

pub struct Bus {
    cpu_vram: [u8; 2048],
    prg_rom: Vec<u8>,
    ppu: NesPPU,

    cycles: usize,
}

impl Bus {
    pub fn new(rom: Rom) -> Self {
        let ppu = NesPPU::new(rom.chr_rom, rom.mirroring);
        Self {
            cpu_vram: [0; 2048],
            prg_rom: rom.prg_rom,
            ppu,
            cycles: 0,
        }
    }

    pub fn tick(&mut self, cycles: u8) {
        self.cycles += cycles as usize;
        self.ppu.tick(cycles * 3);
    }

    pub fn poll_nmi_status(&mut self) -> Option<u8> {
        self.ppu.nmi_interrupt.take()
    }

    fn read_prg_rom(&self, mut addr: u16) -> u8 {
        addr -= 0x8000;
        if self.prg_rom.len() == 0x4000 && addr >= 0x4000 {
            addr = addr % 0x4000;
        }
        self.prg_rom[addr as usize]
    }
}

impl Mem for Bus {
    fn mem_read(&mut self, addr: u16) -> u8 {
        match addr {
            RAM..=RAM_MIRROR_END => {
                let mirr_addr = addr & 0b00000111_11111111;
                self.cpu_vram[mirr_addr as usize]
            }
            // CTRL | MASK | OAMADDR | SCROLL | PPUADDR | OAMDMA
            0x2000 | 0x2001 | 0x2003 | 0x2005 | 0x2006 | 0x4014 => {
                panic!("Read from write-only PPU register: {:#X}", addr)
            }

            0x2002 => self.ppu.read_status(),   // STATUS
            0x2004 => self.ppu.read_oam_data(), // OAMDATA
            0x2007 => self.ppu.read_data(),     // DATA

            PPU_REGISTERS_MIRROR_START..=PPU_REGISTERS_MIRROR_END => {
                let mirr_addr = addr & 0b00100000_00000111;
                self.mem_read(mirr_addr)
            }

            PRG_ROM_START..=PRG_ROM_END => self.read_prg_rom(addr),

            _ => {
                println!("Ignoring memory read at address: {:#X}", addr);
                0
            }
        }
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        match addr {
            RAM..=RAM_MIRROR_END => {
                let mirr_addr = addr & 0b11111111111;
                self.cpu_vram[mirr_addr as usize] = data;
            }

            /* Registers */
            0x2000 => self.ppu.write_to_ctrl(data), // CTRL
            0x2001 => self.ppu.write_to_mask(data), // MASK
            0x2002 => panic!("Cannot write to status register"),
            0x2003 => self.ppu.write_to_oam_addr(data), // OAMADDR
            0x2004 => self.ppu.write_to_oam_data(data), // OAMDATA
            0x2005 => self.ppu.write_to_scroll(data),   // SCROLL
            0x2006 => self.ppu.write_to_ppu_addr(data), // PPUADDR
            0x2007 => self.ppu.write_to_data(data),     // DATA

            PPU_REGISTERS_MIRROR_START..=PPU_REGISTERS_MIRROR_END => {
                let mirr_addr = addr & 0b00100000_00000111;
                self.mem_write(mirr_addr, data)
            }

            PRG_ROM_START..=PRG_ROM_END => {
                panic!("Attempting to write to Cartridge ROM: {:#X}", addr);
            }

            _ => {
                println!("Ignoring memory write at address: {:#X}", addr);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::cartridge::test;

    #[test]
    fn test_mem_read_write_to_ram() {
        let mut bus = Bus::new(test::test_rom());
        bus.mem_write(0x01, 0x55);
        assert_eq!(bus.mem_read(0x01), 0x55);
    }
}
