mod error;
pub use error::Error;

mod settings;
use settings::Settings;

pub fn run() -> Result<(), Error> {
    let settings = Settings::new()?;

    dbg!(&settings);

    Ok(())
}
