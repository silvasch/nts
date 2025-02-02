use std::path::Path;

fn main() {
    let data_dir = std::env::var("NTS_DATA_DIR")
        .map(|raw_data_dir| Path::new(&raw_data_dir).to_path_buf())
        .unwrap_or(
            xdg::BaseDirectories::with_prefix("nts")
                .unwrap()
                .get_data_home(),
        );

    let password = rpassword::prompt_password("pwd> ").unwrap();
    let password_hash = sha256::digest(password.trim());

    std::fs::create_dir_all(&data_dir).unwrap();
    std::fs::write(data_dir.join("pwd"), password_hash).unwrap();

    println!("ok");
}
