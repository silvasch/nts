use base64::prelude::*;
use jiff::{tz::TimeZone, Timestamp};

mod error;
use std::io::Read;

pub use error::Error;

mod settings;
use rouille::{Request, Response};
use settings::Settings;

const RAW_WRITE_SCRIPT: &str = include_str!("../scripts/new.sh");
const RAW_READ_SCRIPT: &str = include_str!("../scripts/get.sh");

pub fn run() -> Result<(), Error> {
    let settings = Settings::new()?;

    std::fs::create_dir_all(&settings.data_dir).unwrap();

    rouille::start_server(
        format!("0.0.0.0:{}", settings.port),
        move |request| match request.url().as_str() {
            "/api/new" => {
                if request.method() != "POST" {
                    return Response::text("this route only allows POST requests.")
                        .with_status_code(405);
                }

                if !authenticate(&settings.password, request) {
                    return Response::text("invalid password").with_status_code(401);
                }

                let mut body = request
                    .data()
                    .expect("should only panic if Request::data was called before.");
                let mut note = String::new();
                body.read_to_string(&mut note)
                    .map_err(Error::BodyRead)
                    .unwrap();
                note = note.trim().to_string();
                if note.is_empty() {
                    return Response::text("note is empty - it won't be saved");
                }

                let file_name = &settings
                    .data_dir
                    .join(jiff::Timestamp::now().as_millisecond().to_string())
                    .with_extension("txt");

                std::fs::create_dir_all(file_name.parent().unwrap()).unwrap();
                std::fs::write(file_name, note)
                    .map_err(Error::WriteNote)
                    .unwrap();

                Response::text("ok")
            }
            "/api/get" => {
                if request.method() != "GET" {
                    return Response::text("this route only allows GET requests.")
                        .with_status_code(405);
                }

                if !authenticate(&settings.password, request) {
                    return Response::text("invalid password").with_status_code(401);
                }

                let mut text = String::new();

                let mut files = vec![];

                for file in std::fs::read_dir(&settings.data_dir).unwrap() {
                    files.push(file.unwrap().path());
                }

                files.sort_unstable();

                for file in files {
                    let contents = std::fs::read_to_string(&file).unwrap();
                    text.push_str(&format!(
                        "{}\n=====\n> {}\n\n",
                        Timestamp::from_millisecond(
                            file.file_stem().unwrap().to_string_lossy().parse().unwrap()
                        )
                        .unwrap()
                        .to_zoned(TimeZone::system())
                        .strftime("%a %b %d %H:%M:%S %Y"),
                        contents.replace('\n', "\n> ")
                    ));
                }

                Response::text(text)
            }
            "/new" => {
                if request.method() != "GET" {
                    return Response::text("this route only allows GET requests.")
                        .with_status_code(405);
                }

                let script = RAW_WRITE_SCRIPT.replace("{{HOST}}", &settings.external_url);

                Response::text(script)
            }
            "/get" => {
                if request.method() != "GET" {
                    return Response::text("this route only allows GET requests.")
                        .with_status_code(405);
                }

                let script = RAW_READ_SCRIPT.replace("{{HOST}}", &settings.external_url);

                Response::text(script)
            }
            route => {
                Response::text(format!("'{}' is not a valid route.", route)).with_status_code(404)
            }
        },
    );
}

fn authenticate(expected_password: &str, request: &Request) -> bool {
    let authorization = match request.header("Authorization") {
        Some(authorization) => authorization,
        None => return false,
    };
    let base64_password = authorization.split_whitespace().nth(1).unwrap();
    let raw_password = BASE64_STANDARD.decode(base64_password).unwrap();
    let password = std::str::from_utf8(&raw_password).unwrap().trim_end();

    password == expected_password
}
