use base64::prelude::*;

mod error;
use std::io::Read;

pub use error::Error;

mod settings;
use rouille::Response;
use settings::Settings;

const RAW_SCRIPT: &str = include_str!("../script.sh");

pub fn run() -> Result<(), Error> {
    let settings = Settings::new()?;

    let data_dir = xdg::BaseDirectories::with_prefix("nts")?.get_data_home();

    rouille::start_server(
        format!("0.0.0.0:{}", settings.port),
        move |request| match request.url().as_str() {
            "/new" => {
                if request.method() != "POST" {
                    return Response::text("this route only allows POST requests.")
                        .with_status_code(405);
                }

                let authorization = request.header("Authorization").unwrap();
                let base64_password = authorization.split_whitespace().nth(1).unwrap();
                let raw_password = BASE64_STANDARD.decode(base64_password).unwrap();
                let password = std::str::from_utf8(&raw_password).unwrap().trim_end();

                if password != settings.password {
                    return Response::text("invalid password").with_status_code(401);
                }

                let mut body = request
                    .data()
                    .expect("should only panic if Request::data was called before.");
                let mut note = String::new();
                body.read_to_string(&mut note)
                    .map_err(Error::BodyRead)
                    .unwrap();

                let file_name = data_dir
                    .join(jiff::Timestamp::now().as_millisecond().to_string())
                    .with_extension("txt");

                std::fs::create_dir_all(&file_name.parent().unwrap()).unwrap();
                std::fs::write(file_name, note)
                    .map_err(Error::WriteNote)
                    .unwrap();

                Response::text("ok")
            }
            "/script" => {
                if request.method() != "GET" {
                    return Response::text("this route only allows GET requests.")
                        .with_status_code(405);
                }

                let script = RAW_SCRIPT.replace("{{HOST}}", &settings.external_url);

                Response::text(script)
            }
            route => {
                Response::text(format!("'{}' is not a valid route.", route)).with_status_code(404)
            }
        },
    );
}
