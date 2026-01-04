use ratatui::style::Color;

#[derive(Clone, Debug)]
pub struct ThemeColors {
    pub background: Color,
    pub foreground: Color,
    pub border: Color,
    pub title: Color,
    pub selected: Color,
    pub active: Color,
    pub inactive: Color,
    pub warning: Color,
    pub error: Color,
    pub success: Color,
    pub info: Color,
    pub menu_bg: Color,
    pub menu_fg: Color,
    pub menu_selected: Color,
    pub note_color: Color,
    pub note_selected: Color,
    pub playhead: Color,
    pub current_tick: Color,
    pub grid: Color,
    pub piano_key_white: Color,
    pub piano_key_black: Color,
}

impl ThemeColors {
    pub fn retro() -> Self {
        Self {
            background: Color::Black,
            foreground: Color::Green,
            border: Color::Green,
            title: Color::Rgb(0, 255, 0), // Bright green
            selected: Color::Yellow,
            active: Color::Rgb(0, 255, 0), // Bright green
            inactive: Color::DarkGray,
            warning: Color::Yellow,
            error: Color::Red,
            success: Color::Green,
            info: Color::Cyan,
            menu_bg: Color::Black,
            menu_fg: Color::Green,
            menu_selected: Color::Rgb(0, 255, 0), // Bright green
            note_color: Color::Rgb(0, 255, 0), // Bright green
            note_selected: Color::Yellow,
            playhead: Color::Red,
            current_tick: Color::Rgb(0, 255, 0), // Bright green
            grid: Color::DarkGray,
            piano_key_white: Color::White,
            piano_key_black: Color::DarkGray,
        }
    }

    pub fn amber() -> Self {
        Self {
            background: Color::Black,
            foreground: Color::Rgb(255, 176, 0), // Amber
            border: Color::Rgb(255, 200, 0),
            title: Color::Rgb(255, 220, 0),
            selected: Color::Yellow,
            active: Color::Rgb(255, 200, 0),
            inactive: Color::DarkGray,
            warning: Color::Yellow,
            error: Color::Red,
            success: Color::Rgb(255, 200, 0),
            info: Color::Rgb(255, 220, 0),
            menu_bg: Color::Black,
            menu_fg: Color::Rgb(255, 176, 0),
            menu_selected: Color::Rgb(255, 220, 0),
            note_color: Color::Rgb(255, 200, 0),
            note_selected: Color::Yellow,
            playhead: Color::Red,
            current_tick: Color::Rgb(255, 220, 0),
            grid: Color::DarkGray,
            piano_key_white: Color::Rgb(255, 200, 0),
            piano_key_black: Color::DarkGray,
        }
    }

    pub fn matrix() -> Self {
        Self {
            background: Color::Black,
            foreground: Color::Green,
            border: Color::Rgb(0, 255, 0), // Bright green
            title: Color::Rgb(0, 255, 0), // Bright green
            selected: Color::Cyan,
            active: Color::Rgb(0, 255, 0), // Bright green
            inactive: Color::DarkGray,
            warning: Color::Yellow,
            error: Color::Red,
            success: Color::Green,
            info: Color::Cyan,
            menu_bg: Color::Black,
            menu_fg: Color::Green,
            menu_selected: Color::Rgb(0, 255, 0), // Bright green
            note_color: Color::Rgb(0, 255, 0), // Bright green
            note_selected: Color::Cyan,
            playhead: Color::Red,
            current_tick: Color::Rgb(0, 255, 0), // Bright green
            grid: Color::DarkGray,
            piano_key_white: Color::Rgb(0, 255, 0), // Bright green
            piano_key_black: Color::DarkGray,
        }
    }

