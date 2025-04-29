use rust_i18n::t;

#[derive(PartialEq)]
pub enum NavType {
    Back,
    Quit,
    Details,
    Next,
    Previous,
    Start,
    Language,
}

pub fn get_elements(items: Vec<NavType>, locale: String) -> Vec<(String, String)> {
    let mut nb = vec![(String::from(""), format!(" {}: ", t!("navigation.main", locale = &locale)))];
    if items[0] == NavType::Start {
        nb = vec![];
    }
    items.iter()
        .map(|n| (format!("[{}]", get_shortcut(n)), format!(" {} ", get_text(n, &locale))))
        .for_each(|e| nb.push(e));
    nb
}

fn get_text(nav: &NavType, locale: &str) -> String {
    match nav {
        NavType::Back => t!("navigation.back", locale = locale).to_string(),
        NavType::Quit => t!("navigation.quit", locale = locale).to_string(),
        NavType::Details => t!("navigation.details", locale = locale).to_string(),
        NavType::Next => t!("navigation.next", locale = locale).to_string(),
        NavType::Previous => t!("navigation.previous", locale = locale).to_string(),
        NavType::Start => t!("navigation.start", locale = locale).to_string(),
        NavType::Language => t!("navigation.language", locale = locale).to_string(),
    }
}

fn get_shortcut(nav: &NavType) -> &str {
    match nav {
        NavType::Back => "b",
        NavType::Quit => "q",
        NavType::Details => "d",
        NavType::Next => "->",
        NavType::Previous => "<-",
        NavType::Start => "s",
        NavType::Language => "c"
    }
}



