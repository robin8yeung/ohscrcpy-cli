use byteorder::{BigEndian, WriteBytesExt};

/// 控制子类型，对应协议 0x10 帧的 subType 字节
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum ControlSubType {
    TouchDown  = 0x01,
    TouchMove  = 0x02,
    TouchUp    = 0x03,
    KeyBack    = 0x10,
    KeyEvent   = 0x14,
    TextInput  = 0x15,
}

/// 编码触摸按下事件（设备像素坐标，uint32 BE）
pub fn encode_touch_down(x: u32, y: u32, pointer_id: u16) -> Vec<u8> {
    encode_touch(ControlSubType::TouchDown, x, y, pointer_id)
}

/// 编码触摸移动事件（设备像素坐标，uint32 BE）
pub fn encode_touch_move(x: u32, y: u32, pointer_id: u16) -> Vec<u8> {
    encode_touch(ControlSubType::TouchMove, x, y, pointer_id)
}

/// 编码触摸抬起事件（设备像素坐标，uint32 BE）
pub fn encode_touch_up(x: u32, y: u32, pointer_id: u16) -> Vec<u8> {
    encode_touch(ControlSubType::TouchUp, x, y, pointer_id)
}

/// 编码返回键事件（backKey subType=0x13，与 Flutter 对齐）
pub fn encode_key_back() -> Vec<u8> {
    vec![0x13u8] // subType: BACK_KEY，body 为空
}

/// 编码实时按键事件（KEY_EVENT, subType=0x14）
/// 协议格式: subType(1B) + isPressed(1B) + keyCode(4B BE)
pub fn encode_key_event(keycode: u32, is_pressed: bool) -> Vec<u8> {
    let mut buf = Vec::with_capacity(6);
    buf.push(ControlSubType::KeyEvent as u8);
    buf.push(if is_pressed { 1 } else { 0 });
    buf.write_u32::<BigEndian>(keycode).unwrap();
    buf
}

/// 编码文本输入（TEXT_INPUT, subType=0x15）
/// 协议格式: subType(1B) + UTF-8 text bytes
pub fn encode_text_input(text: &str) -> Vec<u8> {
    let bytes = text.as_bytes();
    let mut buf = Vec::with_capacity(1 + bytes.len());
    buf.push(ControlSubType::TextInput as u8);
    buf.extend_from_slice(bytes);
    buf
}

/// 编码视频参数配置（changeVideoParams, subType=0x42）
/// body: maxShort(4 BE) + bitrate(4 BE) + frameRate(4 BE)
pub fn encode_video_params(max_short: u32, bitrate: u32, frame_rate: u32) -> Vec<u8> {
    let mut buf = Vec::with_capacity(1 + 12);
    buf.push(0x42u8); // subType: CHANGE_VIDEO_PARAMS
    buf.write_u32::<BigEndian>(max_short).unwrap();
    buf.write_u32::<BigEndian>(bitrate).unwrap();
    buf.write_u32::<BigEndian>(frame_rate).unwrap();
    buf
}

/// 协议格式: subType(1) + x(4 BE uint32) + y(4 BE uint32) + pointerId(2 BE uint16) = 11 bytes
fn encode_touch(sub_type: ControlSubType, x: u32, y: u32, pointer_id: u16) -> Vec<u8> {
    let mut buf = Vec::with_capacity(1 + 4 + 4 + 2);
    buf.push(sub_type as u8);
    buf.write_u32::<BigEndian>(x).unwrap();
    buf.write_u32::<BigEndian>(y).unwrap();
    buf.write_u16::<BigEndian>(pointer_id).unwrap();
    buf
}

