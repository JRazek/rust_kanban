use crossterm::event;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::fmt::{self, Display, Formatter};

/// Represents an key.
#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug, Serialize, Deserialize)]
pub enum Key {
    Enter,
    Tab,
    Backspace,
    Esc,
    Space,
    Left,
    Right,
    Up,
    Down,
    Ins,
    Delete,
    Home,
    End,
    PageUp,
    PageDown,
    F0,
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
    Char(char),
    Ctrl(char),
    Alt(char),
    BackTab,
    ShiftUp,
    ShiftDown,
    ShiftLeft,
    ShiftRight,
    Unknown,
}

impl Key {
    pub fn from_f(n: u8) -> Key {
        match n {
            0 => Key::F0,
            1 => Key::F1,
            2 => Key::F2,
            3 => Key::F3,
            4 => Key::F4,
            5 => Key::F5,
            6 => Key::F6,
            7 => Key::F7,
            8 => Key::F8,
            9 => Key::F9,
            10 => Key::F10,
            11 => Key::F11,
            12 => Key::F12,
            _ => panic!("unknown function key: F{}", n),
        }
    }
    pub fn to_digit(&self) -> u8 {
        // check if char is a digit if so return it
        match self {
            Key::Char(c) => c.to_digit(10).unwrap() as u8,
            _ => panic!("not a digit"),
        }
    }
}

impl Display for Key {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Key::Alt(' ') => write!(f, "<Alt+Space>"),
            Key::Ctrl(' ') => write!(f, "<Ctrl+Space>"),
            Key::Char(' ') => write!(f, "<Space>"),
            Key::Alt(c) => write!(f, "<Alt+{}>", c),
            Key::Ctrl(c) => write!(f, "<Ctrl+{}>", c),
            Key::Char(c) => write!(f, "<{}>", c),
            Key::Tab => write!(f, "<Tab>"),
            Key::BackTab => write!(f, "<Shift+Tab>"),
            Key::ShiftUp => write!(f, "<Shift+Up>"),
            Key::ShiftDown => write!(f, "<Shift+Down>"),
            Key::ShiftLeft => write!(f, "<Shift+Left>"),
            Key::ShiftRight => write!(f, "<Shift+Right>"),
            _ => write!(f, "<{:?}>", self),
        }
    }
}

