// Wrapper enum for keycode type which doesn't implement Deserialize.
// This is messy, but it's the best way to do it when the target doesn't
// cooperate.

use keybind::Keycode;
use serde::Deserialize;

/// Copied from keybind's source code to create an identical enum
#[derive(Deserialize, Clone)]
pub enum KeyCodeDef {
    Key0,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    Escape,
    Space,
    LControl,
    RControl,
    LShift,
    RShift,
    LAlt,
    RAlt,
    Enter,
}

/// https://stackoverflow.com/questions/59984712/rust-macro-to-convert-between-identical-enums
macro_rules! convert_enum{($src: ident, $dst: ident, $($variant: ident,)*)=> {
    impl From<$src> for $dst {
        fn from(src: $src) -> Self {
            match src {
                $($src::$variant => Self::$variant,)*
            }
        }
    }
}}

// Expands to impl From<KeyCodeDef> for KeyCode {...}
convert_enum!(
    KeyCodeDef, Keycode, Key0, Key1, Key2, Key3, Key4, Key5, Key6, Key7, Key8, Key9, A, B, C, D, E,
    F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, F1, F2, F3, F4, F5, F6, F7, F8,
    F9, F10, F11, F12, Escape, Space, LControl, RControl, LShift, RShift, LAlt, RAlt, Enter,
);
