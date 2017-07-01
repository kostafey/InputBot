use Code;
use linux::x11::*;
use linux::x11::xlib::*;
use linux::get_display2;
use *;

fn get_code(keysym: u8) -> u8 {
    unsafe{XKeysymToKeycode(get_display2(), keysym as _)}
}

lazy_static! {
pub static ref LBUTTON: Code = {get_code(0x01)};
pub static ref RBUTTON: Code = {get_code(0x02)};
pub static ref MBUTTON: Code = {get_code(0x04)};
pub static ref XBUTTON1: Code = {get_code(0x05)};
pub static ref XBUTTON2: Code = {get_code(0x06)};
pub static ref BACKSPACE: Code = {get_code(0x08)};
pub static ref TAB: Code = {get_code(0x09)};
pub static ref ENTER: Code = {get_code(0x0D)};
pub static ref SHIFT: Code = {get_code(0x10)};
pub static ref CONTROL: Code = {get_code(0x11)};
pub static ref ESCAPE: Code = {get_code(0x1B)};
pub static ref SPACE: Code = {get_code(0x20)};
pub static ref HOME: Code = {get_code(0x24)};
pub static ref LEFT: Code = {get_code(0x25)};
pub static ref UP: Code = {get_code(0x26)};
pub static ref RIGHT: Code = {get_code(0x27)};
pub static ref DOWN: Code = {get_code(0x28)};
pub static ref INSERT: Code = {get_code(0x2D)};
pub static ref DELETE: Code = {get_code(0x2E)};
pub static ref NUM_0: Code = {get_code(0x30)};
pub static ref NUM_1: Code = {get_code(0x31)};
pub static ref NUM_2: Code = {get_code(0x32)};
pub static ref NUM_3: Code = {get_code(0x33)};
pub static ref NUM_4: Code = {get_code(0x34)};
pub static ref NUM_5: Code = {get_code(0x35)};
pub static ref NUM_6: Code = {get_code(0x36)};
pub static ref NUM_7: Code = {get_code(0x37)};
pub static ref NUM_8: Code = {get_code(0x38)};
pub static ref NUM_9: Code = {get_code(0x39)};
pub static ref A: Code = {get_code(0x41)};
pub static ref B: Code = {get_code(0x42)};
pub static ref C: Code = {get_code(0x43)};
pub static ref D: Code = {get_code(0x44)};
pub static ref E: Code = {get_code(0x45)};
pub static ref F: Code = {get_code(0x46)};
pub static ref G: Code = {get_code(0x47)};
pub static ref H: Code = {get_code(0x48)};
pub static ref I: Code = {get_code(0x49)};
pub static ref J: Code = {get_code(0x4A)};
pub static ref K: Code = {get_code(0x4B)};
pub static ref L: Code = {get_code(0x4C)};
pub static ref M: Code = {get_code(0x4D)};
pub static ref N: Code = {get_code(0x4E)};
pub static ref O: Code = {get_code(0x4f)};
pub static ref P: Code = {get_code(0x50)};
pub static ref Q: Code = {get_code(0x51)};
pub static ref R: Code = {get_code(0x52)};
pub static ref S: Code = {get_code(0x53)};
pub static ref T: Code = {get_code(0x54)};
pub static ref U: Code = {get_code(0x55)};
pub static ref V: Code = {get_code(0x56)};
pub static ref W: Code = {get_code(0x57)};
pub static ref X: Code = {get_code(0x58)};
pub static ref Y: Code = {get_code(0x59)};
pub static ref Z: Code = {get_code(0x5A)};
pub static ref NUMPAD_0: Code = {get_code(0x60)};
pub static ref NUMPAD_1: Code = {get_code(0x61)};
pub static ref NUMPAD_2: Code = {get_code(0x62)};
pub static ref NUMPAD_3: Code = {get_code(0x63)};
pub static ref NUMPAD_4: Code = {get_code(0x64)};
pub static ref NUMPAD_5: Code = {get_code(0x65)};
pub static ref NUMPAD_6: Code = {get_code(0x66)};
pub static ref NUMPAD_7: Code = {get_code(0x67)};
pub static ref NUMPAD_8: Code = {get_code(0x68)};
pub static ref NUMPAD_9: Code = {get_code(0x69)};
pub static ref F1: Code = {get_code(0x70)};
pub static ref F2: Code = {get_code(0x71)};
pub static ref F3: Code = {get_code(0x72)};
pub static ref F4: Code = {get_code(0x73)};
pub static ref F5: Code = {get_code(0x74)};
pub static ref F6: Code = {get_code(0x75)};
pub static ref F7: Code = {get_code(0x76)};
pub static ref F8: Code = {get_code(0x77)};
pub static ref F9: Code = {get_code(0x78)};
pub static ref F10: Code = {get_code(0x79)};
pub static ref F11: Code = {get_code(0x7A)};
pub static ref F12: Code = {get_code(0x7B)};
pub static ref F13: Code = {get_code(0x7C)};
pub static ref F14: Code = {get_code(0x7D)};
pub static ref F15: Code = {get_code(0x7E)};
pub static ref F16: Code = {get_code(0x7f)};
pub static ref F17: Code = {get_code(0x80)};
pub static ref F18: Code = {get_code(0x81)};
pub static ref F19: Code = {get_code(0x82)};
pub static ref F20: Code = {get_code(0x83)};
pub static ref F21: Code = {get_code(0x84)};
pub static ref F22: Code = {get_code(0x85)};
pub static ref F23: Code = {get_code(0x86)};
pub static ref F24: Code = {get_code(0x87)};
pub static ref NUM_LOCK: Code = {get_code(0x7F)};
pub static ref SCROLL_LOCK: Code = {get_code(0x91)};
pub static ref LSHIFT: Code = {get_code(0xA0)};
pub static ref RSHIFT: Code = {get_code(0xA1)};
pub static ref LCONTROL: Code = {get_code(0xA2)};
pub static ref RCONTROL: Code = {get_code(0xA3)};
}