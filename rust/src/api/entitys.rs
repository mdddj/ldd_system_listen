use flutter_rust_bridge::frb;
use multiinput::{KeyId, KeyboardDisplayInfo, State};


pub struct LogEntry {
    pub time_millis: i64,
    pub level: i32,
    pub tag: String,
    pub msg: String,
}

#[derive(Debug)]
pub enum LddState {
    Pressed,
    Released,
}
impl  LddState {
    #[frb(sync)]
    pub fn format_string(&self)-> String {
        match self {
            LddState::Pressed => "按下".to_owned(),
            LddState::Released => "松开".to_owned(),
        }
    }
}

impl From<State> for LddState {
    fn from(state: State) -> Self {
        match state {
            State::Pressed => LddState::Pressed,
            State::Released => LddState::Released,
        }
    }
}

// Optionally, implement the conversion from LddState to State
impl From<LddState> for State {
    fn from(ldd_state: LddState) -> Self {
        match ldd_state {
            LddState::Pressed => State::Pressed,
            LddState::Released => State::Released,
        }
    }
}

#[derive(Debug)]
pub enum LddRawEvent {
    LddKeyboardEvent(usize, LddKeyId, LddState, LddKeyboard),
    ScanGunEvent(Vec<LddKeyId>,LddKeyboard)
}
#[derive(Debug,Clone)]
pub struct LddKeyboard {
    pub name: String,
    pub serial: Option<String>,
}



impl From<KeyboardDisplayInfo> for LddKeyboard {
    fn from(value: KeyboardDisplayInfo) -> Self {
        LddKeyboard {
            name: value.name,
            serial: value.serial,
        }
    }
}


#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum LddKeyId {
    Escape,
    Return,
    Backspace,
    Left,
    Right,
    Up,
    Down,
    Space,
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
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Shift,
    LeftCtrl,
    RightCtrl,
    LeftAlt,
    RightAlt,
    CapsLock,
    Pause,
    PageUp,
    PageDown,
    PrintScreen,
    Insert,
    End,
    Home,
    Delete,
    Add,
    Subtract,
    Multiply,
    Separator,
    Decimal,
    Divide,
    BackTick,
    BackSlash,
    ForwardSlash,
    Plus,
    Minus,
    FullStop,
    Comma,
    Tab,
    Numlock,
    LeftSquareBracket,
    RightSquareBracket,
    SemiColon,
    Apostrophe,
    Hash,
}

impl LddKeyId  {

    ///转传统字符串
    #[frb(sync)]
   pub fn key_id_to_string(&self) -> String {
        let c = self.clone();
        let f = format!("{:?}",c);
        let str = match c {
            LddKeyId::A => "a",
            LddKeyId::B => "b",
            LddKeyId::C => "c",
            LddKeyId::D => "d",
            LddKeyId::E => "e",
            LddKeyId::F => "f",
            LddKeyId::G => "g",
            LddKeyId::H => "h",
            LddKeyId::I => "i",
            LddKeyId::J => "j",
            LddKeyId::K => "k",
            LddKeyId::L => "l",
            LddKeyId::M => "m",
            LddKeyId::N => "n",
            LddKeyId::O => "o",
            LddKeyId::P => "p",
            LddKeyId::Q => "q",
            LddKeyId::R => "r",
            LddKeyId::S => "s",
            LddKeyId::T => "t",
            LddKeyId::U => "u",
            LddKeyId::V => "v",
            LddKeyId::W => "w",
            LddKeyId::X => "x",
            LddKeyId::Y => "y",
            LddKeyId::Z => "z",
            LddKeyId::Zero => "0",
            LddKeyId::One => "1",
            LddKeyId::Two => "2",
            LddKeyId::Three => "3",
            LddKeyId::Four => "4",
            LddKeyId::Five => "5",
            LddKeyId::Six => "6",
            LddKeyId::Seven => "7",
            LddKeyId::Eight => "8",
            LddKeyId::Nine => "9",
            _ => &f,
        };
        str.to_owned()
    }
    
}

