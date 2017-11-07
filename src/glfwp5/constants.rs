#[cfg(feature = "glfw")]

use glfw;

// these constants come from and are inspired by the pyprocessing project
// (https://code.google.com/p/pyprocessing/source/browse/trunk/pyprocessing/constants.py)
// with some slight adjustments

// shapes
// pub const POINTS = GL_POINTS;
// pub const LINES = GL_LINES;
// pub const TRIANGLES = GL_TRIANGLES;
// pub const TRIANGLE_FAN = GL_TRIANGLE_FAN;
// pub const TRIANGLE_STRIP = GL_TRIANGLE_STRIP;
// pub const QUADS = GL_QUADS;
// pub const QUAD_STRIP = GL_QUAD_STRIP;
pub const CLOSE: isize = 1;

// perlin noise table
pub const perlin: [isize; 512] = [
    151,
    160,
    137,
    91,
    90,
    15,
    131,
    13,
    201,
    95,
    96,
    53,
    194,
    233,
    7,
    225,
    140,
    36,
    103,
    30,
    69,
    142,
    8,
    99,
    37,
    240,
    21,
    10,
    23,
    190,
    6,
    148,
    247,
    120,
    234,
    75,
    0,
    26,
    197,
    62,
    94,
    252,
    219,
    203,
    117,
    35,
    11,
    32,
    57,
    177,
    33,
    88,
    237,
    149,
    56,
    87,
    174,
    20,
    125,
    136,
    171,
    168,
    68,
    175,
    74,
    165,
    71,
    134,
    139,
    48,
    27,
    166,
    77,
    146,
    158,
    231,
    83,
    111,
    229,
    122,
    60,
    211,
    133,
    230,
    220,
    105,
    92,
    41,
    55,
    46,
    245,
    40,
    244,
    102,
    143,
    54,
    65,
    25,
    63,
    161,
    1,
    216,
    80,
    73,
    209,
    76,
    132,
    187,
    208,
    89,
    18,
    169,
    200,
    196,
    135,
    130,
    116,
    188,
    159,
    86,
    164,
    100,
    109,
    198,
    173,
    186,
    3,
    64,
    52,
    217,
    226,
    250,
    124,
    123,
    5,
    202,
    38,
    147,
    118,
    126,
    255,
    82,
    85,
    212,
    207,
    206,
    59,
    227,
    47,
    16,
    58,
    17,
    182,
    189,
    28,
    42,
    223,
    183,
    170,
    213,
    119,
    248,
    152,
    2,
    44,
    154,
    163,
    70,
    221,
    153,
    101,
    155,
    167,
    43,
    172,
    9,
    129,
    22,
    39,
    253,
    19,
    98,
    108,
    110,
    79,
    113,
    224,
    232,
    178,
    185,
    112,
    104,
    218,
    246,
    97,
    228,
    251,
    34,
    242,
    193,
    238,
    210,
    144,
    12,
    191,
    179,
    162,
    241,
    81,
    51,
    145,
    235,
    249,
    14,
    239,
    107,
    49,
    192,
    214,
    31,
    181,
    199,
    106,
    157,
    184,
    84,
    204,
    176,
    115,
    121,
    50,
    45,
    127,
    4,
    150,
    254,
    138,
    236,
    205,
    93,
    222,
    114,
    67,
    29,
    24,
    72,
    243,
    141,
    128,
    195,
    78,
    66,
    215,
    61,
    156,
    180,
    151,
    160,
    137,
    91,
    90,
    15,
    131,
    13,
    201,
    95,
    96,
    53,
    194,
    233,
    7,
    225,
    140,
    36,
    103,
    30,
    69,
    142,
    8,
    99,
    37,
    240,
    21,
    10,
    23,
    190,
    6,
    148,
    247,
    120,
    234,
    75,
    0,
    26,
    197,
    62,
    94,
    252,
    219,
    203,
    117,
    35,
    11,
    32,
    57,
    177,
    33,
    88,
    237,
    149,
    56,
    87,
    174,
    20,
    125,
    136,
    171,
    168,
    68,
    175,
    74,
    165,
    71,
    134,
    139,
    48,
    27,
    166,
    77,
    146,
    158,
    231,
    83,
    111,
    229,
    122,
    60,
    211,
    133,
    230,
    220,
    105,
    92,
    41,
    55,
    46,
    245,
    40,
    244,
    102,
    143,
    54,
    65,
    25,
    63,
    161,
    1,
    216,
    80,
    73,
    209,
    76,
    132,
    187,
    208,
    89,
    18,
    169,
    200,
    196,
    135,
    130,
    116,
    188,
    159,
    86,
    164,
    100,
    109,
    198,
    173,
    186,
    3,
    64,
    52,
    217,
    226,
    250,
    124,
    123,
    5,
    202,
    38,
    147,
    118,
    126,
    255,
    82,
    85,
    212,
    207,
    206,
    59,
    227,
    47,
    16,
    58,
    17,
    182,
    189,
    28,
    42,
    223,
    183,
    170,
    213,
    119,
    248,
    152,
    2,
    44,
    154,
    163,
    70,
    221,
    153,
    101,
    155,
    167,
    43,
    172,
    9,
    129,
    22,
    39,
    253,
    19,
    98,
    108,
    110,
    79,
    113,
    224,
    232,
    178,
    185,
    112,
    104,
    218,
    246,
    97,
    228,
    251,
    34,
    242,
    193,
    238,
    210,
    144,
    12,
    191,
    179,
    162,
    241,
    81,
    51,
    145,
    235,
    249,
    14,
    239,
    107,
    49,
    192,
    214,
    31,
    181,
    199,
    106,
    157,
    184,
    84,
    204,
    176,
    115,
    121,
    50,
    45,
    127,
    4,
    150,
    254,
    138,
    236,
    205,
    93,
    222,
    114,
    67,
    29,
    24,
    72,
    243,
    141,
    128,
    195,
    78,
    66,
    215,
    61,
    156,
    180,
];

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Button4,
    Button5,
    Button6,
    Button7,
    Button8,
}

