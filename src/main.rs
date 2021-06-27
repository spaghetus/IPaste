mod de;
use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use ipfs_api::IpfsClient;
use keybind::{Keybind, Keycode};
use std::fs;
use std::io::Cursor;
use std::path::PathBuf;

#[tokio::main]
async fn main() {
    let bind_path = dirs::config_dir()
        .unwrap_or(PathBuf::from("no"))
        .join("ipaste-bind.json");
    let mut keybind = Keybind::new(
        match fs::read_to_string(bind_path.clone()) {
            Err(_) => { // The read failed, so we can fall back to the default.
                println!("Keybinds can be configured at {:?}, which will suppress this message.", bind_path);
                vec![Keycode::LControl, Keycode::RControl]
            }
            Ok(content) => {
                // It's safe to panic here because the user has probably made a mistake.
                let def: Vec<de::KeyCodeDef> =
                    serde_json::from_str(&content).expect("Invalid keybind spec");
                // Convert everything from our enums to keybind's
                def.iter()
                    .map(|def| Keycode::from(def.clone()))
                    .collect::<Vec<Keycode>>()
            }
        }
        .as_slice(),
    );
    loop {
        if keybind.triggered() {
            let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
            match ctx.get_contents() {
                Ok(s) => {
                    println!("keybind hit");
                    upload(s, ctx).await;
                }
                Err(e) => {
                    println!("{}", e);
                }
            }
        }
    }
}

async fn upload(s: String, mut ctx: ClipboardContext) {
    let client = IpfsClient::default();
    let data = Cursor::new(s);

    match client.add(data).await {
        Ok(res) => {
            let link = format!("https://dweb.link/ipfs/{}", res.hash);
            ctx.set_contents(link).unwrap()
        }
        Err(e) => eprintln!("error adding file: {}", e),
    }
}