    pub fn c64() -> Self {
        Self {
            background: Color::Rgb(64, 49, 141), // C64 Blue
            foreground: Color::Rgb(136, 113, 239), // C64 Light Blue
            border: Color::Rgb(136, 113, 239),
            title: Color::Rgb(162, 162, 162), // C64 Light Gray
            selected: Color::Rgb(255, 255, 255), // White
            active: Color::Rgb(136, 113, 239),
            inactive: Color::Rgb(64, 49, 141),
            warning: Color::Rgb(255, 162, 0), // Orange
            error: Color::Rgb(136, 0, 0), // Dark Red
            success: Color::Rgb(136, 113, 239),
            info: Color::Rgb(162, 162, 162),
            menu_bg: Color::Rgb(64, 49, 141),
            menu_fg: Color::Rgb(136, 113, 239),
            menu_selected: Color::Rgb(255, 255, 255),
            note_color: Color::Rgb(136, 113, 239),
            note_selected: Color::Rgb(255, 255, 255),
            playhead: Color::Rgb(255, 162, 0),
            current_tick: Color::Rgb(136, 113, 239),
            grid: Color::Rgb(64, 49, 141),
            piano_key_white: Color::Rgb(162, 162, 162),
            piano_key_black: Color::Rgb(64, 49, 141),
        }
    }

    pub fn apple2() -> Self {
        Self {
            background: Color::Black,
            foreground: Color::Rgb(255, 255, 0), // Apple II Green (yellow-green)
            border: Color::Rgb(255, 255, 128),
            title: Color::Rgb(255, 255, 128),
            selected: Color::Rgb(255, 255, 255), // White
            active: Color::Rgb(255, 255, 0),
            inactive: Color::DarkGray,
            warning: Color::Rgb(255, 200, 0),
            error: Color::Red,
            success: Color::Rgb(255, 255, 0),
            info: Color::Rgb(255, 255, 128),
            menu_bg: Color::Black,
            menu_fg: Color::Rgb(255, 255, 0),
            menu_selected: Color::Rgb(255, 255, 128),
            note_color: Color::Rgb(255, 255, 0),
            note_selected: Color::Rgb(255, 255, 255),
            playhead: Color::Red,
            current_tick: Color::Rgb(255, 255, 128),
            grid: Color::DarkGray,
            piano_key_white: Color::Rgb(255, 255, 0),
            piano_key_black: Color::DarkGray,
        }
    }

    pub fn monochrome() -> Self {
        Self {
            background: Color::Black,
            foreground: Color::White,
            border: Color::White,
            title: Color::White,
            selected: Color::White,
            active: Color::White,
            inactive: Color::DarkGray,
            warning: Color::White,
            error: Color::White,
            success: Color::White,
            info: Color::White,
            menu_bg: Color::Black,
            menu_fg: Color::White,
            menu_selected: Color::White,
            note_color: Color::White,
            note_selected: Color::White,
            playhead: Color::White,
            current_tick: Color::White,
            grid: Color::DarkGray,
            piano_key_white: Color::White,
            piano_key_black: Color::DarkGray,
        }
    }

    pub fn neon() -> Self {
        Self {
            background: Color::Black,
            foreground: Color::Rgb(0, 255, 255), // Cyan
            border: Color::Rgb(255, 0, 255), // Magenta
            title: Color::Rgb(255, 0, 255),
            selected: Color::Rgb(255, 255, 0), // Yellow
            active: Color::Rgb(0, 255, 255),
            inactive: Color::DarkGray,
            warning: Color::Rgb(255, 128, 0), // Orange
            error: Color::Rgb(255, 0, 128), // Pink
            success: Color::Rgb(0, 255, 128), // Green
            info: Color::Rgb(128, 0, 255), // Purple
            menu_bg: Color::Black,
            menu_fg: Color::Rgb(0, 255, 255),
            menu_selected: Color::Rgb(255, 0, 255),
            note_color: Color::Rgb(0, 255, 255),
            note_selected: Color::Rgb(255, 255, 0),
            playhead: Color::Rgb(255, 0, 128),
            current_tick: Color::Rgb(0, 255, 128),
            grid: Color::DarkGray,
            piano_key_white: Color::Rgb(0, 255, 255),
            piano_key_black: Color::DarkGray,
        }
    }

    pub fn from_theme(theme: &crate::settings::Theme) -> Self {
        match theme {
            crate::settings::Theme::Retro => Self::retro(),
            crate::settings::Theme::Amber => Self::amber(),
            crate::settings::Theme::Matrix => Self::matrix(),
            crate::settings::Theme::C64 => Self::c64(),
            crate::settings::Theme::Apple2 => Self::apple2(),
            crate::settings::Theme::Monochrome => Self::monochrome(),
            crate::settings::Theme::Neon => Self::neon(),
            crate::settings::Theme::Default => Self::default_theme(),
            crate::settings::Theme::Dark => Self::dark_theme(),
            crate::settings::Theme::Light => Self::light_theme(),
            crate::settings::Theme::HighContrast => Self::high_contrast_theme(),
        }
    }

