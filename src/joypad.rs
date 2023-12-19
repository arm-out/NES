bitflags! {
    #[derive(Copy, Clone)]
    pub struct JoypadButtons: u8 {
        const Right = 0b10000000;
        const Left = 0b01000000;
        const Down = 0b00100000;
        const Up = 0b00010000;
        const Start = 0b00001000;
        const Select = 0b00000100;
        const ButtonB = 0b00000010;
        const ButtonA = 0b00000001;
    }
}

pub struct Joypad {
    strobe: bool,
    button_index: u8,
    button_status: JoypadButtons,
}

impl Joypad {
    pub fn new() -> Self {
        Joypad {
            strobe: false,
            button_index: 0,
            button_status: JoypadButtons::from_bits_truncate(0),
        }
    }

    pub fn write(&mut self, data: u8) {
        self.strobe = data & 1 == 1;
        if self.strobe {
            self.button_index = 0
        }
    }

    pub fn read(&mut self) -> u8 {
        if self.button_index > 7 {
            return 1;
        }

        let response = (self.button_status.bits() & (1 << self.button_index)) >> self.button_index;
        if !self.strobe && self.button_index <= 7 {
            self.button_index += 1;
        }

        response
    }

    pub fn set_button_pressed_status(&mut self, button: JoypadButtons, pressed: bool) {
        self.button_status.set(button, pressed);
    }
}
