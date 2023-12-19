bitflags! {
    // 7 6 5 4 3 2 1 0
    // B G R s b M m G
    // [B] Emphasize blue
    // [G] Emphasize green
    // [R] Emphasize red
    // [s] Show sprites
    // [b] Show background
    // [M] Show sprites in leftmost 8 pixels of screen
    // [m] Show background in leftmost 8 pixels of screen
    // [G] Greyscale

    pub struct MaskRegister: u8 {
        const Greyscale = 0b00000001;
        const Leftmost8pxlBackground = 0b00000010;
        const Leftmost8pxlSprite = 0b00000100;
        const ShowBackground = 0b00001000;
        const ShowSprites = 0b00010000;
        const EmphasizeRed = 0b00100000;
        const EmphasizeGreen = 0b01000000;
        const EmphasizeBlue = 0b10000000;
    }
}

pub enum Color {
    Red,
    Green,
    Blue,
}

impl MaskRegister {
    pub fn new() -> Self {
        MaskRegister::from_bits_truncate(0b00000000)
    }

    pub fn is_grayscale(&self) -> bool {
        self.contains(MaskRegister::Greyscale)
    }

    pub fn leftmost_8pxl_background(&self) -> bool {
        self.contains(MaskRegister::Leftmost8pxlBackground)
    }

    pub fn leftmost_8pxl_sprite(&self) -> bool {
        self.contains(MaskRegister::Leftmost8pxlSprite)
    }

    pub fn show_background(&self) -> bool {
        self.contains(MaskRegister::ShowBackground)
    }

    pub fn show_sprites(&self) -> bool {
        self.contains(MaskRegister::ShowSprites)
    }

    pub fn emphasise(&self) -> Vec<Color> {
        let mut result = Vec::<Color>::new();
        if self.contains(MaskRegister::EmphasizeRed) {
            result.push(Color::Red);
        }
        if self.contains(MaskRegister::EmphasizeGreen) {
            result.push(Color::Green);
        }
        if self.contains(MaskRegister::EmphasizeBlue) {
            result.push(Color::Blue);
        }
        result
    }

    pub fn update(&mut self, data: u8) {
        self.set(MaskRegister::Greyscale, data & 0b00000001 != 0);
        self.set(MaskRegister::Leftmost8pxlBackground, data & 0b00000010 != 0);
        self.set(MaskRegister::Leftmost8pxlSprite, data & 0b00000100 != 0);
        self.set(MaskRegister::ShowBackground, data & 0b00001000 != 0);
        self.set(MaskRegister::ShowSprites, data & 0b00010000 != 0);
        self.set(MaskRegister::EmphasizeRed, data & 0b00100000 != 0);
        self.set(MaskRegister::EmphasizeGreen, data & 0b01000000 != 0);
        self.set(MaskRegister::EmphasizeBlue, data & 0b10000000 != 0);
    }
}
