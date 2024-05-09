pub const CONFIG_FILE: &str = "./cfg/config.json";
pub const SECRETS_FILE: &str = "./cfg/secrets.json";
pub const TOKENS_FILE: &str = "./cfg/tokens.json";

pub const SCOPES: &'static [&str] = &[
    "https://www.googleapis.com/auth/forms.responses.readonly",
    "https://www.googleapis.com/auth/forms.body.readonly",
];
