mod error;
use std::io::Read;

pub use error::Error;

mod settings;
use rouille::Response;
use settings::Settings;

pub fn run() -> Result<(), Error> {
    let settings = Settings::new()?;

    rouille::start_server(
        format!("0.0.0.0:{}", settings.port),
        move |request| match request.url().as_str() {
            "/new" => {
                if request.method() != "POST" {
                    return Response::text("this route only allows POST requests.")
                        .with_status_code(405);
                }

                let mut body = request
                    .data()
                    .expect("should only panic if Request::data was called before.");
                let mut note = String::new();
                body.read_to_string(&mut note)
                    .map_err(Error::BodyRead)
                    .unwrap();

                println!("{}", note);

                Response::text("ok")
            }
            route => {
                Response::text(format!("'{}' is not a valid route.", route)).with_status_code(404)
            }
        },
    );
}