    pub fn default_theme() -> Self {
        Self {
            background: Color::Black,
            foreground: Color::White,
            border: Color::LightBlue,
            title: Color::Cyan,
            selected: Color::Yellow,
            active: Color::Green,
            inactive: Color::DarkGray,
            warning: Color::Yellow,
            error: Color::Red,
            success: Color::Green,
            info: Color::Cyan,
            menu_bg: Color::DarkGray,
            menu_fg: Color::White,
            menu_selected: Color::Yellow,
            note_color: Color::Cyan,
            note_selected: Color::Yellow,
            playhead: Color::Red,
            current_tick: Color::Green,
            grid: Color::DarkGray,
            piano_key_white: Color::White,
            piano_key_black: Color::DarkGray,
        }
    }

    pub fn dark_theme() -> Self {
        Self {
            background: Color::Rgb(20, 20, 20),
            foreground: Color::Rgb(200, 200, 200),
            border: Color::Rgb(100, 100, 100),
            title: Color::Rgb(150, 150, 255),
            selected: Color::Rgb(255, 200, 100),
            active: Color::Rgb(100, 255, 100),
            inactive: Color::Rgb(60, 60, 60),
            warning: Color::Rgb(255, 200, 100),
            error: Color::Rgb(255, 100, 100),
            success: Color::Rgb(100, 255, 100),
            info: Color::Rgb(100, 200, 255),
            menu_bg: Color::Rgb(30, 30, 30),
            menu_fg: Color::Rgb(200, 200, 200),
            menu_selected: Color::Rgb(255, 200, 100),
            note_color: Color::Rgb(100, 200, 255),
            note_selected: Color::Rgb(255, 200, 100),
            playhead: Color::Rgb(255, 100, 100),
            current_tick: Color::Rgb(100, 255, 100),
            grid: Color::Rgb(40, 40, 40),
            piano_key_white: Color::Rgb(200, 200, 200),
            piano_key_black: Color::Rgb(60, 60, 60),
        }
    }

    pub fn light_theme() -> Self {
        Self {
            background: Color::Rgb(240, 240, 240),
            foreground: Color::Rgb(20, 20, 20),
            border: Color::Rgb(100, 100, 100),
            title: Color::Rgb(0, 0, 200),
            selected: Color::Rgb(200, 100, 0),
            active: Color::Rgb(0, 150, 0),
            inactive: Color::Rgb(180, 180, 180),
            warning: Color::Rgb(200, 150, 0),
            error: Color::Rgb(200, 0, 0),
            success: Color::Rgb(0, 150, 0),
            info: Color::Rgb(0, 100, 200),
            menu_bg: Color::Rgb(220, 220, 220),
            menu_fg: Color::Rgb(20, 20, 20),
            menu_selected: Color::Rgb(200, 100, 0),
            note_color: Color::Rgb(0, 100, 200),
            note_selected: Color::Rgb(200, 100, 0),
            playhead: Color::Rgb(200, 0, 0),
            current_tick: Color::Rgb(0, 150, 0),
            grid: Color::Rgb(200, 200, 200),
            piano_key_white: Color::Rgb(20, 20, 20),
            piano_key_black: Color::Rgb(100, 100, 100),
        }
    }

    pub fn high_contrast_theme() -> Self {
        Self {
            background: Color::Black,
            foreground: Color::White,
            border: Color::White,
            title: Color::White,
            selected: Color::Yellow,
            active: Color::White,
            inactive: Color::DarkGray,
            warning: Color::Yellow,
            error: Color::Red,
            success: Color::White,
            info: Color::White,
            menu_bg: Color::Black,
            menu_fg: Color::White,
            menu_selected: Color::Yellow,
            note_color: Color::White,
            note_selected: Color::Yellow,
            playhead: Color::Red,
            current_tick: Color::White,
            grid: Color::DarkGray,
            piano_key_white: Color::White,
            piano_key_black: Color::DarkGray,
        }
    }
}

