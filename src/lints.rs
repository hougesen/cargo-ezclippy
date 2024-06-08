#[derive(Debug, serde::Deserialize)]
pub struct Lint {
    pub id: String,
    pub group: String,
    pub level: String,
}

pub fn get_available_lints() -> Result<Vec<Lint>, reqwest::Error> {
    reqwest::blocking::get("https://rust-lang.github.io/rust-clippy/master/lints.json")?
        .json::<Vec<Lint>>()
}
