extern crate chrono;
extern crate image;
extern crate notify_rust;
extern crate rand;
extern crate viuer;

use std::{env, fs::File, path::{Path}, thread};
use background_config::Config;
use std::io;
use rand::Rng;
mod background;
mod background_config;
mod audiostream;

fn download_image(url: &str) -> String {
    let img_bytes = reqwest::blocking::get(url).unwrap().bytes().unwrap();
    let mut bytes = img_bytes.as_ref();
    let path = env::temp_dir().join(Path::new("bg.gif"));
    let _path = path.clone();
    let mut out = File::create(path).unwrap();
    io::copy(&mut bytes, &mut out);
    
    return _path.into_os_string().into_string().unwrap();
}

fn current_timestamp() -> String {
    chrono::Local::now().format("[%H:%M:%S]").to_string()
}

fn notify(title: &str) {
    println!("{} {}\n", current_timestamp(), title);
    let _notify = notify_rust::Notification::new()
        .summary("Now playing")
        .body(title)
        .show();
}

fn rand_image_url() -> String {
    let mut rng = rand::thread_rng();
    let rand = rng.gen_range(1..83).to_string();
    if rand.len() < 2 {
        return format!("https://plaza.one/img/backs/{}{}.gif", "0", rand);
    } else {
        return format!("https://plaza.one/img/backs/{}.gif", rand);
    }
}

fn main() {
    let callback = |title: &str| {
        notify(title);
    };

    let image = download_image(rand_image_url().as_str());

    let conf = Config::new(vec![image.as_str()]);
    thread::spawn(move || audiostream::open("https://plaza.one/ogg", &callback));

    background::run(conf);

}
