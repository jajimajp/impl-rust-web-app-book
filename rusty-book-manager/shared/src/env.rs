use std::env;
use strum::EnumString;

#[derive(Default, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum Environment {
    // 開発環境向けで動作している
    #[default]
    Development,
    // 本番環境向けで動作している
    Production,
}

/// 開発環境か本番環境かを示す
pub fn which() -> Environment {
    // debug_assertions が on の場合はデバッグビルド
    // そうでない場合はリリースビルドと判定する
    #[cfg(debug_assertions)]
    let default_env = Environment::Development;
    #[cfg(not(debug_assertions))]
    let default_env = Environment::Production;

    match env::var("ENV") {
        Err(_) => default_env,
        Ok(v) => v.parse().unwrap_or(default_env),
    }
}
