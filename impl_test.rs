pub struct Rafy {
    url: String,
    calls_function: String,
    info: Info,
}

pub struct Info {
    info: String,
}

impl Rafy {
    pub fn new(url: &str) -> Rafy {
        let info = Info { info: "This is some description".to_string() };
        let call_function = Rafy::a_function();
        Rafy { url: url.to_string(), calls_function: call_function, info: info }
    }
    pub fn get_total(&self) -> String {
        format!("{} {}", self.url, "<- Google!")
    }
    fn a_function() -> String {
        "return from function".to_string()
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
    println!("{}", thing.calls_function);
}
