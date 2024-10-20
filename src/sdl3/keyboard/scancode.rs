#![allow(unreachable_patterns)]

use libc::c_char;
use std::ffi::{CStr, CString};
use std::mem::transmute;

use crate::sys;
use crate::sys::scancode::*;

#[repr(i32)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Scancode {
    Unknown = SDL_SCANCODE_UNKNOWN.0,
    A = SDL_SCANCODE_A.0,
    B = SDL_SCANCODE_B.0,
    C = SDL_SCANCODE_C.0,
    D = SDL_SCANCODE_D.0,
    E = SDL_SCANCODE_E.0,
    F = SDL_SCANCODE_F.0,
    G = SDL_SCANCODE_G.0,
    H = SDL_SCANCODE_H.0,
    I = SDL_SCANCODE_I.0,
    J = SDL_SCANCODE_J.0,
    K = SDL_SCANCODE_K.0,
    L = SDL_SCANCODE_L.0,
    M = SDL_SCANCODE_M.0,
    N = SDL_SCANCODE_N.0,
    O = SDL_SCANCODE_O.0,
    P = SDL_SCANCODE_P.0,
    Q = SDL_SCANCODE_Q.0,
    R = SDL_SCANCODE_R.0,
    S = SDL_SCANCODE_S.0,
    T = SDL_SCANCODE_T.0,
    U = SDL_SCANCODE_U.0,
    V = SDL_SCANCODE_V.0,
    W = SDL_SCANCODE_W.0,
    X = SDL_SCANCODE_X.0,
    Y = SDL_SCANCODE_Y.0,
    Z = SDL_SCANCODE_Z.0,
    _1 = SDL_SCANCODE_1.0,
    _2 = SDL_SCANCODE_2.0,
    _3 = SDL_SCANCODE_3.0,
    _4 = SDL_SCANCODE_4.0,
    _5 = SDL_SCANCODE_5.0,
    _6 = SDL_SCANCODE_6.0,
    _7 = SDL_SCANCODE_7.0,
    _8 = SDL_SCANCODE_8.0,
    _9 = SDL_SCANCODE_9.0,
    _0 = SDL_SCANCODE_0.0,
    Return = SDL_SCANCODE_RETURN.0,
    Escape = SDL_SCANCODE_ESCAPE.0,
    Backspace = SDL_SCANCODE_BACKSPACE.0,
    Tab = SDL_SCANCODE_TAB.0,
    Space = SDL_SCANCODE_SPACE.0,
    Minus = SDL_SCANCODE_MINUS.0,
    Equals = SDL_SCANCODE_EQUALS.0,
    LeftBracket = SDL_SCANCODE_LEFTBRACKET.0,
    RightBracket = SDL_SCANCODE_RIGHTBRACKET.0,
    Backslash = SDL_SCANCODE_BACKSLASH.0,
    NonUsHash = SDL_SCANCODE_NONUSHASH.0,
    Semicolon = SDL_SCANCODE_SEMICOLON.0,
    Apostrophe = SDL_SCANCODE_APOSTROPHE.0,
    Grave = SDL_SCANCODE_GRAVE.0,
    Comma = SDL_SCANCODE_COMMA.0,
    Period = SDL_SCANCODE_PERIOD.0,
    Slash = SDL_SCANCODE_SLASH.0,
    CapsLock = SDL_SCANCODE_CAPSLOCK.0,
    F1 = SDL_SCANCODE_F1.0,
    F2 = SDL_SCANCODE_F2.0,
    F3 = SDL_SCANCODE_F3.0,
    F4 = SDL_SCANCODE_F4.0,
    F5 = SDL_SCANCODE_F5.0,
    F6 = SDL_SCANCODE_F6.0,
    F7 = SDL_SCANCODE_F7.0,
    F8 = SDL_SCANCODE_F8.0,
    F9 = SDL_SCANCODE_F9.0,
    F10 = SDL_SCANCODE_F10.0,
    F11 = SDL_SCANCODE_F11.0,
    F12 = SDL_SCANCODE_F12.0,
    PrintScreen = SDL_SCANCODE_PRINTSCREEN.0,
    ScrollLock = SDL_SCANCODE_SCROLLLOCK.0,
    Pause = SDL_SCANCODE_PAUSE.0,
    Insert = SDL_SCANCODE_INSERT.0,
    Home = SDL_SCANCODE_HOME.0,
    PageUp = SDL_SCANCODE_PAGEUP.0,
    Delete = SDL_SCANCODE_DELETE.0,
    End = SDL_SCANCODE_END.0,
    PageDown = SDL_SCANCODE_PAGEDOWN.0,
    Right = SDL_SCANCODE_RIGHT.0,
    Left = SDL_SCANCODE_LEFT.0,
    Down = SDL_SCANCODE_DOWN.0,
    Up = SDL_SCANCODE_UP.0,
    NumLockClear = SDL_SCANCODE_NUMLOCKCLEAR.0,
    KpDivide = SDL_SCANCODE_KP_DIVIDE.0,
    KpMultiply = SDL_SCANCODE_KP_MULTIPLY.0,
    KpMinus = SDL_SCANCODE_KP_MINUS.0,
    KpPlus = SDL_SCANCODE_KP_PLUS.0,
    KpEnter = SDL_SCANCODE_KP_ENTER.0,
    Kp1 = SDL_SCANCODE_KP_1.0,
    Kp2 = SDL_SCANCODE_KP_2.0,
    Kp3 = SDL_SCANCODE_KP_3.0,
    Kp4 = SDL_SCANCODE_KP_4.0,
    Kp5 = SDL_SCANCODE_KP_5.0,
    Kp6 = SDL_SCANCODE_KP_6.0,
    Kp7 = SDL_SCANCODE_KP_7.0,
    Kp8 = SDL_SCANCODE_KP_8.0,
    Kp9 = SDL_SCANCODE_KP_9.0,
    Kp0 = SDL_SCANCODE_KP_0.0,
    KpPeriod = SDL_SCANCODE_KP_PERIOD.0,
    NonUsBackslash = SDL_SCANCODE_NONUSBACKSLASH.0,
    Application = SDL_SCANCODE_APPLICATION.0,
    Power = SDL_SCANCODE_POWER.0,
    KpEquals = SDL_SCANCODE_KP_EQUALS.0,
    F13 = SDL_SCANCODE_F13.0,
    F14 = SDL_SCANCODE_F14.0,
    F15 = SDL_SCANCODE_F15.0,
    F16 = SDL_SCANCODE_F16.0,
    F17 = SDL_SCANCODE_F17.0,
    F18 = SDL_SCANCODE_F18.0,
    F19 = SDL_SCANCODE_F19.0,
    F20 = SDL_SCANCODE_F20.0,
    F21 = SDL_SCANCODE_F21.0,
    F22 = SDL_SCANCODE_F22.0,
    F23 = SDL_SCANCODE_F23.0,
    F24 = SDL_SCANCODE_F24.0,
    Execute = SDL_SCANCODE_EXECUTE.0,
    Help = SDL_SCANCODE_HELP.0,
    Menu = SDL_SCANCODE_MENU.0,
    Select = SDL_SCANCODE_SELECT.0,
    Stop = SDL_SCANCODE_STOP.0,
    Again = SDL_SCANCODE_AGAIN.0,
    Undo = SDL_SCANCODE_UNDO.0,
    Cut = SDL_SCANCODE_CUT.0,
    Copy = SDL_SCANCODE_COPY.0,
    Paste = SDL_SCANCODE_PASTE.0,
    Find = SDL_SCANCODE_FIND.0,
    Mute = SDL_SCANCODE_MUTE.0,
    VolumeUp = SDL_SCANCODE_VOLUMEUP.0,
    VolumeDown = SDL_SCANCODE_VOLUMEDOWN.0,
    KpComma = SDL_SCANCODE_KP_COMMA.0,
    KpEqualsAs400 = SDL_SCANCODE_KP_EQUALSAS400.0,
    International1 = SDL_SCANCODE_INTERNATIONAL1.0,
    International2 = SDL_SCANCODE_INTERNATIONAL2.0,
    International3 = SDL_SCANCODE_INTERNATIONAL3.0,
    International4 = SDL_SCANCODE_INTERNATIONAL4.0,
    International5 = SDL_SCANCODE_INTERNATIONAL5.0,
    International6 = SDL_SCANCODE_INTERNATIONAL6.0,
    International7 = SDL_SCANCODE_INTERNATIONAL7.0,
    International8 = SDL_SCANCODE_INTERNATIONAL8.0,
    International9 = SDL_SCANCODE_INTERNATIONAL9.0,
    Lang1 = SDL_SCANCODE_LANG1.0,
    Lang2 = SDL_SCANCODE_LANG2.0,
    Lang3 = SDL_SCANCODE_LANG3.0,
    Lang4 = SDL_SCANCODE_LANG4.0,
    Lang5 = SDL_SCANCODE_LANG5.0,
    Lang6 = SDL_SCANCODE_LANG6.0,
    Lang7 = SDL_SCANCODE_LANG7.0,
    Lang8 = SDL_SCANCODE_LANG8.0,
    Lang9 = SDL_SCANCODE_LANG9.0,
    AltErase = SDL_SCANCODE_ALTERASE.0,
    SysReq = SDL_SCANCODE_SYSREQ.0,
    Cancel = SDL_SCANCODE_CANCEL.0,
    Clear = SDL_SCANCODE_CLEAR.0,
    Prior = SDL_SCANCODE_PRIOR.0,
    Return2 = SDL_SCANCODE_RETURN2.0,
    Separator = SDL_SCANCODE_SEPARATOR.0,
    Out = SDL_SCANCODE_OUT.0,
    Oper = SDL_SCANCODE_OPER.0,
    ClearAgain = SDL_SCANCODE_CLEARAGAIN.0,
    CrSel = SDL_SCANCODE_CRSEL.0,
    ExSel = SDL_SCANCODE_EXSEL.0,
    Kp00 = SDL_SCANCODE_KP_00.0,
    Kp000 = SDL_SCANCODE_KP_000.0,
    ThousandsSeparator = SDL_SCANCODE_THOUSANDSSEPARATOR.0,
    DecimalSeparator = SDL_SCANCODE_DECIMALSEPARATOR.0,
    CurrencyUnit = SDL_SCANCODE_CURRENCYUNIT.0,
    CurrencySubunit = SDL_SCANCODE_CURRENCYSUBUNIT.0,
    KpLeftParen = SDL_SCANCODE_KP_LEFTPAREN.0,
    KpRightParen = SDL_SCANCODE_KP_RIGHTPAREN.0,
    KpLeftBrace = SDL_SCANCODE_KP_LEFTBRACE.0,
    KpRightBrace = SDL_SCANCODE_KP_RIGHTBRACE.0,
    KpTab = SDL_SCANCODE_KP_TAB.0,
    KpBackspace = SDL_SCANCODE_KP_BACKSPACE.0,
    KpA = SDL_SCANCODE_KP_A.0,
    KpB = SDL_SCANCODE_KP_B.0,
    KpC = SDL_SCANCODE_KP_C.0,
    KpD = SDL_SCANCODE_KP_D.0,
    KpE = SDL_SCANCODE_KP_E.0,
    KpF = SDL_SCANCODE_KP_F.0,
    KpXor = SDL_SCANCODE_KP_XOR.0,
    KpPower = SDL_SCANCODE_KP_POWER.0,
    KpPercent = SDL_SCANCODE_KP_PERCENT.0,
    KpLess = SDL_SCANCODE_KP_LESS.0,
    KpGreater = SDL_SCANCODE_KP_GREATER.0,
    KpAmpersand = SDL_SCANCODE_KP_AMPERSAND.0,
    KpDblAmpersand = SDL_SCANCODE_KP_DBLAMPERSAND.0,
    KpVerticalBar = SDL_SCANCODE_KP_VERTICALBAR.0,
    KpDblVerticalBar = SDL_SCANCODE_KP_DBLVERTICALBAR.0,
    KpColon = SDL_SCANCODE_KP_COLON.0,
    KpHash = SDL_SCANCODE_KP_HASH.0,
    KpSpace = SDL_SCANCODE_KP_SPACE.0,
    KpAt = SDL_SCANCODE_KP_AT.0,
    KpExclam = SDL_SCANCODE_KP_EXCLAM.0,
    KpMemStore = SDL_SCANCODE_KP_MEMSTORE.0,
    KpMemRecall = SDL_SCANCODE_KP_MEMRECALL.0,
    KpMemClear = SDL_SCANCODE_KP_MEMCLEAR.0,
    KpMemAdd = SDL_SCANCODE_KP_MEMADD.0,
    KpMemSubtract = SDL_SCANCODE_KP_MEMSUBTRACT.0,
    KpMemMultiply = SDL_SCANCODE_KP_MEMMULTIPLY.0,
    KpMemDivide = SDL_SCANCODE_KP_MEMDIVIDE.0,
    KpPlusMinus = SDL_SCANCODE_KP_PLUSMINUS.0,
    KpClear = SDL_SCANCODE_KP_CLEAR.0,
    KpClearEntry = SDL_SCANCODE_KP_CLEARENTRY.0,
    KpBinary = SDL_SCANCODE_KP_BINARY.0,
    KpOctal = SDL_SCANCODE_KP_OCTAL.0,
    KpDecimal = SDL_SCANCODE_KP_DECIMAL.0,
    KpHexadecimal = SDL_SCANCODE_KP_HEXADECIMAL.0,
    LCtrl = SDL_SCANCODE_LCTRL.0,
    LShift = SDL_SCANCODE_LSHIFT.0,
    LAlt = SDL_SCANCODE_LALT.0,
    LGui = SDL_SCANCODE_LGUI.0,
    RCtrl = SDL_SCANCODE_RCTRL.0,
    RShift = SDL_SCANCODE_RSHIFT.0,
    RAlt = SDL_SCANCODE_RALT.0,
    RGui = SDL_SCANCODE_RGUI.0,
    Mode = SDL_SCANCODE_MODE.0,
    Sleep = SDL_SCANCODE_SLEEP.0,
    Wake = SDL_SCANCODE_WAKE.0,
    ChannelIncrement = SDL_SCANCODE_CHANNEL_INCREMENT.0,
    ChannelDecrement = SDL_SCANCODE_CHANNEL_DECREMENT.0,
    MediaPlay = SDL_SCANCODE_MEDIA_PLAY.0,
    MediaPause = SDL_SCANCODE_MEDIA_PAUSE.0,
    MediaRecord = SDL_SCANCODE_MEDIA_RECORD.0,
    MediaFastForward = SDL_SCANCODE_MEDIA_FAST_FORWARD.0,
    MediaRewind = SDL_SCANCODE_MEDIA_REWIND.0,
    MediaNextTrack = SDL_SCANCODE_MEDIA_NEXT_TRACK.0,
    MediaPreviousTrack = SDL_SCANCODE_MEDIA_PREVIOUS_TRACK.0,
    MediaStop = SDL_SCANCODE_MEDIA_STOP.0,
    MediaEject = SDL_SCANCODE_MEDIA_EJECT.0,
    MediaPlayPause = SDL_SCANCODE_MEDIA_PLAY_PAUSE.0,
    MediaSelect = SDL_SCANCODE_MEDIA_SELECT.0,
    AcNew = SDL_SCANCODE_AC_NEW.0,
    AcOpen = SDL_SCANCODE_AC_OPEN.0,
    AcClose = SDL_SCANCODE_AC_CLOSE.0,
    AcExit = SDL_SCANCODE_AC_EXIT.0,
    AcSave = SDL_SCANCODE_AC_SAVE.0,
    AcPrint = SDL_SCANCODE_AC_PRINT.0,
    AcProperties = SDL_SCANCODE_AC_PROPERTIES.0,
    AcSearch = SDL_SCANCODE_AC_SEARCH.0,
    AcHome = SDL_SCANCODE_AC_HOME.0,
    AcBack = SDL_SCANCODE_AC_BACK.0,
    AcForward = SDL_SCANCODE_AC_FORWARD.0,
    AcStop = SDL_SCANCODE_AC_STOP.0,
    AcRefresh = SDL_SCANCODE_AC_REFRESH.0,
    AcBookmarks = SDL_SCANCODE_AC_BOOKMARKS.0,
    SoftLeft = SDL_SCANCODE_SOFTLEFT.0,
    SoftRight = SDL_SCANCODE_SOFTRIGHT.0,
    Call = SDL_SCANCODE_CALL.0,
    EndCall = SDL_SCANCODE_ENDCALL.0,
    Reserved = SDL_SCANCODE_RESERVED.0,
    Count = SDL_SCANCODE_COUNT.0,
}

