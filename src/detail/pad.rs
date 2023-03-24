use super::super::core::nes_pad::PadButton;

const BUTTON_A: u8 = 1;
const BUTTON_B: u8 = 2;
const BUTTON_SELECT: u8 = 4;
const BUTTON_START: u8 = 8;
const BUTTON_UP: u8 = 16;
const BUTTON_DOWN: u8 = 32;
const BUTTON_LEFT: u8 = 64;
const BUTTON_RIGHT: u8 = 128;


#[derive(Default, Debug)]
pub struct Pad {
    is_strobe_enable: bool,
    button_status: u8,
    read_idx: i32
}

impl Pad {
    pub fn default() -> Self {
        Self { is_strobe_enable: true, button_status: 0, read_idx: 0 }
    }

    pub fn push_button(&mut self, button: PadButton) {
        match button {
            PadButton::A => self.button_status |= BUTTON_A,
            PadButton::B => self.button_status |= BUTTON_B,
            PadButton::SELECT => self.button_status |= BUTTON_SELECT,
            PadButton::START => self.button_status |= BUTTON_START,
            PadButton::UP => self.button_status |= BUTTON_UP,
            PadButton::DOWN => self.button_status |= BUTTON_DOWN,
            PadButton::LEFT => self.button_status |= BUTTON_LEFT,
            PadButton::RIGHT => self.button_status |= BUTTON_RIGHT,
        }
    }

    pub fn release_button(&mut self, button: PadButton) {
        match button {
            PadButton::A => self.button_status &= !BUTTON_A,
            PadButton::B => self.button_status &= !BUTTON_B,
            PadButton::SELECT => self.button_status &= !BUTTON_SELECT,
            PadButton::START => self.button_status &= !BUTTON_START,
            PadButton::UP => self.button_status &= !BUTTON_UP,
            PadButton::DOWN => self.button_status &= !BUTTON_DOWN,
            PadButton::LEFT => self.button_status &= !BUTTON_LEFT,
            PadButton::RIGHT => self.button_status &= !BUTTON_RIGHT,
        }
    }

    pub fn read_pad(&mut self) -> u8 { 
        let ret: u8 = (self.button_status >> self.read_idx) & 1;
        if self.is_strobe_enable {
            self.read_idx += 1;
            self.read_idx %= 8;
        }
        ret
    }

    pub fn set_strobe(&mut self, flag: bool) {
        self.is_strobe_enable = flag;
        if flag {
            self.read_idx = 0;
        }
    }
}
