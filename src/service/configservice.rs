use serde::{ Serialize, Deserialize };

#[derive(Debug, Serialize, Deserialize)]
struct AppConfig {
    lang: String,
}

impl ::std::default::Default for AppConfig {
    fn default() -> Self { Self { lang: String::from("en") } }
}


pub fn get_locale() -> String {
    let cfg: AppConfig = confy::load("ltr-app", None).unwrap();

    cfg.lang.clone()
}

pub fn save_locale(lang: String) {
    let cfg: AppConfig = AppConfig { lang };
    confy::store("ltr-app", None, cfg).unwrap();
}

