#![feature(prelude_import)]
#![no_main]
#![no_std]
#[prelude_import]
use core::prelude::rust_2021::*;
#[macro_use]
extern crate core;
use rmk::macros::rmk_central;
use panic_probe as _;
use defmt_rtt as _;
pub(crate) const COL: usize = 5usize;
pub(crate) const ROW: usize = 8usize;
pub(crate) const NUM_LAYER: usize = 4usize;
static KEYBOARD_USB_CONFIG: ::rmk::config::KeyboardUsbConfig = ::rmk::config::KeyboardUsbConfig {
    vid: 19531u16,
    pid: 17987u16,
    manufacturer: "drin",
    product_name: "awe-kbd",
    serial_number: "vial:f64c2b3c:000001",
};
pub const VIAL_KEYBOARD_DEF: &'static [u8] = &[
    253u8, 55u8, 122u8, 88u8, 90u8, 0u8, 0u8, 4u8, 230u8, 214u8, 180u8, 70u8, 2u8, 0u8,
    33u8, 1u8, 22u8, 0u8, 0u8, 0u8, 116u8, 47u8, 229u8, 163u8, 224u8, 5u8, 22u8, 1u8,
    187u8, 93u8, 0u8, 61u8, 136u8, 137u8, 198u8, 84u8, 54u8, 195u8, 23u8, 79u8, 228u8,
    219u8, 151u8, 45u8, 6u8, 63u8, 159u8, 120u8, 202u8, 229u8, 32u8, 59u8, 78u8, 85u8,
    107u8, 22u8, 29u8, 77u8, 52u8, 52u8, 114u8, 146u8, 109u8, 55u8, 224u8, 247u8, 239u8,
    113u8, 120u8, 166u8, 31u8, 101u8, 139u8, 47u8, 16u8, 110u8, 132u8, 61u8, 92u8, 108u8,
    124u8, 187u8, 100u8, 114u8, 230u8, 115u8, 251u8, 153u8, 115u8, 25u8, 90u8, 36u8,
    113u8, 231u8, 247u8, 210u8, 142u8, 51u8, 110u8, 224u8, 146u8, 7u8, 240u8, 164u8,
    151u8, 151u8, 90u8, 35u8, 55u8, 23u8, 183u8, 180u8, 155u8, 205u8, 50u8, 112u8, 167u8,
    86u8, 74u8, 139u8, 247u8, 168u8, 87u8, 148u8, 101u8, 144u8, 217u8, 38u8, 242u8,
    203u8, 202u8, 139u8, 144u8, 88u8, 250u8, 37u8, 10u8, 226u8, 53u8, 63u8, 58u8, 108u8,
    142u8, 129u8, 18u8, 230u8, 129u8, 109u8, 52u8, 166u8, 113u8, 215u8, 32u8, 221u8,
    63u8, 80u8, 240u8, 204u8, 125u8, 150u8, 161u8, 231u8, 116u8, 6u8, 255u8, 128u8,
    110u8, 228u8, 84u8, 56u8, 95u8, 227u8, 124u8, 57u8, 30u8, 38u8, 110u8, 16u8, 137u8,
    144u8, 55u8, 26u8, 67u8, 183u8, 85u8, 58u8, 147u8, 181u8, 77u8, 181u8, 91u8, 188u8,
    154u8, 210u8, 251u8, 188u8, 228u8, 208u8, 235u8, 103u8, 73u8, 235u8, 52u8, 126u8,
    166u8, 176u8, 138u8, 192u8, 121u8, 82u8, 95u8, 227u8, 234u8, 7u8, 71u8, 200u8, 137u8,
    153u8, 190u8, 99u8, 8u8, 202u8, 245u8, 144u8, 253u8, 162u8, 76u8, 56u8, 211u8, 53u8,
    198u8, 194u8, 237u8, 171u8, 237u8, 45u8, 254u8, 104u8, 71u8, 106u8, 207u8, 30u8,
    134u8, 53u8, 243u8, 32u8, 96u8, 133u8, 161u8, 176u8, 45u8, 245u8, 21u8, 46u8, 211u8,
    15u8, 157u8, 94u8, 94u8, 237u8, 86u8, 235u8, 187u8, 79u8, 33u8, 138u8, 155u8, 121u8,
    61u8, 236u8, 244u8, 32u8, 36u8, 202u8, 144u8, 98u8, 146u8, 53u8, 187u8, 3u8, 0u8,
    224u8, 177u8, 6u8, 14u8, 202u8, 233u8, 61u8, 195u8, 57u8, 130u8, 1u8, 246u8, 151u8,
    232u8, 155u8, 68u8, 213u8, 227u8, 118u8, 242u8, 92u8, 88u8, 157u8, 31u8, 13u8, 242u8,
    164u8, 210u8, 230u8, 236u8, 122u8, 62u8, 190u8, 112u8, 29u8, 242u8, 220u8, 247u8,
    119u8, 106u8, 20u8, 69u8, 72u8, 205u8, 134u8, 108u8, 229u8, 131u8, 62u8, 217u8,
    199u8, 179u8, 240u8, 243u8, 121u8, 33u8, 244u8, 119u8, 229u8, 141u8, 252u8, 175u8,
    255u8, 91u8, 227u8, 205u8, 106u8, 59u8, 56u8, 195u8, 149u8, 107u8, 149u8, 116u8,
    19u8, 55u8, 76u8, 140u8, 23u8, 55u8, 172u8, 109u8, 204u8, 142u8, 89u8, 164u8, 82u8,
    183u8, 58u8, 167u8, 175u8, 85u8, 136u8, 253u8, 103u8, 116u8, 139u8, 97u8, 193u8,
    11u8, 189u8, 15u8, 235u8, 12u8, 239u8, 15u8, 207u8, 92u8, 121u8, 134u8, 251u8, 198u8,
    185u8, 250u8, 154u8, 147u8, 180u8, 177u8, 194u8, 82u8, 147u8, 47u8, 71u8, 164u8,
    48u8, 104u8, 203u8, 251u8, 235u8, 173u8, 106u8, 5u8, 112u8, 11u8, 131u8, 129u8,
    178u8, 206u8, 78u8, 23u8, 160u8, 225u8, 235u8, 52u8, 26u8, 80u8, 163u8, 209u8, 56u8,
    206u8, 185u8, 28u8, 139u8, 207u8, 147u8, 217u8, 250u8, 202u8, 76u8, 112u8, 30u8,
    255u8, 174u8, 98u8, 163u8, 185u8, 217u8, 170u8, 199u8, 98u8, 29u8, 105u8, 152u8,
    48u8, 207u8, 157u8, 85u8, 154u8, 224u8, 98u8, 133u8, 243u8, 98u8, 163u8, 179u8,
    155u8, 26u8, 123u8, 174u8, 156u8, 21u8, 169u8, 108u8, 0u8, 0u8, 235u8, 95u8, 110u8,
    228u8, 4u8, 39u8, 4u8, 9u8, 0u8, 1u8, 215u8, 3u8, 151u8, 10u8, 0u8, 0u8, 81u8, 222u8,
    206u8, 166u8, 177u8, 196u8, 103u8, 251u8, 2u8, 0u8, 0u8, 0u8, 0u8, 4u8, 89u8, 90u8,
];
pub const VIAL_KEYBOARD_ID: &'static [u8] = &[
    185u8, 188u8, 9u8, 178u8, 157u8, 55u8, 76u8, 234u8,
];
static VIAL_CONFIG: ::rmk::config::VialConfig = ::rmk::config::VialConfig {
    vial_keyboard_id: &VIAL_KEYBOARD_ID,
    vial_keyboard_def: &VIAL_KEYBOARD_DEF,
};
pub const fn get_default_keymap() -> [[[::rmk::action::KeyAction; COL]; ROW]; NUM_LAYER] {
    [
        [
            [
                ::rmk::action::KeyAction::Single(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::Q),
                ),
                ::rmk::action::KeyAction::Single(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::W),
                ),
                ::rmk::action::KeyAction::Single(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::E),
                ),
                ::rmk::action::KeyAction::Single(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::R),
                ),
                ::rmk::action::KeyAction::Single(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::T),
                ),
            ],
            [
                ::rmk::action::KeyAction::Single(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::Bootloader),
                ),
                ::rmk::action::KeyAction::Single(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::Reboot),
                ),
                ::rmk::action::KeyAction::Single(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::D),
                ),
                ::rmk::action::KeyAction::Single(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::F),
                ),
                ::rmk::action::KeyAction::Single(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::G),
                ),
            ],
            [
                ::rmk::action::KeyAction::Single(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::Z),
                ),
                ::rmk::action::KeyAction::Single(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::X),
                ),
                ::rmk::action::KeyAction::Single(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::C),
                ),
                ::rmk::action::KeyAction::Single(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::V),
                ),
                ::rmk::action::KeyAction::Single(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::B),
                ),
            ],
            [
                ::rmk::action::KeyAction::No,
                ::rmk::action::KeyAction::No,
                ::rmk::action::KeyAction::Single(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::Escape),
                ),
                ::rmk::action::KeyAction::LayerTapHold(
                    ::rmk::action::Action::LayerToggle(1u8),
                    1u8,
                ),
                ::rmk::action::KeyAction::Single(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::Enter),
                ),
            ],
            [
                ::rmk::action::KeyAction::Single(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::Y),
                ),
                ::rmk::action::KeyAction::Single(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::U),
                ),
                ::rmk::action::KeyAction::Single(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::I),
                ),
                ::rmk::action::KeyAction::Single(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::O),
                ),
                ::rmk::action::KeyAction::Single(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::P),
                ),
            ],
            [
                ::rmk::action::KeyAction::Single(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::H),
                ),
                ::rmk::action::KeyAction::Single(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::J),
                ),
                ::rmk::action::KeyAction::Single(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::K),
                ),
                ::rmk::action::KeyAction::Single(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::L),
                ),
                ::rmk::action::KeyAction::Single(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::Semicolon),
                ),
            ],
            [
                ::rmk::action::KeyAction::Single(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::N),
                ),
                ::rmk::action::KeyAction::Single(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::M),
                ),
                ::rmk::action::KeyAction::Single(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::Comma),
                ),
                ::rmk::action::KeyAction::Single(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::Dot),
                ),
                ::rmk::action::KeyAction::Single(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::Enter),
                ),
            ],
            [
                ::rmk::action::KeyAction::Single(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::Space),
                ),
                ::rmk::action::KeyAction::Single(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::Backspace),
                ),
                ::rmk::action::KeyAction::LayerTapHold(
                    ::rmk::action::Action::LayerToggle(2u8),
                    2u8,
                ),
                ::rmk::action::KeyAction::No,
                ::rmk::action::KeyAction::No,
            ],
        ],
        [
            [
                ::rmk::action::KeyAction::Single(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::KbMute),
                ),
                ::rmk::action::KeyAction::WithModifier(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::Kc1),
                    ::rmk::keycode::ModifierCombination::new_from(
                        false,
                        false,
                        false,
                        true,
                        false,
                    ),
                ),
                ::rmk::action::KeyAction::WithModifier(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::Kc2),
                    ::rmk::keycode::ModifierCombination::new_from(
                        false,
                        false,
                        false,
                        true,
                        false,
                    ),
                ),
                ::rmk::action::KeyAction::WithModifier(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::Kc3),
                    ::rmk::keycode::ModifierCombination::new_from(
                        false,
                        false,
                        false,
                        true,
                        false,
                    ),
                ),
                ::rmk::action::KeyAction::No,
            ],
            [
                ::rmk::action::KeyAction::Single(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::KbVolumeUp),
                ),
                ::rmk::action::KeyAction::WithModifier(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::Kc4),
                    ::rmk::keycode::ModifierCombination::new_from(
                        false,
                        false,
                        false,
                        true,
                        false,
                    ),
                ),
                ::rmk::action::KeyAction::WithModifier(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::Kc5),
                    ::rmk::keycode::ModifierCombination::new_from(
                        false,
                        false,
                        false,
                        true,
                        false,
                    ),
                ),
                ::rmk::action::KeyAction::WithModifier(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::Kc6),
                    ::rmk::keycode::ModifierCombination::new_from(
                        false,
                        false,
                        false,
                        true,
                        false,
                    ),
                ),
                ::rmk::action::KeyAction::No,
            ],
            [
                ::rmk::action::KeyAction::Single(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::KbVolumeDown),
                ),
                ::rmk::action::KeyAction::WithModifier(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::Kc7),
                    ::rmk::keycode::ModifierCombination::new_from(
                        false,
                        false,
                        false,
                        true,
                        false,
                    ),
                ),
                ::rmk::action::KeyAction::WithModifier(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::Kc8),
                    ::rmk::keycode::ModifierCombination::new_from(
                        false,
                        false,
                        false,
                        true,
                        false,
                    ),
                ),
                ::rmk::action::KeyAction::No,
                ::rmk::action::KeyAction::No,
            ],
            [
                ::rmk::action::KeyAction::No,
                ::rmk::action::KeyAction::No,
                ::rmk::action::KeyAction::Single(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::Escape),
                ),
                ::rmk::action::KeyAction::No,
                ::rmk::action::KeyAction::Single(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::Enter),
                ),
            ],
            [
                ::rmk::action::KeyAction::No,
                ::rmk::action::KeyAction::Single(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::BrightnessDown),
                ),
                ::rmk::action::KeyAction::Single(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::BrightnessUp),
                ),
                ::rmk::action::KeyAction::No,
                ::rmk::action::KeyAction::No,
            ],
            [
                ::rmk::action::KeyAction::Single(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::Left),
                ),
                ::rmk::action::KeyAction::Single(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::Down),
                ),
                ::rmk::action::KeyAction::Single(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::UP),
                ),
                ::rmk::action::KeyAction::Single(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::Right),
                ),
                ::rmk::action::KeyAction::WithModifier(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::Quote),
                    ::rmk::keycode::ModifierCombination::new_from(
                        false,
                        false,
                        false,
                        true,
                        false,
                    ),
                ),
            ],
            [
                ::rmk::action::KeyAction::No,
                ::rmk::action::KeyAction::Single(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::Semicolon),
                ),
                ::rmk::action::KeyAction::WithModifier(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::Semicolon),
                    ::rmk::keycode::ModifierCombination::new_from(
                        false,
                        false,
                        false,
                        true,
                        false,
                    ),
                ),
                ::rmk::action::KeyAction::No,
                ::rmk::action::KeyAction::Single(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::Quote),
                ),
            ],
            [
                ::rmk::action::KeyAction::Single(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::Space),
                ),
                ::rmk::action::KeyAction::Single(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::Backspace),
                ),
                ::rmk::action::KeyAction::LayerTapHold(
                    ::rmk::action::Action::LayerToggle(2u8),
                    2u8,
                ),
                ::rmk::action::KeyAction::No,
                ::rmk::action::KeyAction::No,
            ],
        ],
        [
            [
                ::rmk::action::KeyAction::WithModifier(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::Kc9),
                    ::rmk::keycode::ModifierCombination::new_from(
                        false,
                        false,
                        false,
                        true,
                        false,
                    ),
                ),
                ::rmk::action::KeyAction::Single(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::Kc1),
                ),
                ::rmk::action::KeyAction::Single(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::Kc2),
                ),
                ::rmk::action::KeyAction::Single(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::Kc3),
                ),
                ::rmk::action::KeyAction::WithModifier(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::Kc0),
                    ::rmk::keycode::ModifierCombination::new_from(
                        false,
                        false,
                        false,
                        true,
                        false,
                    ),
                ),
            ],
            [
                ::rmk::action::KeyAction::Single(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::LeftBracket),
                ),
                ::rmk::action::KeyAction::Single(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::Kc4),
                ),
                ::rmk::action::KeyAction::Single(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::Kc5),
                ),
                ::rmk::action::KeyAction::Single(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::Kc6),
                ),
                ::rmk::action::KeyAction::Single(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::RightBracket),
                ),
            ],
            [
                ::rmk::action::KeyAction::WithModifier(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::LeftBracket),
                    ::rmk::keycode::ModifierCombination::new_from(
                        false,
                        false,
                        false,
                        true,
                        false,
                    ),
                ),
                ::rmk::action::KeyAction::Single(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::Kc7),
                ),
                ::rmk::action::KeyAction::Single(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::Kc8),
                ),
                ::rmk::action::KeyAction::Single(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::Kc9),
                ),
                ::rmk::action::KeyAction::WithModifier(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::RightBracket),
                    ::rmk::keycode::ModifierCombination::new_from(
                        false,
                        false,
                        false,
                        true,
                        false,
                    ),
                ),
            ],
            [
                ::rmk::action::KeyAction::No,
                ::rmk::action::KeyAction::No,
                ::rmk::action::KeyAction::Single(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::Escape),
                ),
                ::rmk::action::KeyAction::LayerTapHold(
                    ::rmk::action::Action::LayerToggle(1u8),
                    1u8,
                ),
                ::rmk::action::KeyAction::Single(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::Enter),
                ),
            ],
            [
                ::rmk::action::KeyAction::Single(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::F1),
                ),
                ::rmk::action::KeyAction::Single(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::F2),
                ),
                ::rmk::action::KeyAction::Single(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::F5),
                ),
                ::rmk::action::KeyAction::Single(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::F11),
                ),
                ::rmk::action::KeyAction::Single(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::F12),
                ),
            ],
            [
                ::rmk::action::KeyAction::Single(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::Kc0),
                ),
                ::rmk::action::KeyAction::Single(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::Slash),
                ),
                ::rmk::action::KeyAction::Single(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::Backslash),
                ),
                ::rmk::action::KeyAction::No,
                ::rmk::action::KeyAction::Single(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::Enter),
                ),
            ],
            [
                ::rmk::action::KeyAction::No,
                ::rmk::action::KeyAction::Single(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::Minus),
                ),
                ::rmk::action::KeyAction::WithModifier(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::Equal),
                    ::rmk::keycode::ModifierCombination::new_from(
                        false,
                        false,
                        false,
                        true,
                        false,
                    ),
                ),
                ::rmk::action::KeyAction::WithModifier(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::Minus),
                    ::rmk::keycode::ModifierCombination::new_from(
                        false,
                        false,
                        false,
                        true,
                        false,
                    ),
                ),
                ::rmk::action::KeyAction::Single(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::Equal),
                ),
            ],
            [
                ::rmk::action::KeyAction::Single(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::Space),
                ),
                ::rmk::action::KeyAction::Single(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::Backspace),
                ),
                ::rmk::action::KeyAction::No,
                ::rmk::action::KeyAction::No,
                ::rmk::action::KeyAction::No,
            ],
        ],
        [
            [
                ::rmk::action::KeyAction::No,
                ::rmk::action::KeyAction::No,
                ::rmk::action::KeyAction::No,
                ::rmk::action::KeyAction::No,
                ::rmk::action::KeyAction::No,
            ],
            [
                ::rmk::action::KeyAction::No,
                ::rmk::action::KeyAction::No,
                ::rmk::action::KeyAction::Single(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::OutputUsb),
                ),
                ::rmk::action::KeyAction::Single(
                    ::rmk::action::Action::Key(::rmk::keycode::KeyCode::OutputBluetooth),
                ),
                ::rmk::action::KeyAction::No,
            ],
            [
                ::rmk::action::KeyAction::No,
                ::rmk::action::KeyAction::No,
                ::rmk::action::KeyAction::No,
                ::rmk::action::KeyAction::No,
                ::rmk::action::KeyAction::No,
            ],
            [
                ::rmk::action::KeyAction::No,
                ::rmk::action::KeyAction::No,
                ::rmk::action::KeyAction::No,
                ::rmk::action::KeyAction::No,
                ::rmk::action::KeyAction::No,
            ],
            [
                ::rmk::action::KeyAction::No,
                ::rmk::action::KeyAction::No,
                ::rmk::action::KeyAction::No,
                ::rmk::action::KeyAction::No,
                ::rmk::action::KeyAction::No,
            ],
            [
                ::rmk::action::KeyAction::No,
                ::rmk::action::KeyAction::No,
                ::rmk::action::KeyAction::No,
                ::rmk::action::KeyAction::No,
                ::rmk::action::KeyAction::No,
            ],
            [
                ::rmk::action::KeyAction::No,
                ::rmk::action::KeyAction::No,
                ::rmk::action::KeyAction::No,
                ::rmk::action::KeyAction::No,
                ::rmk::action::KeyAction::No,
            ],
            [
                ::rmk::action::KeyAction::No,
                ::rmk::action::KeyAction::No,
                ::rmk::action::KeyAction::No,
                ::rmk::action::KeyAction::No,
                ::rmk::action::KeyAction::No,
            ],
        ],
    ]
}
use ::embassy_nrf::bind_interrupts;
struct Irqs;
#[automatically_derived]
impl ::core::marker::Copy for Irqs {}
#[automatically_derived]
impl ::core::clone::Clone for Irqs {
    #[inline]
    fn clone(&self) -> Irqs {
        *self
    }
}
#[allow(non_snake_case)]
#[no_mangle]
unsafe extern "C" fn USBD() {
    <::embassy_nrf::usb::InterruptHandler<
        ::embassy_nrf::peripherals::USBD,
    > as ::embassy_nrf::interrupt::typelevel::Handler<
        ::embassy_nrf::interrupt::typelevel::USBD,
    >>::on_interrupt();
}
unsafe impl ::embassy_nrf::interrupt::typelevel::Binding<
    ::embassy_nrf::interrupt::typelevel::USBD,
    ::embassy_nrf::usb::InterruptHandler<::embassy_nrf::peripherals::USBD>,
