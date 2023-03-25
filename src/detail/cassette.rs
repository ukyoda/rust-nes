use super::constants;
use std::{mem, default};

extern crate static_assertions as sa;

const PRG_ROM_UNIT: i32 = 16 * 1024;
const CHR_ROM_UNIT: i32 = 8 * 1024;

enum Mirroring {
    Horizontal,
    Vertical
}

#[repr(C)]
#[derive(Debug)]
struct INESHeader {
    pub magic: [u8; 4],
    // 16KiB 単位の PRG ROM のサイズ
    pub prg_rom_size: u8,
    // 8KiB 単位の CHR ROM のサイズ
    pub chr_rom_size: u8,
    // TODO: フラグの中身を調査する
    pub flags6: u8,
    pub flags7: u8,
    pub flags8: u8,
    pub flags9: u8,
    pub flags10: u8,

    pub padding: [u8; 5]
}

impl Default for INESHeader {
    #[inline]
    fn default() -> INESHeader {
        INESHeader { 
            magic: [0; 4], 
            prg_rom_size: 0, 
            chr_rom_size: 0, 
            flags6: 0,
            flags7: 0,
            flags8: 0,
            flags9: 0,
            flags10: 0,
            padding: [0; 5] 
        }
    }
}


#[test]
fn check_the_sizeof_inesheader() {
    assert_eq!(16, mem::size_of::<INESHeader>(), "aaa");
}

#[derive(Debug)]
pub struct Cassette {
    prg_rom: [u8; constants::PRG_ROM_MAX],
    chr_rom: [u8; constants::CHR_ROM_MAX],
    initialized: bool,
    header: INESHeader,
    // TODO: usizeを使うのはもしかしたら問題があるかも
    //       https://internals.rust-lang.org/t/pre-rfc-usize-is-not-size-t/15369
    prg_rom_size: usize,
    chr_rom_size: usize,
}

impl Default for Cassette {
    #[inline]
    fn default() -> Cassette {
        Cassette {
            prg_rom: [0; constants::PRG_ROM_MAX],
            chr_rom: [0; constants::CHR_ROM_MAX],
            initialized: false,
            header: Default::default(),
            prg_rom_size: 0,
            chr_rom_size: 0,
        }
    }
}

impl Cassette {
    pub fn initialize(&self, p_buffer: u8, buffer_size: usize) {

    }
}