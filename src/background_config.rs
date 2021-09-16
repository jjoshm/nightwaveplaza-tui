use std::time::Duration;
use viuer::Config as ViuerConfig;

pub struct Config<'a> {
    pub files: Vec<&'a str>,
    pub loop_gif: bool,
    pub name: bool,
    pub mirror: bool,
    pub recursive: bool,
    pub static_gif: bool,
    pub viuer_config: ViuerConfig,
    pub frame_duration: Option<Duration>,
}

impl<'a> Config<'a> {
    pub fn new(img: Vec<&'a str>) -> Config<'a> {

        let viuer_config = ViuerConfig {
            #[cfg(feature = "sixel")]
            use_sixel: false,
            ..Default::default()
        };
        Config {
            files: img,
            loop_gif: true,
            name: false,
            mirror: false,
            recursive: false,
            static_gif: false,
            viuer_config: viuer_config,
            frame_duration: None,
        }
    }
}
