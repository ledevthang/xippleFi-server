use std::env::VarError;

pub struct EnvError(pub String);

pub fn read_env(env: &str) -> Result<String, EnvError> {
    std::env::var(env).map_err(|error| match error {
        VarError::NotUnicode(message) => EnvError(message.to_str().unwrap_or_default().to_string()),
        VarError::NotPresent => EnvError(format!("Missing {} env configuration", env)),
    })
}
