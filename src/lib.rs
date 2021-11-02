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

pub fn foo() {
    strlen("hello world"); // &'static str
    strlen(String::from("hei verden")); // String: AsRef<str>
}
