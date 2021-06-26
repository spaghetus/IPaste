use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;
use  ipfs_api::IpfsClient;
use   std::io::Cursor;
use   keybind::{Keybind, Keycode};

#[tokio::main]
async fn main() {
    let mut keybind = Keybind::new(&[Keycode::LControl, Keycode::RControl]);
    loop {
        if keybind.triggered() {
            let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
            match ctx.get_contents() {
                Ok(s) => {
                    println!("keybind hit");
                    upload(s, ctx).await;
                },
                Err(e) => {
                    println!("{}",e);
                }
            }
        }
    }
}

async fn upload(s: String, mut ctx: ClipboardContext){
    let client = IpfsClient::default();
    let data = Cursor::new(s);

    match client.add(data).await {
        Ok(res) => {
            let link = format!("https://dweb.link/ipfs/{}", res.hash);
            ctx.set_contents(link).unwrap()
        },
        Err(e) => eprintln!("error adding file: {}", e)
    }
}