impl Scancode {
    pub fn from_i32(n: i32) -> Option<Scancode> {
        use self::Scancode::*;
        let n = n as u32;

        Some(match unsafe { transmute::<u32, SDL_Scancode>(n) } {
            SDL_SCANCODE_UNKNOWN => Unknown,
            SDL_SCANCODE_A => A,
            SDL_SCANCODE_B => B,
            SDL_SCANCODE_C => C,
            SDL_SCANCODE_D => D,
            SDL_SCANCODE_E => E,
            SDL_SCANCODE_F => F,
            SDL_SCANCODE_G => G,
            SDL_SCANCODE_H => H,
            SDL_SCANCODE_I => I,
            SDL_SCANCODE_J => J,
            SDL_SCANCODE_K => K,
            SDL_SCANCODE_L => L,
            SDL_SCANCODE_M => M,
            SDL_SCANCODE_N => N,
            SDL_SCANCODE_O => O,
            SDL_SCANCODE_P => P,
            SDL_SCANCODE_Q => Q,
            SDL_SCANCODE_R => R,
            SDL_SCANCODE_S => S,
            SDL_SCANCODE_T => T,
            SDL_SCANCODE_U => U,
            SDL_SCANCODE_V => V,
            SDL_SCANCODE_W => W,
            SDL_SCANCODE_X => X,
            SDL_SCANCODE_Y => Y,
            SDL_SCANCODE_Z => Z,
            SDL_SCANCODE_1 => _1,
            SDL_SCANCODE_2 => _2,
            SDL_SCANCODE_3 => _3,
            SDL_SCANCODE_4 => _4,
            SDL_SCANCODE_5 => _5,
            SDL_SCANCODE_6 => _6,
            SDL_SCANCODE_7 => _7,
            SDL_SCANCODE_8 => _8,
            SDL_SCANCODE_9 => _9,
            SDL_SCANCODE_0 => _0,
            SDL_SCANCODE_RETURN => Return,
            SDL_SCANCODE_ESCAPE => Escape,
            SDL_SCANCODE_BACKSPACE => Backspace,
            SDL_SCANCODE_TAB => Tab,
            SDL_SCANCODE_SPACE => Space,
            SDL_SCANCODE_MINUS => Minus,
            SDL_SCANCODE_EQUALS => Equals,
            SDL_SCANCODE_LEFTBRACKET => LeftBracket,
            SDL_SCANCODE_RIGHTBRACKET => RightBracket,
            SDL_SCANCODE_BACKSLASH => Backslash,
            SDL_SCANCODE_NONUSHASH => NonUsHash,
            SDL_SCANCODE_SEMICOLON => Semicolon,
            SDL_SCANCODE_APOSTROPHE => Apostrophe,
            SDL_SCANCODE_GRAVE => Grave,
            SDL_SCANCODE_COMMA => Comma,
            SDL_SCANCODE_PERIOD => Period,
            SDL_SCANCODE_SLASH => Slash,
            SDL_SCANCODE_CAPSLOCK => CapsLock,
            SDL_SCANCODE_F1 => F1,
            SDL_SCANCODE_F2 => F2,
            SDL_SCANCODE_F3 => F3,
            SDL_SCANCODE_F4 => F4,
            SDL_SCANCODE_F5 => F5,
            SDL_SCANCODE_F6 => F6,
            SDL_SCANCODE_F7 => F7,
            SDL_SCANCODE_F8 => F8,
            SDL_SCANCODE_F9 => F9,
            SDL_SCANCODE_F10 => F10,
            SDL_SCANCODE_F11 => F11,
            SDL_SCANCODE_F12 => F12,
            SDL_SCANCODE_PRINTSCREEN => PrintScreen,
            SDL_SCANCODE_SCROLLLOCK => ScrollLock,
            SDL_SCANCODE_PAUSE => Pause,
            SDL_SCANCODE_INSERT => Insert,
            SDL_SCANCODE_HOME => Home,
            SDL_SCANCODE_PAGEUP => PageUp,
            SDL_SCANCODE_DELETE => Delete,
            SDL_SCANCODE_END => End,
            SDL_SCANCODE_PAGEDOWN => PageDown,
            SDL_SCANCODE_RIGHT => Right,
            SDL_SCANCODE_LEFT => Left,
            SDL_SCANCODE_DOWN => Down,
            SDL_SCANCODE_UP => Up,
            SDL_SCANCODE_NUMLOCKCLEAR => NumLockClear,
            SDL_SCANCODE_KP_DIVIDE => KpDivide,
            SDL_SCANCODE_KP_MULTIPLY => KpMultiply,
            SDL_SCANCODE_KP_MINUS => KpMinus,
            SDL_SCANCODE_KP_PLUS => KpPlus,
            SDL_SCANCODE_KP_ENTER => KpEnter,
            SDL_SCANCODE_KP_1 => Kp1,
            SDL_SCANCODE_KP_2 => Kp2,
            SDL_SCANCODE_KP_3 => Kp3,
            SDL_SCANCODE_KP_4 => Kp4,
            SDL_SCANCODE_KP_5 => Kp5,
            SDL_SCANCODE_KP_6 => Kp6,
            SDL_SCANCODE_KP_7 => Kp7,
            SDL_SCANCODE_KP_8 => Kp8,
            SDL_SCANCODE_KP_9 => Kp9,
            SDL_SCANCODE_KP_0 => Kp0,
            SDL_SCANCODE_KP_PERIOD => KpPeriod,
            SDL_SCANCODE_NONUSBACKSLASH => NonUsBackslash,
            SDL_SCANCODE_APPLICATION => Application,
            SDL_SCANCODE_POWER => Power,
            SDL_SCANCODE_KP_EQUALS => KpEquals,
            SDL_SCANCODE_F13 => F13,
            SDL_SCANCODE_F14 => F14,
            SDL_SCANCODE_F15 => F15,
            SDL_SCANCODE_F16 => F16,
            SDL_SCANCODE_F17 => F17,
            SDL_SCANCODE_F18 => F18,
            SDL_SCANCODE_F19 => F19,
            SDL_SCANCODE_F20 => F20,
            SDL_SCANCODE_F21 => F21,
            SDL_SCANCODE_F22 => F22,
            SDL_SCANCODE_F23 => F23,
            SDL_SCANCODE_F24 => F24,
            SDL_SCANCODE_EXECUTE => Execute,
            SDL_SCANCODE_HELP => Help,
            SDL_SCANCODE_MENU => Menu,
            SDL_SCANCODE_SELECT => Select,
            SDL_SCANCODE_STOP => Stop,
            SDL_SCANCODE_AGAIN => Again,
            SDL_SCANCODE_UNDO => Undo,
            SDL_SCANCODE_CUT => Cut,
            SDL_SCANCODE_COPY => Copy,
            SDL_SCANCODE_PASTE => Paste,
            SDL_SCANCODE_FIND => Find,
            SDL_SCANCODE_MUTE => Mute,
            SDL_SCANCODE_VOLUMEUP => VolumeUp,
            SDL_SCANCODE_VOLUMEDOWN => VolumeDown,
            SDL_SCANCODE_KP_COMMA => KpComma,
            SDL_SCANCODE_KP_EQUALSAS400 => KpEqualsAs400,
            SDL_SCANCODE_INTERNATIONAL1 => International1,
            SDL_SCANCODE_INTERNATIONAL2 => International2,
            SDL_SCANCODE_INTERNATIONAL3 => International3,
            SDL_SCANCODE_INTERNATIONAL4 => International4,
            SDL_SCANCODE_INTERNATIONAL5 => International5,
            SDL_SCANCODE_INTERNATIONAL6 => International6,
            SDL_SCANCODE_INTERNATIONAL7 => International7,
            SDL_SCANCODE_INTERNATIONAL8 => International8,
            SDL_SCANCODE_INTERNATIONAL9 => International9,
            SDL_SCANCODE_LANG1 => Lang1,
            SDL_SCANCODE_LANG2 => Lang2,
            SDL_SCANCODE_LANG3 => Lang3,
            SDL_SCANCODE_LANG4 => Lang4,
            SDL_SCANCODE_LANG5 => Lang5,
            SDL_SCANCODE_LANG6 => Lang6,
            SDL_SCANCODE_LANG7 => Lang7,
            SDL_SCANCODE_LANG8 => Lang8,
            SDL_SCANCODE_LANG9 => Lang9,
            SDL_SCANCODE_ALTERASE => AltErase,
            SDL_SCANCODE_SYSREQ => SysReq,
            SDL_SCANCODE_CANCEL => Cancel,
            SDL_SCANCODE_CLEAR => Clear,
            SDL_SCANCODE_PRIOR => Prior,
            SDL_SCANCODE_RETURN2 => Return2,
            SDL_SCANCODE_SEPARATOR => Separator,
            SDL_SCANCODE_OUT => Out,
            SDL_SCANCODE_OPER => Oper,
            SDL_SCANCODE_CLEARAGAIN => ClearAgain,
            SDL_SCANCODE_CRSEL => CrSel,
            SDL_SCANCODE_EXSEL => ExSel,
            SDL_SCANCODE_KP_00 => Kp00,
            SDL_SCANCODE_KP_000 => Kp000,
            SDL_SCANCODE_THOUSANDSSEPARATOR => ThousandsSeparator,
            SDL_SCANCODE_DECIMALSEPARATOR => DecimalSeparator,
            SDL_SCANCODE_CURRENCYUNIT => CurrencyUnit,
            SDL_SCANCODE_CURRENCYSUBUNIT => CurrencySubunit,
            SDL_SCANCODE_KP_LEFTPAREN => KpLeftParen,
            SDL_SCANCODE_KP_RIGHTPAREN => KpRightParen,
            SDL_SCANCODE_KP_LEFTBRACE => KpLeftBrace,
            SDL_SCANCODE_KP_RIGHTBRACE => KpRightBrace,
            SDL_SCANCODE_KP_TAB => KpTab,
            SDL_SCANCODE_KP_BACKSPACE => KpBackspace,
            SDL_SCANCODE_KP_A => KpA,
            SDL_SCANCODE_KP_B => KpB,
            SDL_SCANCODE_KP_C => KpC,
            SDL_SCANCODE_KP_D => KpD,
            SDL_SCANCODE_KP_E => KpE,
            SDL_SCANCODE_KP_F => KpF,
            SDL_SCANCODE_KP_XOR => KpXor,
            SDL_SCANCODE_KP_POWER => KpPower,
            SDL_SCANCODE_KP_PERCENT => KpPercent,
            SDL_SCANCODE_KP_LESS => KpLess,
            SDL_SCANCODE_KP_GREATER => KpGreater,
            SDL_SCANCODE_KP_AMPERSAND => KpAmpersand,
            SDL_SCANCODE_KP_DBLAMPERSAND => KpDblAmpersand,
            SDL_SCANCODE_KP_VERTICALBAR => KpVerticalBar,
            SDL_SCANCODE_KP_DBLVERTICALBAR => KpDblVerticalBar,
            SDL_SCANCODE_KP_COLON => KpColon,
            SDL_SCANCODE_KP_HASH => KpHash,
            SDL_SCANCODE_KP_SPACE => KpSpace,
            SDL_SCANCODE_KP_AT => KpAt,
            SDL_SCANCODE_KP_EXCLAM => KpExclam,
            SDL_SCANCODE_KP_MEMSTORE => KpMemStore,
            SDL_SCANCODE_KP_MEMRECALL => KpMemRecall,
            SDL_SCANCODE_KP_MEMCLEAR => KpMemClear,
            SDL_SCANCODE_KP_MEMADD => KpMemAdd,
            SDL_SCANCODE_KP_MEMSUBTRACT => KpMemSubtract,
            SDL_SCANCODE_KP_MEMMULTIPLY => KpMemMultiply,
            SDL_SCANCODE_KP_MEMDIVIDE => KpMemDivide,
            SDL_SCANCODE_KP_PLUSMINUS => KpPlusMinus,
            SDL_SCANCODE_KP_CLEAR => KpClear,
            SDL_SCANCODE_KP_CLEARENTRY => KpClearEntry,
            SDL_SCANCODE_KP_BINARY => KpBinary,
            SDL_SCANCODE_KP_OCTAL => KpOctal,
            SDL_SCANCODE_KP_DECIMAL => KpDecimal,
            SDL_SCANCODE_KP_HEXADECIMAL => KpHexadecimal,
            SDL_SCANCODE_LCTRL => LCtrl,
            SDL_SCANCODE_LSHIFT => LShift,
            SDL_SCANCODE_LALT => LAlt,
            SDL_SCANCODE_LGUI => LGui,
            SDL_SCANCODE_RCTRL => RCtrl,
            SDL_SCANCODE_RSHIFT => RShift,
            SDL_SCANCODE_RALT => RAlt,
            SDL_SCANCODE_RGUI => RGui,
            SDL_SCANCODE_MODE => Mode,
            SDL_SCANCODE_SLEEP => Sleep,
            SDL_SCANCODE_WAKE => Wake,
            SDL_SCANCODE_CHANNEL_INCREMENT => ChannelIncrement,
            SDL_SCANCODE_CHANNEL_DECREMENT => ChannelDecrement,
            SDL_SCANCODE_MEDIA_PLAY => MediaPlay,
            SDL_SCANCODE_MEDIA_PAUSE => MediaPause,
            SDL_SCANCODE_MEDIA_RECORD => MediaRecord,
            SDL_SCANCODE_MEDIA_FAST_FORWARD => MediaFastForward,
            SDL_SCANCODE_MEDIA_REWIND => MediaRewind,
            SDL_SCANCODE_MEDIA_NEXT_TRACK => MediaNextTrack,
            SDL_SCANCODE_MEDIA_PREVIOUS_TRACK => MediaPreviousTrack,
            SDL_SCANCODE_MEDIA_STOP => MediaStop,
            SDL_SCANCODE_MEDIA_EJECT => MediaEject,
            SDL_SCANCODE_MEDIA_PLAY_PAUSE => MediaPlayPause,
            SDL_SCANCODE_MEDIA_SELECT => MediaSelect,
            SDL_SCANCODE_AC_NEW => AcNew,
            SDL_SCANCODE_AC_OPEN => AcOpen,
            SDL_SCANCODE_AC_CLOSE => AcClose,
            SDL_SCANCODE_AC_EXIT => AcExit,
            SDL_SCANCODE_AC_SAVE => AcSave,
            SDL_SCANCODE_AC_PRINT => AcPrint,
            SDL_SCANCODE_AC_PROPERTIES => AcProperties,
            SDL_SCANCODE_AC_SEARCH => AcSearch,
            SDL_SCANCODE_AC_HOME => AcHome,
            SDL_SCANCODE_AC_BACK => AcBack,
            SDL_SCANCODE_AC_FORWARD => AcForward,
            SDL_SCANCODE_AC_STOP => AcStop,
            SDL_SCANCODE_AC_REFRESH => AcRefresh,
            SDL_SCANCODE_AC_BOOKMARKS => AcBookmarks,
            SDL_SCANCODE_SOFTLEFT => SoftLeft,
            SDL_SCANCODE_SOFTRIGHT => SoftRight,
            SDL_SCANCODE_CALL => Call,
            SDL_SCANCODE_ENDCALL => EndCall,
            SDL_SCANCODE_RESERVED => Reserved,
            SDL_SCANCODE_COUNT => Count,
            _ => return None,
        })
    }
}

