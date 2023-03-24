use rust_nes::detail::pad::Pad;
use rust_nes::core::nes_pad::PadButton;

fn main() {
    let mut pad: Pad = Default::default();

    // Neutral
    println!("{:?}", pad);

    // Press A
    pad.push_button(PadButton::A);
    println!("{:?}", pad);

    // Press B
    pad.push_button(PadButton::B);
    println!("{:?}", pad);

    // Release A
    pad.release_button(PadButton::A);

    // Press B
    pad.push_button(PadButton::B);
    println!("{:?}", pad);
}
