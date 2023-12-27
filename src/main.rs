use std::{char, env};

use rand::Rng;

enum Theme {
    All,
    Lower,
    Upper,
    Numbers,
    Letters,
    Smileys,
    Food,
    Animals,
    Ascii,
    Christmas,
    Easter,
    Halloween,
    NewYear,
    Thanksgiving,
    Valentine,
}

struct ThemeInfo {
    name: &'static str,
    chars: Vec<char>,
}

impl ThemeInfo {
    fn new(name: &'static str, chars: Vec<char>) -> Self {
        Self {
            name,
            chars,
        }
    }
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let speed: u32 = if args.len() > 1 {
        match args[1].parse::<u32>() {
            Ok(speed) => speed,
            Err(_) => {
                println!("Invalid speed: {}", args[1]);
                std::process::exit(1);
            }
        }
    } else {
        100
    };

    let theme: &str = if args.len() > 2 {
        &args[2]
    } else {
        "all"
    };

    let themes = init_themes();

    let mut chars = Vec::new();
    for theme_info in &themes {
        if theme_info.name == theme {
            chars = theme_info.chars.clone();
            break;
        }
    }

    if chars.is_empty() {
        println!("Invalid theme: {}", theme);
        println!("Available themes:");
        for theme_info in themes {
            println!("  {}", theme_info.name);
        }

        std::process::exit(1);
    }

    clear_screen();
    println!("vec size: {}", chars.len());
    loop {
        scroll_background(&chars);
        std::thread::sleep(std::time::Duration::from_millis(speed as u64));
    }
}

fn scroll_background(chars: &Vec<char>) {
    let mut rng = rand::thread_rng();
    let mut line = String::new();

    let term_size = term_size::dimensions().unwrap();

    for _ in 0..rng.gen_range(0..term_size.0) {
        line.push(' ');
    }

    for _ in 0..rng.gen_range(0..4) {
        line.push(chars[rng.gen_range(0..chars.len())]);
    }
    
    println!("{}", line);
}

fn init_themes() -> Vec<ThemeInfo> {
    let mut themes = Vec::new();

    themes.push(ThemeInfo::new("all", load_theme(Theme::All)));
    themes.push(ThemeInfo::new("lower", load_theme(Theme::Lower)));
    themes.push(ThemeInfo::new("upper", load_theme(Theme::Upper)));
    themes.push(ThemeInfo::new("numbers", load_theme(Theme::Numbers)));
    themes.push(ThemeInfo::new("letters", load_theme(Theme::Letters)));
    themes.push(ThemeInfo::new("smileys", load_theme(Theme::Smileys)));
    themes.push(ThemeInfo::new("food", load_theme(Theme::Food)));
    themes.push(ThemeInfo::new("animals", load_theme(Theme::Animals)));
    themes.push(ThemeInfo::new("ascii", load_theme(Theme::Ascii)));
    themes.push(ThemeInfo::new("christmas", load_theme(Theme::Christmas)));
    themes.push(ThemeInfo::new("easter", load_theme(Theme::Easter)));
    themes.push(ThemeInfo::new("halloween", load_theme(Theme::Halloween)));
    themes.push(ThemeInfo::new("newyear", load_theme(Theme::NewYear)));
    themes.push(ThemeInfo::new("thanksgiving", load_theme(Theme::Thanksgiving)));
    themes.push(ThemeInfo::new("valentine", load_theme(Theme::Valentine)));

    themes
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn load_theme(theme: Theme) -> Vec<char> {

    let mut chars = Vec::new();

    match theme {
        Theme::All => {
            chars.extend(load_theme(Theme::Lower));
            chars.extend(load_theme(Theme::Upper));
            chars.extend(load_theme(Theme::Numbers));
            chars.extend(load_theme(Theme::Smileys));
            chars.extend(load_theme(Theme::Food));
            chars.extend(load_theme(Theme::Animals));
            chars.extend(load_theme(Theme::Ascii));
            chars.extend(load_theme(Theme::Christmas));
            chars.extend(load_theme(Theme::Easter));
            chars.extend(load_theme(Theme::Halloween));
            chars.extend(load_theme(Theme::NewYear));
            chars.extend(load_theme(Theme::Thanksgiving));
            chars.extend(load_theme(Theme::Valentine));
        }

        Theme::Lower => {
            for i in 0x61..=0x7A {
                if let Some(c) = char::from_u32(i) {
                    chars.push(c);
                }
            }
        }

        Theme::Upper => {
            for i in 0x41..=0x5A {
                if let Some(c) = char::from_u32(i) {
                    chars.push(c);
                }
            }
        }

        Theme::Numbers => {
            for i in 0x30..=0x39 {
                if let Some(c) = char::from_u32(i) {
                    chars.push(c);
                }
            }
        }

        Theme::Letters => {
            chars.extend(load_theme(Theme::Lower));
            chars.extend(load_theme(Theme::Upper));
        }

        Theme::Smileys => {
            for i in 0x1F600..=0x1F64F {
                if let Some(c) = char::from_u32(i) {
                    chars.push(c);
                }
            }
        }

        Theme::Food => {
            for i in 0x1F300..=0x1F35F {
                if let Some(c) = char::from_u32(i) {
                    chars.push(c);
                }
            }
        }

        Theme::Animals => {
            for i in 0x1F400..=0x1F43F {
                if let Some(c) = char::from_u32(i) {
                    chars.push(c);
                }
            }
        }

        Theme::Ascii => {
            for i in 0x20..=0x7E {
                if let Some(c) = char::from_u32(i) {
                    chars.push(c);
                }
            }
        }

        Theme::Christmas => {
            chars.push('🎄');
            chars.push('🎅');
            chars.push('🤶');
            chars.push('🎁');
            chars.push('🦌');
            chars.push('🧦');
            chars.push('🔔');
            chars.push('🎶');
            chars.push('🎵');
        }

        Theme::Easter => {
            for i in 0x1F383..=0x1F3C6 {
                if let Some(c) = char::from_u32(i) {
                    chars.push(c);
                }
            }
        }

        Theme::Halloween => {
            for i in 0x1F383..=0x1F3C6 {
                if let Some(c) = char::from_u32(i) {
                    chars.push(c);
                }
            }
        }

        Theme::NewYear => {
            for i in 0x1F384..=0x1F3C6 {
                if let Some(c) = char::from_u32(i) {
                    chars.push(c);
                }
            }
        }

        Theme::Thanksgiving => {
            for i in 0x1F383..=0x1F3C6 {
                if let Some(c) = char::from_u32(i) {
                    chars.push(c);
                }
            }
        }

        Theme::Valentine => {
            for i in 0x1F48C..=0x1F49F {
                if let Some(c) = char::from_u32(i) {
                    chars.push(c);
                }
            }
        }
    }

    chars
}