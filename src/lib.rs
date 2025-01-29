mod error;
pub use error::Error;

mod settings;
use settings::Settings;

pub fn run() -> Result<(), Error> {
    let settings = Settings::new()?;

    rouille::start_server(format!("0.0.0.0:{}", settings.port), move |_request| {
        rouille::Response::text("Hello, World!")
    });
}
