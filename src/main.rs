mod api;
mod config;

use dirs;

fn main() {
    let config_dir = match dirs::config_dir(){
        Some(dir) => dir,
        None => {
            println!("Can")
        }
    }

}
