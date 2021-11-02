pub fn strlen(s: impl AsRef<str>) -> usize {
    s.as_ref().len()
}

pub fn strlen_refstr(s: &str) -> usize {
    s.len()
}

pub fn strlen_string(s: String) -> usize {
    s.len()
}

pub fn strlen2<S>(s: S) -> usize
where
    S: AsRef<str>,
{
    s.as_ref().len()
}

pub fn bool_then<T>(b: bool, f: impl FnOnce() -> T) -> Option<T> {
    if b {
        Some(f())
    } else {
        None
    }
}

pub trait Hei {
    fn hei(&self);
}

impl Hei for &str {
    fn hei(&self) {
        println!("hei {}", self);
    }
}

impl Hei for String {
    fn hei(&self) {
        println!("hei {}", self);
    }
}

pub fn foo() {
    strlen("hello world"); // &'static str
    strlen(String::from("hei verden")); // String: AsRef<str>

    "J".hei();

    for h in vec!["J", "Jon"] {
        h.hei();
    }

    bar_slice(&["J", "Jon"]);
    bar_slice(&[String::from("J"), String::from("Jon")]);
}

pub fn bar(h: impl Hei) {
    h.hei();
}

pub fn bar2<H: Hei>(h: H) {
    h.hei();
}

pub fn bar_str(h: &str) {
    h.hei();
}

pub fn bar_slice(s: &[impl Hei]) {
    for h in s {
        h.hei();
    }
}