impl From<MouseButton> for glfw::MouseButton {
    fn from(btn: MouseButton) -> Self {
        match btn {
            MouseButton::Left => glfw::MouseButton::Button1,
            MouseButton::Right => glfw::MouseButton::Button2,
            MouseButton::Middle => glfw::MouseButton::Button3,
            MouseButton::Button4 => glfw::MouseButton::Button4,
            MouseButton::Button5 => glfw::MouseButton::Button5,
            MouseButton::Button6 => glfw::MouseButton::Button6,
            MouseButton::Button7 => glfw::MouseButton::Button7,
            MouseButton::Button8 => glfw::MouseButton::Button8,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Key {
    Space,
    Apostrophe,
    Comma,
    Minus,
    Period,
    Slash,
    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    Semicolon,
    Equals,
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
    LBracket,
    Backslash,
    RBracket,
    Grave,
    World1,
    World2,
    Escape,
    Enter,
    Tab,
    Backspace,
    Insert,
    Delete,
    Right,
    Left,
    Down,
    Up,
    PageUp,
    PageDown,
    Home,
    End,
    CapsLock,
    ScrollLock,
    NumLock,
    PrintScreen,
    Pause,
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
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    F25,
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
    NumpadDecimal,
    NumpadDivide,
    NumpadMultiply,
    NumpadSubtract,
    NumpadAdd,
    NumpadEnter,
    NumpadEqual,
    LShift,
    LControl,
    LAlt,
    LSuper,
    RShift,
    RControl,
    RAlt,
    RSuper,
    Menu,
    Unknown,
}

impl From<Key> for glfw::Key {
    fn from(btn: Key) -> Self {
        match btn {
            Key::Space => glfw::Key::Space,
            Key::Apostrophe => glfw::Key::Apostrophe,
            Key::Comma => glfw::Key::Comma,
            Key::Minus => glfw::Key::Minus,
            Key::Period => glfw::Key::Period,
            Key::Slash => glfw::Key::Slash,
            Key::Num0 => glfw::Key::Num0,
            Key::Num1 => glfw::Key::Num1,
            Key::Num2 => glfw::Key::Num2,
            Key::Num3 => glfw::Key::Num3,
            Key::Num4 => glfw::Key::Num4,
            Key::Num5 => glfw::Key::Num5,
            Key::Num6 => glfw::Key::Num6,
            Key::Num7 => glfw::Key::Num7,
            Key::Num8 => glfw::Key::Num8,
            Key::Num9 => glfw::Key::Num9,
            Key::Semicolon => glfw::Key::Semicolon,
            Key::Equals => glfw::Key::Equal,
            Key::A => glfw::Key::A,
            Key::B => glfw::Key::B,
            Key::C => glfw::Key::C,
            Key::D => glfw::Key::D,
            Key::E => glfw::Key::E,
            Key::F => glfw::Key::F,
            Key::G => glfw::Key::G,
            Key::H => glfw::Key::H,
            Key::I => glfw::Key::I,
            Key::J => glfw::Key::J,
            Key::K => glfw::Key::K,
            Key::L => glfw::Key::L,
            Key::M => glfw::Key::M,
            Key::N => glfw::Key::N,
            Key::O => glfw::Key::O,
            Key::P => glfw::Key::P,
            Key::Q => glfw::Key::Q,
            Key::R => glfw::Key::R,
            Key::S => glfw::Key::S,
            Key::T => glfw::Key::T,
            Key::U => glfw::Key::U,
            Key::V => glfw::Key::V,
            Key::W => glfw::Key::W,
            Key::X => glfw::Key::X,
            Key::Y => glfw::Key::Y,
            Key::Z => glfw::Key::Z,
            Key::LBracket => glfw::Key::LeftBracket,
            Key::Backslash => glfw::Key::Backslash,
            Key::RBracket => glfw::Key::RightBracket,
            Key::Grave => glfw::Key::GraveAccent,
            Key::World1 => glfw::Key::World1,
            Key::World2 => glfw::Key::World2,
            Key::Escape => glfw::Key::Escape,
            Key::Enter => glfw::Key::Enter,
            Key::Tab => glfw::Key::Tab,
            Key::Backspace => glfw::Key::Backspace,
            Key::Insert => glfw::Key::Insert,
            Key::Delete => glfw::Key::Delete,
            Key::Right => glfw::Key::Right,
            Key::Left => glfw::Key::Left,
            Key::Down => glfw::Key::Down,
            Key::Up => glfw::Key::Up,
            Key::PageUp => glfw::Key::PageUp,
            Key::PageDown => glfw::Key::PageDown,
            Key::Home => glfw::Key::Home,
            Key::End => glfw::Key::End,
            Key::CapsLock => glfw::Key::CapsLock,
            Key::ScrollLock => glfw::Key::ScrollLock,
            Key::NumLock => glfw::Key::NumLock,
            Key::PrintScreen => glfw::Key::PrintScreen,
            Key::Pause => glfw::Key::Pause,
            Key::F1 => glfw::Key::F1,
            Key::F2 => glfw::Key::F2,
            Key::F3 => glfw::Key::F3,
            Key::F4 => glfw::Key::F4,
            Key::F5 => glfw::Key::F5,
            Key::F6 => glfw::Key::F6,
            Key::F7 => glfw::Key::F7,
            Key::F8 => glfw::Key::F8,
            Key::F9 => glfw::Key::F9,
            Key::F10 => glfw::Key::F10,
            Key::F11 => glfw::Key::F11,
            Key::F12 => glfw::Key::F12,
            Key::F13 => glfw::Key::F13,
            Key::F14 => glfw::Key::F14,
            Key::F15 => glfw::Key::F15,
            Key::F16 => glfw::Key::F16,
            Key::F17 => glfw::Key::F17,
            Key::F18 => glfw::Key::F18,
            Key::F19 => glfw::Key::F19,
            Key::F20 => glfw::Key::F20,
            Key::F21 => glfw::Key::F21,
            Key::F22 => glfw::Key::F22,
            Key::F23 => glfw::Key::F23,
            Key::F24 => glfw::Key::F24,
            Key::F25 => glfw::Key::F25,
            Key::Numpad0 => glfw::Key::Kp0,
            Key::Numpad1 => glfw::Key::Kp1,
            Key::Numpad2 => glfw::Key::Kp2,
            Key::Numpad3 => glfw::Key::Kp3,
            Key::Numpad4 => glfw::Key::Kp4,
            Key::Numpad5 => glfw::Key::Kp5,
            Key::Numpad6 => glfw::Key::Kp6,
            Key::Numpad7 => glfw::Key::Kp7,
            Key::Numpad8 => glfw::Key::Kp8,
            Key::Numpad9 => glfw::Key::Kp9,
            Key::NumpadDecimal => glfw::Key::KpDecimal,
            Key::NumpadDivide => glfw::Key::KpDivide,
            Key::NumpadMultiply => glfw::Key::KpMultiply,
            Key::NumpadSubtract => glfw::Key::KpSubtract,
            Key::NumpadAdd => glfw::Key::KpAdd,
            Key::NumpadEnter => glfw::Key::KpEnter,
            Key::NumpadEqual => glfw::Key::KpEqual,
            Key::LShift => glfw::Key::LeftShift,
            Key::LControl => glfw::Key::LeftControl,
            Key::LAlt => glfw::Key::LeftAlt,
            Key::LSuper => glfw::Key::LeftSuper,
            Key::RShift => glfw::Key::RightShift,
            Key::RControl => glfw::Key::RightControl,
            Key::RAlt => glfw::Key::RightAlt,
            Key::RSuper => glfw::Key::RightSuper,
            Key::Menu => glfw::Key::Menu,
            Key::Unknown => glfw::Key::Unknown,
        }
    }
}