> for Irqs {}
#[allow(non_snake_case)]
#[no_mangle]
unsafe extern "C" fn SAADC() {
    <::embassy_nrf::saadc::InterruptHandler as ::embassy_nrf::interrupt::typelevel::Handler<
        ::embassy_nrf::interrupt::typelevel::SAADC,
    >>::on_interrupt();
}
unsafe impl ::embassy_nrf::interrupt::typelevel::Binding<
    ::embassy_nrf::interrupt::typelevel::SAADC,
    ::embassy_nrf::saadc::InterruptHandler,
> for Irqs {}
#[allow(non_snake_case)]
#[no_mangle]
unsafe extern "C" fn CLOCK_POWER() {
    <::embassy_nrf::usb::vbus_detect::InterruptHandler as ::embassy_nrf::interrupt::typelevel::Handler<
        ::embassy_nrf::interrupt::typelevel::CLOCK_POWER,
    >>::on_interrupt();
}
unsafe impl ::embassy_nrf::interrupt::typelevel::Binding<
    ::embassy_nrf::interrupt::typelevel::CLOCK_POWER,
    ::embassy_nrf::usb::vbus_detect::InterruptHandler,
> for Irqs {}
#[doc(hidden)]
async fn ____embassy_main_task(spawner: ::embassy_executor::Spawner) {
    match () {
        () => {
            if {
                const CHECK: bool = {
                    const fn check() -> bool {
                        let module_path = "central".as_bytes();
                        if if 7usize > module_path.len() {
                            false
                        } else {
                            module_path[0usize] == 99u8 && module_path[1usize] == 101u8
                                && module_path[2usize] == 110u8
                                && module_path[3usize] == 116u8
                                && module_path[4usize] == 114u8
                                && module_path[5usize] == 97u8
                                && module_path[6usize] == 108u8
                                && if 7usize == module_path.len() {
                                    true
                                } else {
                                    module_path[7usize] == b':'
                                }
                        } {
                            return true;
                        }
                        false
                    }
                    check()
                };
                CHECK
            } {
                defmt::export::acquire_header_and_release(
                    &{
                        defmt::export::make_istr({
                            #[link_section = ".defmt.info.{\"package\":\"rmk-config\",\"tag\":\"defmt_info\",\"data\":\"RMK start!\",\"disambiguator\":\"14837084225462078411\",\"crate_name\":\"central\"}"]
                            #[export_name = "{\"package\":\"rmk-config\",\"tag\":\"defmt_info\",\"data\":\"RMK start!\",\"disambiguator\":\"14837084225462078411\",\"crate_name\":\"central\"}"]
                            static DEFMT_LOG_STATEMENT: u8 = 0;
                            &DEFMT_LOG_STATEMENT as *const u8 as u16
                        })
                    },
                );
            }
        }
    };
    use embassy_nrf::interrupt::InterruptExt;
    let mut config = ::embassy_nrf::config::Config::default();
    config.gpiote_interrupt_priority = ::embassy_nrf::interrupt::Priority::P3;
    config.time_interrupt_priority = ::embassy_nrf::interrupt::Priority::P3;
    ::embassy_nrf::interrupt::USBD.set_priority(::embassy_nrf::interrupt::Priority::P2);
    ::embassy_nrf::interrupt::CLOCK_POWER
        .set_priority(::embassy_nrf::interrupt::Priority::P2);
    let p = ::embassy_nrf::init(config);
    let software_vbus = ::rmk::ble::SOFTWARE_VBUS
        .get_or_init(|| ::embassy_nrf::usb::vbus_detect::SoftwareVbusDetect::new(
            true,
            false,
        ));
    let driver = ::embassy_nrf::usb::Driver::new(p.USBD, Irqs, software_vbus);
    let light_config = ::rmk::config::LightConfig {
        capslock: None,
        numslock: None,
        scrolllock: None,
    };
    let behavior_config = ::rmk::config::BehaviorConfig {
        tri_layer: ::core::option::Option::Some([1u8, 2u8, 3u8]),
        tap_hold: ::rmk::config::TapHoldConfig::default(),
        one_shot: ::rmk::config::OneShotConfig::default(),
        combo: ::core::default::Default::default(),
    };
    let direct_pins = {
        let p0_22_0_0 = Some(
            ::embassy_nrf::gpio::Input::new(
                ::embassy_nrf::gpio::AnyPin::from(p.P0_22),
                ::embassy_nrf::gpio::Pull::Up,
            ),
        );
        let p0_17_0_1 = Some(
            ::embassy_nrf::gpio::Input::new(
                ::embassy_nrf::gpio::AnyPin::from(p.P0_17),
                ::embassy_nrf::gpio::Pull::Up,
            ),
        );
        let p0_08_0_2 = Some(
            ::embassy_nrf::gpio::Input::new(
                ::embassy_nrf::gpio::AnyPin::from(p.P0_08),
                ::embassy_nrf::gpio::Pull::Up,
            ),
        );
        let p0_24_0_3 = Some(
            ::embassy_nrf::gpio::Input::new(
                ::embassy_nrf::gpio::AnyPin::from(p.P0_24),
                ::embassy_nrf::gpio::Pull::Up,
            ),
        );
        let p0_20_0_4 = Some(
            ::embassy_nrf::gpio::Input::new(
                ::embassy_nrf::gpio::AnyPin::from(p.P0_20),
                ::embassy_nrf::gpio::Pull::Up,
            ),
        );
        let direct_pins_row_0 = [p0_22_0_0, p0_17_0_1, p0_08_0_2, p0_24_0_3, p0_20_0_4];
        let p1_13_1_0 = Some(
            ::embassy_nrf::gpio::Input::new(
                ::embassy_nrf::gpio::AnyPin::from(p.P1_13),
                ::embassy_nrf::gpio::Pull::Up,
            ),
        );
        let p1_15_1_1 = Some(
            ::embassy_nrf::gpio::Input::new(
                ::embassy_nrf::gpio::AnyPin::from(p.P1_15),
                ::embassy_nrf::gpio::Pull::Up,
            ),
        );
        let p0_02_1_2 = Some(
            ::embassy_nrf::gpio::Input::new(
                ::embassy_nrf::gpio::AnyPin::from(p.P0_02),
                ::embassy_nrf::gpio::Pull::Up,
            ),
        );
        let p1_00_1_3 = Some(
            ::embassy_nrf::gpio::Input::new(
                ::embassy_nrf::gpio::AnyPin::from(p.P1_00),
                ::embassy_nrf::gpio::Pull::Up,
            ),
        );
        let p0_10_1_4 = Some(
            ::embassy_nrf::gpio::Input::new(
                ::embassy_nrf::gpio::AnyPin::from(p.P0_10),
                ::embassy_nrf::gpio::Pull::Up,
            ),
        );
        let direct_pins_row_1 = [p1_13_1_0, p1_15_1_1, p0_02_1_2, p1_00_1_3, p0_10_1_4];
        let p1_08_2_0 = Some(
            ::embassy_nrf::gpio::Input::new(
                ::embassy_nrf::gpio::AnyPin::from(p.P1_08),
                ::embassy_nrf::gpio::Pull::Up,
            ),
        );
        let p1_02_2_1 = Some(
            ::embassy_nrf::gpio::Input::new(
                ::embassy_nrf::gpio::AnyPin::from(p.P1_02),
                ::embassy_nrf::gpio::Pull::Up,
            ),
        );
        let p0_12_2_2 = Some(
            ::embassy_nrf::gpio::Input::new(
                ::embassy_nrf::gpio::AnyPin::from(p.P0_12),
                ::embassy_nrf::gpio::Pull::Up,
            ),
        );
        let p0_07_2_3 = Some(
            ::embassy_nrf::gpio::Input::new(
                ::embassy_nrf::gpio::AnyPin::from(p.P0_07),
                ::embassy_nrf::gpio::Pull::Up,
            ),
        );
        let p1_11_2_4 = Some(
            ::embassy_nrf::gpio::Input::new(
                ::embassy_nrf::gpio::AnyPin::from(p.P1_11),
                ::embassy_nrf::gpio::Pull::Up,
            ),
        );
        let direct_pins_row_2 = [p1_08_2_0, p1_02_2_1, p0_12_2_2, p0_07_2_3, p1_11_2_4];
        let __3_0 = None;
        let __3_1 = None;
        let p0_11_3_2 = Some(
            ::embassy_nrf::gpio::Input::new(
                ::embassy_nrf::gpio::AnyPin::from(p.P0_11),
                ::embassy_nrf::gpio::Pull::Up,
            ),
        );
        let p0_09_3_3 = Some(
            ::embassy_nrf::gpio::Input::new(
                ::embassy_nrf::gpio::AnyPin::from(p.P0_09),
                ::embassy_nrf::gpio::Pull::Up,
            ),
        );
        let p1_06_3_4 = Some(
            ::embassy_nrf::gpio::Input::new(
                ::embassy_nrf::gpio::AnyPin::from(p.P1_06),
                ::embassy_nrf::gpio::Pull::Up,
            ),
        );
        let direct_pins_row_3 = [__3_0, __3_1, p0_11_3_2, p0_09_3_3, p1_06_3_4];
        let direct_pins = [
            direct_pins_row_0,
            direct_pins_row_1,
            direct_pins_row_2,
            direct_pins_row_3,
        ];
        direct_pins
    };
    let central_addr = [24u8, 226u8, 33u8, 128u8, 192u8, 199u8];
    let peripheral_addr0 = [126u8, 254u8, 115u8, 158u8, 102u8, 227u8];
    let storage_config = ::rmk::config::StorageConfig {
        num_sectors: 16u8,
        start_addr: 393216usize,
        clear_storage: true,
    };
    let (sd, flash) = ::rmk::initialize_nrf_sd_and_flash(
        "rmk",
        spawner,
        Some(central_addr),
    );
    let charging_state_low_active = false;
    let is_charging_pin: ::core::option::Option<::embassy_nrf::gpio::Input<'_>> = None;
    let charge_led_low_active = false;
    let charge_led_pin: ::core::option::Option<::embassy_nrf::gpio::Output<'_>> = None;
    let ble_battery_config = ::rmk::config::BleBatteryConfig::new(
        is_charging_pin,
        charging_state_low_active,
        charge_led_pin,
        charge_led_low_active,
    );
    let rmk_config = ::rmk::config::RmkConfig {
        usb_config: KEYBOARD_USB_CONFIG,
        vial_config: VIAL_CONFIG,
        storage_config,
        behavior_config,
        ble_battery_config,
        ..Default::default()
    };
    let mut light_controller: ::rmk::light::LightController<
        ::embassy_nrf::gpio::Output,
    > = ::rmk::light::LightController::new(light_config);
    let mut default_keymap = get_default_keymap();
    let (keymap, storage) = ::rmk::initialize_keymap_and_storage(
            &mut default_keymap,
            flash,
            rmk_config.storage_config,
            rmk_config.behavior_config.clone(),
        )
        .await;
    let mut keyboard = ::rmk::keyboard::Keyboard::new(
        &keymap,
        rmk_config.behavior_config.clone(),
    );
    let debouncer = ::rmk::debounce::default_debouncer::DefaultDebouncer::<
        COL,
        ROW,
    >::new();
    let mut matrix = ::rmk::split::central::CentralDirectPinMatrix::<
        _,
        _,
        0usize,
        0usize,
        4usize,
        5usize,
        20usize,
    >::new(direct_pins, debouncer, true);
    let mut battery_processor = ::rmk::input_device::battery::BatteryProcessor::new(
        2000u32,
        2806u32,
        &keymap,
    );
    let mut joystick_processor_default = rmk::input_device::joystick::JoystickProcessor::new(
        [[80i16, 0i16], [0i16, 80i16]],
        [29130i16, 29365i16],
        6u16,
        &keymap,
    );
    let mut adc_device = {
        use embassy_nrf::saadc::{self, Input as _};
        let saadc_config = saadc::Config::default();
        embassy_nrf::interrupt::SAADC.set_priority(embassy_nrf::interrupt::Priority::P3);
        let adc = saadc::Saadc::new(
            p.SAADC,
            Irqs,
            saadc_config,
            [
                saadc::ChannelConfig::single_ended(saadc::VddhDiv5Input.degrade_saadc()),
                saadc::ChannelConfig::single_ended(p.P0_31.degrade_saadc()),
                saadc::ChannelConfig::single_ended(p.P0_29.degrade_saadc()),
            ],
        );
        adc.calibrate().await;
        rmk::input_device::adc::NrfAdc::new(
            adc,
            [
                ::rmk::input_device::adc::AnalogEventType::Battery,
                ::rmk::input_device::adc::AnalogEventType::Joystick(2u8),
            ],
            20,
        )
    };
    use ::rmk::input_device::Runnable;
    ::rmk::embassy_futures::join::join(
            ::rmk::embassy_futures::join::join(
                ::rmk::embassy_futures::join::join(
                    ::rmk::embassy_futures::join::join(
                        {
                            use ::rmk::input_device::InputDevice;
                            ::rmk::futures::future::join(
                                async {
                                    loop {
                                        let e = adc_device.read_event().await;
                                        match e {
                                            ::rmk::event::Event::Key(key_event) => {
                                                ::rmk::channel::KEY_EVENT_CHANNEL.send(key_event).await;
                                            }
                                            _ => {
                                                if ::rmk::channel::EVENT_CHANNEL.is_full() {
                                                    let _ = ::rmk::channel::EVENT_CHANNEL.receive().await;
                                                }
                                                ::rmk::channel::EVENT_CHANNEL.send(e).await;
                                            }
                                        }
                                    }
                                },
                                async {
                                    loop {
                                        let e = matrix.read_event().await;
                                        match e {
                                            ::rmk::event::Event::Key(key_event) => {
                                                ::rmk::channel::KEY_EVENT_CHANNEL.send(key_event).await;
                                            }
                                            _ => {
                                                if ::rmk::channel::EVENT_CHANNEL.is_full() {
                                                    let _ = ::rmk::channel::EVENT_CHANNEL.receive().await;
                                                }
                                                ::rmk::channel::EVENT_CHANNEL.send(e).await;
                                            }
                                        }
                                    }
                                },
                            )
                        },
                        ::rmk::run_rmk(
                            &keymap,
                            driver,
                            storage,
                            light_controller,
                            rmk_config,
                            sd,
                        ),
                    ),
                    keyboard.run(),
                ),
                {
                    use rmk::input_device::InputProcessor;
                    async {
                        loop {
                            let event = ::rmk::channel::EVENT_CHANNEL.receive().await;
                            match battery_processor.process(event).await {
                                ::rmk::input_device::ProcessResult::Stop => {
                                    continue;
                                }
                                ::rmk::input_device::ProcessResult::Continue(
                                    next_event,
                                ) => {
                                    let mut current_event = next_event;
                                    match joystick_processor_default
                                        .process(current_event)
                                        .await
                                    {
                                        ::rmk::input_device::ProcessResult::Stop => {
                                            continue;
                                        }
                                        ::rmk::input_device::ProcessResult::Continue(
                                            next_event,
                                        ) => {
                                            current_event = next_event;
                                        }
                                    }
                                }
                            }
                        }
                    }
                },
            ),
            ::rmk::split::central::run_peripheral_manager::<
                4usize,
                5usize,
                4usize,
                0usize,
            >(0usize, [126u8, 254u8, 115u8, 158u8, 102u8, 227u8]),
        )
        .await;
}
fn __embassy_main(
    spawner: ::embassy_executor::Spawner,
) -> ::embassy_executor::SpawnToken<impl Sized> {
    const POOL_SIZE: usize = 1;
    static POOL: ::embassy_executor::_export::TaskPoolRef = ::embassy_executor::_export::TaskPoolRef::new();
    unsafe {
        POOL.get::<_, POOL_SIZE>()
            ._spawn_async_fn(move || ____embassy_main_task(spawner))
    }
}
#[doc(hidden)]
#[export_name = "main"]
pub unsafe extern "C" fn __cortex_m_rt_main_trampoline() {
    #[allow(static_mut_refs)] __cortex_m_rt_main()
}
fn __cortex_m_rt_main() -> ! {
    unsafe fn __make_static<T>(t: &mut T) -> &'static mut T {
        ::core::mem::transmute(t)
    }
    let mut executor = ::embassy_executor::Executor::new();
    let executor = unsafe { __make_static(&mut executor) };
    executor
        .run(|spawner| {
            spawner.must_spawn(__embassy_main(spawner));
        })
}