/// 将 SDL2 Scancode（物理键位）映射为 OpenHarmony KeyCode
/// 返回 None 表示该键无需转发（如未知的多媒体键）
pub fn sdl_to_oh_keycode(scancode: sdl2::keyboard::Scancode) -> Option<u32> {
    use sdl2::keyboard::Scancode;
    match scancode {
        // 字母 A-Z: 2017-2042
        Scancode::A => Some(2017),
        Scancode::B => Some(2018),
        Scancode::C => Some(2019),
        Scancode::D => Some(2020),
        Scancode::E => Some(2021),
        Scancode::F => Some(2022),
        Scancode::G => Some(2023),
        Scancode::H => Some(2024),
        Scancode::I => Some(2025),
        Scancode::J => Some(2026),
        Scancode::K => Some(2027),
        Scancode::L => Some(2028),
        Scancode::M => Some(2029),
        Scancode::N => Some(2030),
        Scancode::O => Some(2031),
        Scancode::P => Some(2032),
        Scancode::Q => Some(2033),
        Scancode::R => Some(2034),
        Scancode::S => Some(2035),
        Scancode::T => Some(2036),
        Scancode::U => Some(2037),
        Scancode::V => Some(2038),
        Scancode::W => Some(2039),
        Scancode::X => Some(2040),
        Scancode::Y => Some(2041),
        Scancode::Z => Some(2042),

        // 数字 0-9: 2000-2009
        Scancode::Num0 => Some(2000),
        Scancode::Num1 => Some(2001),
        Scancode::Num2 => Some(2002),
        Scancode::Num3 => Some(2003),
        Scancode::Num4 => Some(2004),
        Scancode::Num5 => Some(2005),
        Scancode::Num6 => Some(2006),
        Scancode::Num7 => Some(2007),
        Scancode::Num8 => Some(2008),
        Scancode::Num9 => Some(2009),

        // 方向键
        Scancode::Up    => Some(2012),
        Scancode::Down  => Some(2013),
        Scancode::Left  => Some(2014),
        Scancode::Right => Some(2015),

        // 功能键 F1-F12: 2090-2101
        Scancode::F1  => Some(2090),
        Scancode::F2  => Some(2091),
        Scancode::F3  => Some(2092),
        Scancode::F4  => Some(2093),
        Scancode::F5  => Some(2094),
        Scancode::F6  => Some(2095),
        Scancode::F7  => Some(2096),
        Scancode::F8  => Some(2097),
        Scancode::F9  => Some(2098),
        Scancode::F10 => Some(2099),
        Scancode::F11 => Some(2100),
        Scancode::F12 => Some(2101),

        // 特殊键
        Scancode::Space     => Some(2050),
        Scancode::Return    => Some(2054),
        Scancode::Tab       => Some(2049),
        Scancode::Escape    => Some(2070),
        Scancode::Backspace => Some(2055),
        Scancode::Delete    => Some(2071),

        // 修饰键
        Scancode::LShift => Some(2047),
        Scancode::RShift => Some(2048),
        Scancode::LCtrl  => Some(2072),
        Scancode::RCtrl  => Some(2073),
        Scancode::LAlt   => Some(2045),
        Scancode::RAlt   => Some(2046),
        Scancode::LGui   => Some(2076),
        Scancode::RGui   => Some(2077),
        Scancode::CapsLock      => Some(2074),
        Scancode::NumLockClear  => Some(2102),
        Scancode::ScrollLock    => Some(2075),

        // 符号键
        Scancode::Minus        => Some(2060),
        Scancode::Equals       => Some(2061),
        Scancode::LeftBracket  => Some(2056),
        Scancode::RightBracket => Some(2057),
        Scancode::Backslash    => Some(2058),
        Scancode::Semicolon    => Some(2062),
        Scancode::Apostrophe   => Some(2063),
        Scancode::Slash        => Some(2064),
        Scancode::Comma        => Some(2043),
        Scancode::Period       => Some(2044),
        Scancode::Grave        => Some(2059),

        // 导航键
        Scancode::Insert      => Some(2083),
        Scancode::Home        => Some(2081),
        Scancode::End         => Some(2082),
        Scancode::PageUp      => Some(2068),
        Scancode::PageDown    => Some(2069),
        Scancode::PrintScreen => Some(2079),

        // 小键盘
        Scancode::Kp0 => Some(2103),
        Scancode::Kp1 => Some(2104),
        Scancode::Kp2 => Some(2105),
        Scancode::Kp3 => Some(2106),
        Scancode::Kp4 => Some(2107),
        Scancode::Kp5 => Some(2108),
        Scancode::Kp6 => Some(2109),
        Scancode::Kp7 => Some(2110),
        Scancode::Kp8 => Some(2111),
        Scancode::Kp9 => Some(2112),
        Scancode::KpDivide    => Some(2113),
        Scancode::KpMultiply  => Some(2114),
        Scancode::KpMinus     => Some(2115),
        Scancode::KpPlus      => Some(2116),
        Scancode::KpPeriod    => Some(2117),
        Scancode::KpEnter     => Some(2119),

        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_touch_down_size() {
        let b = encode_touch_down(100, 200, 0);
        assert_eq!(b.len(), 11);
        assert_eq!(b[0], 0x01);
        // x = 100 = 0x00000064
        assert_eq!(&b[1..5], &[0x00, 0x00, 0x00, 0x64]);
        // y = 200 = 0x000000C8
        assert_eq!(&b[5..9], &[0x00, 0x00, 0x00, 0xC8]);
    }

    #[test]
    fn test_encode_key_back() {
        let b = encode_key_back();
        assert_eq!(b[0], 0x13);
    }

    #[test]
    fn test_encode_key_event_pressed() {
        let b = encode_key_event(2017, true);
        assert_eq!(b.len(), 6);
        assert_eq!(b[0], 0x14);
        assert_eq!(b[1], 1);
        // keyCode = 2017 = 0x000007E1
        assert_eq!(&b[2..6], &[0x00, 0x00, 0x07, 0xE1]);
    }

    #[test]
    fn test_encode_key_event_released() {
        let b = encode_key_event(2050, false);
        assert_eq!(b.len(), 6);
        assert_eq!(b[0], 0x14);
        assert_eq!(b[1], 0);
        // keyCode = 2050 = 0x00000802
        assert_eq!(&b[2..6], &[0x00, 0x00, 0x08, 0x02]);
    }

    #[test]
    fn test_encode_text_input_ascii() {
        let b = encode_text_input("hello");
        assert_eq!(b.len(), 6);
        assert_eq!(b[0], 0x15);
        assert_eq!(&b[1..], b"hello");
    }

    #[test]
    fn test_encode_text_input_utf8() {
        let b = encode_text_input("你好");
        assert_eq!(b[0], 0x15);
        assert_eq!(&b[1..], "你好".as_bytes());
        assert_eq!(b.len(), 1 + 6); // "你好" = 6 bytes in UTF-8
    }

    #[test]
    fn test_encode_text_input_empty() {
        let b = encode_text_input("");
        assert_eq!(b.len(), 1);
        assert_eq!(b[0], 0x15);
    }

    #[test]
    fn test_sdl_to_oh_keycode_letters() {
        use sdl2::keyboard::Scancode;
        assert_eq!(sdl_to_oh_keycode(Scancode::A), Some(2017));
        assert_eq!(sdl_to_oh_keycode(Scancode::Z), Some(2042));
        assert_eq!(sdl_to_oh_keycode(Scancode::M), Some(2029));
    }

    #[test]
    fn test_sdl_to_oh_keycode_numbers() {
        use sdl2::keyboard::Scancode;
        assert_eq!(sdl_to_oh_keycode(Scancode::Num0), Some(2000));
        assert_eq!(sdl_to_oh_keycode(Scancode::Num9), Some(2009));
    }

    #[test]
    fn test_sdl_to_oh_keycode_special() {
        use sdl2::keyboard::Scancode;
        assert_eq!(sdl_to_oh_keycode(Scancode::Space), Some(2050));
        assert_eq!(sdl_to_oh_keycode(Scancode::Return), Some(2054));
        assert_eq!(sdl_to_oh_keycode(Scancode::Escape), Some(2070));
    }

    #[test]
    fn test_sdl_to_oh_keycode_unknown() {
        use sdl2::keyboard::Scancode;
        // 一个不在映射表中的键（如 Pause）应返回 None
        assert_eq!(sdl_to_oh_keycode(Scancode::Pause), None);
    }
}
