pub mod keyboard {
    #[repr(u8)]
    #[derive(Debug, Clone, Copy)]
    pub enum Modifier {
        None = 0x00,
        LCtrl = 0x01,
        LShift = 0x02,
        LAlt = 0x04,
        LMeta = 0x08,
        RCtrl = 0x10,
        RShift = 0x20,
        RAlt = 0x40,
        RMeta = 0x80,
    }

    #[repr(u8)]
    #[derive(Debug, Clone, Copy)]
    pub enum Keycode {
        None = 0x00,
        A = 0x04,
        B = 0x05,
        C = 0x06,
        D = 0x07,
        E = 0x08,
        F = 0x09,
        G = 0x0a,
        H = 0x0b,
        I = 0x0c,
        J = 0x0d,
        K = 0x0e,
        L = 0x0f,
        M = 0x10,
        N = 0x11,
        O = 0x12,
        P = 0x13,
        Q = 0x14,
        R = 0x15,
        S = 0x16,
        T = 0x17,
        U = 0x18,
        V = 0x19,
        W = 0x1a,
        X = 0x1b,
        Y = 0x1c,
        Z = 0x1d,
        Num1 = 0x1e,
        Num2 = 0x1f,
        Num3 = 0x20,
        Num4 = 0x21,
        Num5 = 0x22,
        Num6 = 0x23,
        Num7 = 0x24,
        Num8 = 0x25,
        Num9 = 0x26,
        Num0 = 0x27,
        Enter = 0x28,
        Esc = 0x29,
        Backspace = 0x2a,
        Tab = 0x2b,
        Space = 0x2c,
        Minus = 0x2d,
        Equal = 0x2e,
        LeftBrace = 0x2f,
        RightBrace = 0x30,
        Backslash = 0x31,
        HashTilde = 0x32,
        Semicolon = 0x33,
        Apostrophe = 0x34,
        Grave = 0x35,
        Comma = 0x36,
        Dot = 0x37,
        Slash = 0x38,
        CapsLock = 0x39,
        F1 = 0x3a,
        F2 = 0x3b,
        F3 = 0x3c,
        F4 = 0x3d,
        F5 = 0x3e,
        F6 = 0x3f,
        F7 = 0x40,
        F8 = 0x41,
        F9 = 0x42,
        F10 = 0x43,
        F11 = 0x44,
        F12 = 0x45,
        SysRq = 0x46,
        ScrollLock = 0x47,
        Pause = 0x48,
        Insert = 0x49,
        Home = 0x4a,
        PageUp = 0x4b,
        Delete = 0x4c,
        End = 0x4d,
        PageDown = 0x4e,
        Right = 0x4f,
        Left = 0x50,
        Down = 0x51,
        Up = 0x52,
        NumLock = 0x53,
        KPDivide = 0x54,
        KPMultiply = 0x55,
        KPMinus = 0x56,
        KPPlus = 0x57,
        KPEnter = 0x58,
        KP1 = 0x59,
        KP2 = 0x5a,
        KP3 = 0x5b,
        KP4 = 0x5c,
        KP5 = 0x5d,
        KP6 = 0x5e,
        KP7 = 0x5f,
        KP8 = 0x60,
        KP9 = 0x61,
        KP0 = 0x62,
        KPDot = 0x63,
        Key102nd = 0x64,
        Compose = 0x65,
        LeftCtrl = 0xe0,
        LeftShift = 0xe1,
        LeftAlt = 0xe2,
        LeftMeta = 0xe3,
        RightCtrl = 0xe4,
        RightShift = 0xe5,
        RightAlt = 0xe6,
        RightMeta = 0xe7,
    }
    #[link(name = "cper", kind = "dylib")]
    extern "C" {
        fn KeyDown(keycodes: *mut u8, modifier: u8);
        fn KeyUp();
    }
    pub fn press_key(keycodes: Vec<Keycode>, modifier: Modifier) {
        let keycodes: Vec<u8> = keycodes.into_iter().map(|k| k as u8).collect();
        let mut mask: [u8; 6] = [0; 6];
        mask[..keycodes.len()].copy_from_slice(&keycodes);
        unsafe {
            KeyDown(mask.as_mut_ptr(), modifier as u8);
        }
    }
    pub fn release_key() {
        unsafe {
            KeyUp()
        }
    }
}
