use crate::Error;

#[derive(Debug)]
pub struct Settings {
    pub port: u16,

    pub external_url: String,

    pub password: String,
}

impl Settings {
    pub fn new() -> Result<Self, Error> {
        let settings = config::Config::builder()
            .add_source(config::Environment::with_prefix("NTS"))
            .build()?;

        let port = settings.get_int("port").unwrap_or(8080);
        let external_url = settings.get_string("external_url")?;
        let password = settings.get_string("password")?;

        Ok(Self {
            port: port as u16,
            external_url,
            password,
        })
    }
}
