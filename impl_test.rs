pub struct Rafy {
    url: String,
    info: Info,
}

pub struct Info {
    info: String,
}

impl Rafy {
    pub fn new(url: &str) -> Rafy {
        let info = Info { info: "This is some description".to_string() };
        Rafy { url: url.to_string(), info: info }
    }
    pub fn get_total(&self) -> String {
        format!("{} {}", self.url, "<- Google!")
    }
}

impl Info {
    pub fn get_info(&self) -> String {
        format!("{} <- Description!", self.info)
    }
}

fn main() {
    let thing = Rafy::new("http://google.com/");
    println!("{}", thing.url);
    println!("{}", thing.get_total());
    println!("{}", thing.info.info);
    println!("{}", thing.info.get_info());