impl From<event::KeyEvent> for Key {
    fn from(key_event: event::KeyEvent) -> Self {
        match key_event {
            event::KeyEvent {
                code: event::KeyCode::Esc,
                kind: event::KeyEventKind::Press,
                ..
            } => Key::Esc,
            event::KeyEvent {
                code: event::KeyCode::Backspace,
                kind: event::KeyEventKind::Press,
                ..
            } => Key::Backspace,
            event::KeyEvent {
                code: event::KeyCode::Left,
                kind: event::KeyEventKind::Press,
                modifiers: event::KeyModifiers::SHIFT,
                ..
            } => Key::ShiftLeft,
            event::KeyEvent {
                code: event::KeyCode::Left,
                kind: event::KeyEventKind::Press,
                ..
            } => Key::Left,
            event::KeyEvent {
                code: event::KeyCode::Right,
                kind: event::KeyEventKind::Press,
                modifiers: event::KeyModifiers::SHIFT,
                ..
            } => Key::ShiftRight,
            event::KeyEvent {
                code: event::KeyCode::Right,
                kind: event::KeyEventKind::Press,
                ..
            } => Key::Right,
            event::KeyEvent {
                code: event::KeyCode::Up,
                kind: event::KeyEventKind::Press,
                modifiers: event::KeyModifiers::SHIFT,
                ..
            } => Key::ShiftUp,
            event::KeyEvent {
                code: event::KeyCode::Up,
                kind: event::KeyEventKind::Press,
                ..
            } => Key::Up,
            event::KeyEvent {
                code: event::KeyCode::Down,
                kind: event::KeyEventKind::Press,
                modifiers: event::KeyModifiers::SHIFT,
                ..
            } => Key::ShiftDown,
            event::KeyEvent {
                code: event::KeyCode::Down,
                kind: event::KeyEventKind::Press,
                ..
            } => Key::Down,
            event::KeyEvent {
                code: event::KeyCode::Home,
                kind: event::KeyEventKind::Press,
                ..
            } => Key::Home,
            event::KeyEvent {
                code: event::KeyCode::End,
                kind: event::KeyEventKind::Press,
                ..
            } => Key::End,
            event::KeyEvent {
                code: event::KeyCode::PageUp,
                kind: event::KeyEventKind::Press,
                ..
            } => Key::PageUp,
            event::KeyEvent {
                code: event::KeyCode::PageDown,
                kind: event::KeyEventKind::Press,
                ..
            } => Key::PageDown,
            event::KeyEvent {
                code: event::KeyCode::Delete,
                kind: event::KeyEventKind::Press,
                ..
            } => Key::Delete,
            event::KeyEvent {
                code: event::KeyCode::Insert,
                kind: event::KeyEventKind::Press,
                ..
            } => Key::Ins,
            event::KeyEvent {
                code: event::KeyCode::F(n),
                kind: event::KeyEventKind::Press,
                ..
            } => Key::from_f(n),
            event::KeyEvent {
                code: event::KeyCode::Enter,
                kind: event::KeyEventKind::Press,
                ..
            } => Key::Enter,
            event::KeyEvent {
                code: event::KeyCode::BackTab,
                kind: event::KeyEventKind::Press,
                modifiers: event::KeyModifiers::SHIFT,
                ..
            } => Key::BackTab,
            event::KeyEvent {
                code: event::KeyCode::Tab,
                kind: event::KeyEventKind::Press,
                ..
            } => Key::Tab,
            event::KeyEvent {
                code: event::KeyCode::Char(c),
                kind: event::KeyEventKind::Press,
                modifiers: event::KeyModifiers::ALT,
                ..
            } => Key::Alt(c),
            event::KeyEvent {
                code: event::KeyCode::Char(c),
                kind: event::KeyEventKind::Press,
                modifiers: event::KeyModifiers::CONTROL,
                ..
            } => Key::Ctrl(c),
            event::KeyEvent {
                code: event::KeyCode::Char(c),
                kind: event::KeyEventKind::Press,
                ..
            } => Key::Char(c),
            _ => Key::Unknown,
        }
    }
}

impl From<&str> for Key {
    fn from(s: &str) -> Self {
        match s {
            "Enter" => Key::Enter,
            "Tab" => Key::Tab,
            "Backspace" => Key::Backspace,
            "Esc" => Key::Esc,
            "Space" => Key::Space,
            "Left" => Key::Left,
            "Right" => Key::Right,
            "Up" => Key::Up,
            "Down" => Key::Down,
            "Ins" => Key::Ins,
            "Delete" => Key::Delete,
            "Home" => Key::Home,
            "End" => Key::End,
            "PageUp" => Key::PageUp,
            "PageDown" => Key::PageDown,
            "F0" => Key::F0,
            "F1" => Key::F1,
            "F2" => Key::F2,
            "F3" => Key::F3,
            "F4" => Key::F4,
            "F5" => Key::F5,
            "F6" => Key::F6,
            "F7" => Key::F7,
            "F8" => Key::F8,
            "F9" => Key::F9,
            "F10" => Key::F10,
            "F11" => Key::F11,
            "F12" => Key::F12,
            "BackTab" => Key::BackTab,
            "ShiftUp" => Key::ShiftUp,
            "ShiftDown" => Key::ShiftDown,
            "ShiftLeft" => Key::ShiftLeft,
            "ShiftRight" => Key::ShiftRight,
            _ => Key::Unknown,
        }
    }
}

impl From<&Map<String, Value>> for Key {
    fn from(value: &Map<String, Value>) -> Self {
        if value.get("Char").is_some() {
            Key::Char(
                value
                    .get("Char")
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .chars()
                    .next()
                    .unwrap(),
            )
        } else if value.get("Alt").is_some() {
            Key::Alt(
                value
                    .get("Alt")
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .chars()
                    .next()
                    .unwrap(),
            )
        } else if value.get("Ctrl").is_some() {
            Key::Ctrl(
                value
                    .get("Ctrl")
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .chars()
                    .next()
                    .unwrap(),
            )
        } else {
            Key::Unknown
        }
    }
}
