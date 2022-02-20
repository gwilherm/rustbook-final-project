use std::path::Path;
pub enum Page {
    Hello,
    NotFound
}

impl Page {
    pub fn get_path(&self) -> String {
        match self {
            Page::Hello    => Page::lookup_path("hello.html"),
            Page::NotFound => Page::lookup_path("404.html"),
        }
    }

    pub fn lookup_path(p: &str) -> String {
        if Path::new(p).exists() {
            p.to_string()
        }
        else {
            format!("/usr/share/hello/{}", p)
        }
    }
}
