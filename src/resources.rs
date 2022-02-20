use std::path::Path;
pub enum Page {
    Hello,
    NotFound
}

impl Page {

    fn as_str(&self) -> &'static str {
        match self {
            Page::Hello    => "hello.html",
            Page::NotFound => "404.html",
        }
    }

    pub fn get_path(p: Page) -> String {
        if Path::new(p.as_str()).exists() {
            p.as_str().to_string()
        }
        else {
            format!("/usr/share/hello/{}", p.as_str())
        }
    }
}
