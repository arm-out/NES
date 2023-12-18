use crate::cpu::Mem;

const RAM: u16 = 0x0000;
const RAM_MIRROR_END: u16 = 0x1FFF;
const PPU_REGISTERS: u16 = 0x2000;
const PPU_REGISTERS_MIRROR_END: u16 = 0x3FFF;

pub struct Bus {
    cpu_vram: [u8; 2048],
}

impl Bus {
    pub fn new() -> Self {
        Self {
            cpu_vram: [0; 2048],
        }
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
            _ => {
                println!("Ignoring memory write at address: {:#X}", addr);
            }
        }
    }
}
