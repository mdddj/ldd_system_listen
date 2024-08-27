use std::{
    thread,
    time::{Duration, Instant},
};

use crate::frb_generated::StreamSink;
use rdev::{listen, Button, Event, EventType, Key};

pub fn start_listen_system_event(sink: StreamSink<LddEvent>) -> Result<(), String> {
    let r = listen(move |event| {
        let ldd: LddEvent = event.into();
        let _ = sink.add(ldd);
    });
    match r {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("{:?}", e)),
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LddKeyboardValue {
    ScanGunValue(String, u128),
    KeyboardValue(LddEvent, u128),
    ScanGunValueV2(Vec<LddEvent>),
    KeyboardValueV2(LddEvent),
}

/// [millis] 设置字符输入之间的最大时间间隔，小于此时间则认为是连续输入
/// [min_size] 假设扫码枪至少输入 10 个字符以上，作为判定标准之一
pub fn start_listen_systen_event_by_ldd(
    sink: StreamSink<LddKeyboardValue>,
    millis: Option<u64>,
    min_size: Option<usize>,
) {
    let mut last_event_time = Instant::now();
    let mut input_buffer = String::new();

    // 使用 10 作为默认值，如果传入 None 则使用默认值
    let millis = millis.unwrap_or(10);
    let min_size = min_size.unwrap_or(10);

    let mut first_char: Option<char> = None;
    let mut is_gun = false;
    let mut is_waiting = false; //是否正在等待处理中...如果等待处理中,直接不处理这次事件.

    if let Err(error) = listen(move |event| {
        //等待判断上次输入事件
        if !is_waiting {
            return;
        }
        let ldd_event: LddEvent = event.clone().into();
        let _ = callback(
            event,
            &mut last_event_time,
            &mut input_buffer,
            &mut first_char,
            millis,
            min_size,
            &mut is_waiting,
            &mut is_gun,
            |v, d| {
                let mill = d.as_millis();
                let _ = sink.add(LddKeyboardValue::ScanGunValue(v, mill));
            },
            |d| {
                let _ = sink.add(LddKeyboardValue::KeyboardValue(
                    ldd_event.clone(),
                    d.as_millis(),
                ));
            },
        );
    }) {
        // println!("Error: {:?}", error);
        // 可以在这里添加进一步的错误处理逻辑，如重试机制或安全退出
        panic!("error:{:?}", error);
    }
}
async fn callback<F, K>(
    event: Event,
    last_event_time: &mut Instant,
    input_buffer: &mut String,
    first_char: &mut Option<char>,
    millis: u64,
    min_size: usize,
    is_waiting: &mut bool,
    is_gun: &mut bool,
    call: F,
    keyboard_call: K,
) where
    F: Fn(String, Duration),
    K: Fn(Duration),
{
    let threshold = Duration::from_millis(millis);

    if let EventType::KeyPress(key) = event.event_type {
        let now = Instant::now();
        let duration = now.duration_since(*last_event_time); //阈值

        // 间隔时间大于阈值,清空缓存,标注键盘输入
        if duration >= threshold {
            //键盘操作
            input_buffer.clear();
            *is_gun = false;
            *first_char = None;
        }

        *last_event_time = now;

        if let Some(key_char) = event.name.and_then(|name| name.chars().next()) {
            // 如果输入间隔小于阈值，判断为扫码枪输入
            if duration < threshold {
                if input_buffer.is_empty() {
                    *first_char = Some(key_char);
                }
                input_buffer.push(key_char);
                *is_gun = true;
            }

            if key == Key::Return && *is_gun {
                if !input_buffer.is_empty() {
                    // 在这里处理扫码枪的输入
                    call(input_buffer.clone(), duration);
                }
                input_buffer.clear();
                *is_gun = false;
                *first_char = None;
            } else if !*is_gun {
                // 处理键盘输入
                keyboard_call(duration);
                input_buffer.clear();
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LddEvent {
    pub name: Option<String>,
    pub event_type: LddEventType,
}

impl From<Event> for LddEvent {
    fn from(event: Event) -> Self {
        LddEvent {
            name: event.name,
            event_type: event.event_type.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LddEventType {
    KeyPress(LddKey),
    KeyRelease(LddKey),
    ButtonPress(LddButton),
    ButtonRelease(LddButton),
    MouseMove { x: f64, y: f64 },
    Wheel { delta_x: i64, delta_y: i64 },
}

impl From<EventType> for LddEventType {
    fn from(event_type: EventType) -> Self {
        match event_type {
            EventType::KeyPress(key) => LddEventType::KeyPress(key.into()),
            EventType::KeyRelease(key) => LddEventType::KeyRelease(key.into()),
            EventType::ButtonPress(button) => LddEventType::ButtonPress(button.into()),
            EventType::ButtonRelease(button) => LddEventType::ButtonRelease(button.into()),
            EventType::MouseMove { x, y } => LddEventType::MouseMove { x, y },
            EventType::Wheel { delta_x, delta_y } => LddEventType::Wheel { delta_x, delta_y },
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LddButton {
    Left,
    Right,
    Middle,
    Unknown(u8),
}
impl From<Button> for LddButton {
    fn from(button: Button) -> Self {
        match button {
            Button::Left => LddButton::Left,
            Button::Right => LddButton::Right,
            Button::Middle => LddButton::Middle,
            Button::Unknown(val) => LddButton::Unknown(val),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LddKey {
    Alt,
    AltGr,
    Backspace,
    CapsLock,
    ControlLeft,
    ControlRight,
    Delete,
    DownArrow,
    End,
    Escape,
    F1,
    F10,
    F11,
    F12,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    Home,
    LeftArrow,
    MetaLeft,
    MetaRight,
    PageDown,
    PageUp,
    Return,
    RightArrow,
    ShiftLeft,
    ShiftRight,
    Space,
    Tab,
    UpArrow,
    PrintScreen,
    ScrollLock,
    Pause,
    NumLock,
    BackQuote,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    Num0,
    Minus,
    Equal,
    KeyQ,
    KeyW,
    KeyE,
    KeyR,
    KeyT,
    KeyY,
    KeyU,
    KeyI,
    KeyO,
    KeyP,
    LeftBracket,
    RightBracket,
    KeyA,
    KeyS,
    KeyD,
    KeyF,
    KeyG,
    KeyH,
    KeyJ,
    KeyK,
    KeyL,
    SemiColon,
    Quote,
    BackSlash,
    IntlBackslash,
    KeyZ,
    KeyX,
    KeyC,
    KeyV,
    KeyB,
    KeyN,
    KeyM,
    Comma,
    Dot,
    Slash,
    Insert,
    KpReturn,
    KpMinus,
    KpPlus,
    KpMultiply,
    KpDivide,
    Kp0,
    Kp1,
    Kp2,
    Kp3,
    Kp4,
    Kp5,
    Kp6,
    Kp7,
    Kp8,
    Kp9,
    KpDelete,
    Function,
    Unknown(u32),
}

impl From<Key> for LddKey {
    fn from(key: Key) -> Self {
        match key {
            Key::Alt => LddKey::Alt,
            Key::AltGr => LddKey::AltGr,
            Key::Backspace => LddKey::Backspace,
            Key::CapsLock => LddKey::CapsLock,
            Key::ControlLeft => LddKey::ControlLeft,
            Key::ControlRight => LddKey::ControlRight,
            Key::Delete => LddKey::Delete,
            Key::DownArrow => LddKey::DownArrow,
            Key::End => LddKey::End,
            Key::Escape => LddKey::Escape,
            Key::F1 => LddKey::F1,
            Key::F10 => LddKey::F10,
            Key::F11 => LddKey::F11,
            Key::F12 => LddKey::F12,
            Key::F2 => LddKey::F2,
            Key::F3 => LddKey::F3,
            Key::F4 => LddKey::F4,
            Key::F5 => LddKey::F5,
            Key::F6 => LddKey::F6,
            Key::F7 => LddKey::F7,
            Key::F8 => LddKey::F8,
            Key::F9 => LddKey::F9,
            Key::Home => LddKey::Home,
            Key::LeftArrow => LddKey::LeftArrow,
            Key::MetaLeft => LddKey::MetaLeft,
            Key::MetaRight => LddKey::MetaRight,
            Key::PageDown => LddKey::PageDown,
            Key::PageUp => LddKey::PageUp,
            Key::Return => LddKey::Return,
            Key::RightArrow => LddKey::RightArrow,
            Key::ShiftLeft => LddKey::ShiftLeft,
            Key::ShiftRight => LddKey::ShiftRight,
            Key::Space => LddKey::Space,
            Key::Tab => LddKey::Tab,
            Key::UpArrow => LddKey::UpArrow,
            Key::PrintScreen => LddKey::PrintScreen,
            Key::ScrollLock => LddKey::ScrollLock,
            Key::Pause => LddKey::Pause,
            Key::NumLock => LddKey::NumLock,
            Key::BackQuote => LddKey::BackQuote,
            Key::Num1 => LddKey::Num1,
            Key::Num2 => LddKey::Num2,
            Key::Num3 => LddKey::Num3,
            Key::Num4 => LddKey::Num4,
            Key::Num5 => LddKey::Num5,
            Key::Num6 => LddKey::Num6,
            Key::Num7 => LddKey::Num7,
            Key::Num8 => LddKey::Num8,
            Key::Num9 => LddKey::Num9,
            Key::Num0 => LddKey::Num0,
            Key::Minus => LddKey::Minus,
            Key::Equal => LddKey::Equal,
            Key::KeyQ => LddKey::KeyQ,
            Key::KeyW => LddKey::KeyW,
            Key::KeyE => LddKey::KeyE,
            Key::KeyR => LddKey::KeyR,
            Key::KeyT => LddKey::KeyT,
            Key::KeyY => LddKey::KeyY,
            Key::KeyU => LddKey::KeyU,
            Key::KeyI => LddKey::KeyI,
            Key::KeyO => LddKey::KeyO,
            Key::KeyP => LddKey::KeyP,
            Key::LeftBracket => LddKey::LeftBracket,
            Key::RightBracket => LddKey::RightBracket,
            Key::KeyA => LddKey::KeyA,
            Key::KeyS => LddKey::KeyS,
            Key::KeyD => LddKey::KeyD,
            Key::KeyF => LddKey::KeyF,
            Key::KeyG => LddKey::KeyG,
            Key::KeyH => LddKey::KeyH,
            Key::KeyJ => LddKey::KeyJ,
            Key::KeyK => LddKey::KeyK,
            Key::KeyL => LddKey::KeyL,
            Key::SemiColon => LddKey::SemiColon,
            Key::Quote => LddKey::Quote,
            Key::BackSlash => LddKey::BackSlash,
            Key::IntlBackslash => LddKey::IntlBackslash,
            Key::KeyZ => LddKey::KeyZ,
            Key::KeyX => LddKey::KeyX,
            Key::KeyC => LddKey::KeyC,
            Key::KeyV => LddKey::KeyV,
            Key::KeyB => LddKey::KeyB,
            Key::KeyN => LddKey::KeyN,
            Key::KeyM => LddKey::KeyM,
            Key::Comma => LddKey::Comma,
            Key::Dot => LddKey::Dot,
            Key::Slash => LddKey::Slash,
            Key::Insert => LddKey::Insert,
            Key::KpReturn => LddKey::KpReturn,
            Key::KpMinus => LddKey::KpMinus,
            Key::KpPlus => LddKey::KpPlus,
            Key::KpMultiply => LddKey::KpMultiply,
            Key::KpDivide => LddKey::KpDivide,
            Key::Kp0 => LddKey::Kp0,
            Key::Kp1 => LddKey::Kp1,
            Key::Kp2 => LddKey::Kp2,
            Key::Kp3 => LddKey::Kp3,
            Key::Kp4 => LddKey::Kp4,
            Key::Kp5 => LddKey::Kp5,
            Key::Kp6 => LddKey::Kp6,
            Key::Kp7 => LddKey::Kp7,
            Key::Kp8 => LddKey::Kp8,
            Key::Kp9 => LddKey::Kp9,
            Key::KpDelete => LddKey::KpDelete,
            Key::Function => LddKey::Function,
            Key::Unknown(val) => LddKey::Unknown(val),
        }
    }
}
