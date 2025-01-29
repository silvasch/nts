use crate::Error;

#[derive(Debug)]
pub struct Settings {
    pub external_url: String,
}

impl Settings {
    pub fn new() -> Result<Self, Error> {
        let settings = config::Config::builder()
            .add_source(config::File::with_name("config"))
            .add_source(config::Environment::with_prefix("NTS"))
            .build()?;

        let external_url = settings.get_string("external_url")?;

        Ok(Self { external_url })
    }
}
