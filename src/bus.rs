use crate::cartridge::Rom;
use crate::cpu::Mem;

const RAM: u16 = 0x0000;
const RAM_MIRROR_END: u16 = 0x1FFF;
const PPU_REGISTERS: u16 = 0x2000;
const PPU_REGISTERS_MIRROR_END: u16 = 0x3FFF;
const PRG_ROM_START: u16 = 0x8000;
const PRG_ROM_END: u16 = 0xFFFF;

pub struct Bus {
    cpu_vram: [u8; 2048],
    rom: Rom,
}

impl Bus {
    pub fn new(rom: Rom) -> Self {
        Self {
            cpu_vram: [0; 2048],
            rom,
        }
    }

    fn read_prg_rom(&self, mut addr: u16) -> u8 {
        addr -= 0x8000;
        if self.rom.prg_rom.len() == 0x4000 && addr >= 0x4000 {
            addr = addr % 0x4000;
        }
        self.rom.prg_rom[addr as usize]
    }
}

impl Mem for Bus {
    fn mem_read(&self, addr: u16) -> u8 {
        match addr {
            RAM..=RAM_MIRROR_END => {
                let mirr_addr = addr & 0b00000111_11111111;
                self.cpu_vram[mirr_addr as usize]
            }

            PPU_REGISTERS..=PPU_REGISTERS_MIRROR_END => {
                let mirr_addr = addr & 0b00100000_00000111;
                todo!("Read from PPU registers: {:#X}", mirr_addr)
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

            PPU_REGISTERS..=PPU_REGISTERS_MIRROR_END => {
                let mirr_addr = addr & 0b00100000_00000111;
                todo!("Read from PPU registers: {:#X}", mirr_addr)
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
