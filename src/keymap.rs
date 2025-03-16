use rmk::action::KeyAction;
use rmk::keycode::ModifierCombination;
use rmk::{a, k, tt, wm};
pub(crate) const COL: usize = 5;
pub(crate) const ROW: usize = 8;
pub(crate) const NUM_LAYER: usize = 4;
pub(crate) const SIZE: usize = 6;
#[rustfmt::skip]
pub const fn get_default_keymap() -> [[[KeyAction; COL]; ROW]; NUM_LAYER] {

    let lshift = ModifierCombination::new_from(false, false, false, true, false);

    [
        [
            [k!(Q), k!(W), k!(E), k!(R), k!(T)],
            [k!(Bootloader), k!(Reboot), k!(D), k!(F), k!(G)],
            [k!(Z), k!(X), k!(C), k!(V), k!(B)],
            [a!(No), a!(No), k!(Escape), tt!(1), k!(Enter)],
            [k!(Y), k!(U), k!(I), k!(O), k!(P)],
            [k!(H), k!(J), k!(K), k!(L), k!(Semicolon)],
            [k!(N), k!(M), k!(Comma), k!(Dot), k!(Enter)],
            [k!(Space), k!(Backspace), tt!(2), a!(No), a!(No)],
        ],
        [
            [k!(KbMute), wm!(Kc1, lshift), wm!(Kc2, lshift), wm!(Kc3, lshift), a!(No),],
            [k!(KbVolumeUp), wm!(Kc4, lshift), wm!(Kc5, lshift), wm!(Kc6, lshift), a!(No)],
            [k!(KbVolumeDown), wm!(Kc7, lshift), wm!(Kc8, lshift), a!(No), a!(No)],
            [a!(No), a!(No), k!(Escape), a!(No), k!(Enter)],
            [a!(No), k!(BrightnessDown), k!(BrightnessUp), a!(No), a!(No)],
            [k!(Left), k!(Down), k!(UP), k!(Right), wm!(Quote, lshift)],
            [a!(No), k!(Semicolon), wm!(Semicolon, lshift), a!(No), k!(Quote)],
            [k!(Space), k!(Backspace), tt!(2), a!(No), a!(No)],
        ],
        [
            [wm!(Kc9, lshift), k!(Kc1), k!(Kc2), k!(Kc3), wm!(Kc0, lshift)],
            [k!(LeftBracket), k!(Kc4), k!(Kc5), k!(Kc6), k!(RightBracket)],
            [wm!(LeftBracket, lshift), k!(Kc7), k!(Kc8), k!(Kc9), wm!(RightBracket, lshift), ],
            [a!(No), a!(No), k!(Escape), tt!(1), k!(Enter)],
            [k!(F1), k!(F2), k!(F5), k!(F11), k!(F12)],
            [ k!(Kc0), k!(Slash), k!(Backslash), a!(No), k!(Enter)],
            [a!(No), k!(Minus), wm!(Equal, lshift), wm!(Minus, lshift), k!(Equal)],
            [k!(Space), k!(Backspace), a!(No), a!(No), a!(No)],
        ],
        [
            [a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), k!(OutputUsb), k!(OutputBluetooth), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No)],
        ]
    ]
}
