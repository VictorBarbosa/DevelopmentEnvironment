use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub prefix: String,
    pub remove_word: Option<String>,
    pub use_db_name: Option<String>,
    pub db_password: Option<String>,
    pub external_port: Option<u16>,
    pub internal_port: Option<u16>,
    pub script_sql_path: Option<String>,
    pub dockerfile_path: Option<String>,
}