use std::fmt;
use sys::keycode::SDL_Keymod;

impl fmt::Display for Scancode {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.name())
    }
}

use crate::keyboard::Keycode;

impl Scancode {
    /// Gets the scancode from a virtual key. Returns None if there is no corresponding scancode.
    #[doc(alias = "SDL_GetScancodeFromKey")]
    pub fn from_keycode(keycode: Keycode, modstate: *mut SDL_Keymod) -> Option<Scancode> {
        unsafe {
            match sys::keyboard::SDL_GetScancodeFromKey(keycode, modstate) {
                SDL_SCANCODE_UNKNOWN => None,
                scancode_id => Scancode::from_i32(scancode_id.0),
            }
        }
    }

    #[doc(alias = "SDL_GetScancodeFromName")]
    pub fn from_name(name: &str) -> Option<Scancode> {
        unsafe {
            match CString::new(name) {
                Ok(name) => match sys::SDL_GetScancodeFromName(name.as_ptr() as *const c_char) {
                    SDL_SCANCODE_UNKNOWN => None,
                    scancode_id => Some(Scancode::from_i32(scancode_id.0).unwrap()),
                },
                // string contains a nul byte - it won't match anything.
                Err(_) => None,
            }
        }
    }

    #[doc(alias = "SDL_GetScancodeName")]
    pub fn name(self) -> &'static str {
        // The name string pointer lives in static, read-only memory.
        // Knowing this, we can always return a string slice.
        unsafe {
            let buf = sys::SDL_GetScancodeName(transmute::<u32, SDL_Scancode>(self as u32));
            ::std::str::from_utf8(CStr::from_ptr(buf as *const _).to_bytes()).unwrap()
        }
    }
}
