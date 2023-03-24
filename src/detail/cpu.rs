
fn is_singled_overflowed(n: u8, m: u8, c: bool) -> bool {
    let res = n + m + c;
    let res: bool = ((m ^ res) & (n ^ res) & 0x80) == 0x80;
    return res;
}

fn get_tows_complement(n: u8) -> u8 {
    !n + 1
}