impl From<KeyId> for LddKeyId {
    fn from(key_id: KeyId) -> Self {
        match key_id {
            KeyId::Escape => LddKeyId::Escape,
            KeyId::Return => LddKeyId::Return,
            KeyId::Backspace => LddKeyId::Backspace,
            KeyId::Left => LddKeyId::Left,
            KeyId::Right => LddKeyId::Right,
            KeyId::Up => LddKeyId::Up,
            KeyId::Down => LddKeyId::Down,
            KeyId::Space => LddKeyId::Space,
            KeyId::A => LddKeyId::A,
            KeyId::B => LddKeyId::B,
            KeyId::C => LddKeyId::C,
            KeyId::D => LddKeyId::D,
            KeyId::E => LddKeyId::E,
            KeyId::F => LddKeyId::F,
            KeyId::G => LddKeyId::G,
            KeyId::H => LddKeyId::H,
            KeyId::I => LddKeyId::I,
            KeyId::J => LddKeyId::J,
            KeyId::K => LddKeyId::K,
            KeyId::L => LddKeyId::L,
            KeyId::M => LddKeyId::M,
            KeyId::N => LddKeyId::N,
            KeyId::O => LddKeyId::O,
            KeyId::P => LddKeyId::P,
            KeyId::Q => LddKeyId::Q,
            KeyId::R => LddKeyId::R,
            KeyId::S => LddKeyId::S,
            KeyId::T => LddKeyId::T,
            KeyId::U => LddKeyId::U,
            KeyId::V => LddKeyId::V,
            KeyId::W => LddKeyId::W,
            KeyId::X => LddKeyId::X,
            KeyId::Y => LddKeyId::Y,
            KeyId::Z => LddKeyId::Z,
            KeyId::F1 => LddKeyId::F1,
            KeyId::F2 => LddKeyId::F2,
            KeyId::F3 => LddKeyId::F3,
            KeyId::F4 => LddKeyId::F4,
            KeyId::F5 => LddKeyId::F5,
            KeyId::F6 => LddKeyId::F6,
            KeyId::F7 => LddKeyId::F7,
            KeyId::F8 => LddKeyId::F8,
            KeyId::F9 => LddKeyId::F9,
            KeyId::F10 => LddKeyId::F10,
            KeyId::F11 => LddKeyId::F11,
            KeyId::F12 => LddKeyId::F12,
            KeyId::Zero => LddKeyId::Zero,
            KeyId::One => LddKeyId::One,
            KeyId::Two => LddKeyId::Two,
            KeyId::Three => LddKeyId::Three,
            KeyId::Four => LddKeyId::Four,
            KeyId::Five => LddKeyId::Five,
            KeyId::Six => LddKeyId::Six,
            KeyId::Seven => LddKeyId::Seven,
            KeyId::Eight => LddKeyId::Eight,
            KeyId::Nine => LddKeyId::Nine,
            KeyId::Shift => LddKeyId::Shift,
            KeyId::LeftCtrl => LddKeyId::LeftCtrl,
            KeyId::RightCtrl => LddKeyId::RightCtrl,
            KeyId::LeftAlt => LddKeyId::LeftAlt,
            KeyId::RightAlt => LddKeyId::RightAlt,
            KeyId::CapsLock => LddKeyId::CapsLock,
            KeyId::Pause => LddKeyId::Pause,
            KeyId::PageUp => LddKeyId::PageUp,
            KeyId::PageDown => LddKeyId::PageDown,
            KeyId::PrintScreen => LddKeyId::PrintScreen,
            KeyId::Insert => LddKeyId::Insert,
            KeyId::End => LddKeyId::End,
            KeyId::Home => LddKeyId::Home,
            KeyId::Delete => LddKeyId::Delete,
            KeyId::Add => LddKeyId::Add,
            KeyId::Subtract => LddKeyId::Subtract,
            KeyId::Multiply => LddKeyId::Multiply,
            KeyId::Separator => LddKeyId::Separator,
            KeyId::Decimal => LddKeyId::Decimal,
            KeyId::Divide => LddKeyId::Divide,
            KeyId::BackTick => LddKeyId::BackTick,
            KeyId::BackSlash => LddKeyId::BackSlash,
            KeyId::ForwardSlash => LddKeyId::ForwardSlash,
            KeyId::Plus => LddKeyId::Plus,
            KeyId::Minus => LddKeyId::Minus,
            KeyId::FullStop => LddKeyId::FullStop,
            KeyId::Comma => LddKeyId::Comma,
            KeyId::Tab => LddKeyId::Tab,
            KeyId::Numlock => LddKeyId::Numlock,
            KeyId::LeftSquareBracket => LddKeyId::LeftSquareBracket,
            KeyId::RightSquareBracket => LddKeyId::RightSquareBracket,
            KeyId::SemiColon => LddKeyId::SemiColon,
            KeyId::Apostrophe => LddKeyId::Apostrophe,
            KeyId::Hash => LddKeyId::Hash,
        }
    }
}