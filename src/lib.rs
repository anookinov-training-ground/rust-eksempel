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

pub fn strlen3<S: AsRef<str>>(s: S) -> usize {
    s.as_ref().len()
}

pub fn strlen_dyn2(s: Box<dyn AsRef<str>>) -> usize {
    s.as_ref().as_ref().len()
}

pub fn strlen_dyn(s: &dyn AsRef<str>) -> usize {
    s.as_ref().len()
}

pub trait Hei // where
//     Self: Sized,
{
    type Name;

    fn hei(&self);

    fn weird()
    where
        Self: Sized,
    {
    }
}

impl Hei for &str {
    type Name = ();

    fn hei(&self) {
        println!("hei {}", self);
    }

    fn weird() {}
}

impl Hei for String {
    type Name = ();

    fn hei(&self) {
        println!("hei {}", self);
    }
}

impl Hei for Box<dyn AsRef<str>> {
    type Name = ();

    fn hei(&self) {
        println!("hei {}", self.as_ref().as_ref());
    }
}

pub fn say_hei_static<H: Hei>(s: H) {
    s.hei();
}

pub fn say_hei_static_str(s: &str) {
    s.hei(); // call compiled static assembly code
}

pub fn say_hei(s: &dyn Hei<Name = ()>) {
    // &dyn Hei
    // stored in &
    //  1. a pointer to the actual, concrete, implementing type
    //  2. a pointer to a vtable for the referenced trait
    //
    // what is a vtable (virtual dispatch table)?
    // dyn Hei, vtable:
    //  struct HeiVtable {
    //    hei: *mut Fn(*mut ()),
    //  }
    //
    // &str -> &dyn Hei
    //
    // 1. pointer to the &str
    // 2. HeiVtable {
    //   hei: &<str as Hei>::hei
    // }
    // String -> &dyn Hei
    //
    // 1. pointer to the String
    // 2. HeiVtable {
    //   hei: &<String as Hei>::hei
    // }
    s.hei();
    // s.vtable.hei(s.pointer)

    // (dyn Hei)::weird();
    // s::weird();
}

// struct Foo {
//     s: [u8],
// }

// pub fn foo_dyn(h: Hei) {}

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

// pub fn bar_dyn(s: &[dyn Hei]) {
//     for h in s {
//         h.hei();
//     }
// }

pub trait HeiAsRef: Hei<Name = ()> + AsRef<str> {}

pub fn baz(s: &dyn HeiAsRef) {
    s.hei();
    let s = s.as_ref();
    s.len();
}

pub fn bool_then<T>(b: bool, f: impl FnOnce() -> T) -> Option<T> {
    if b {
        Some(f())
    } else {
        None
    }
}

use std::iter::Extend;

// pub fn add_true(v: &mut dyn Extend<bool>) {
//     v.extend(std::iter::once(true));
// }

struct MyVec<T>(Vec<T>);

impl<T> Extend<T> for MyVec<T> {
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = T>,
    {
        // ...
    }

    // fn extend_hashmap_intoiter_bool(
    //     &mut self,
    //     iter: std::collection::hash_map::IntoIterator<bool>,
    // ) {
    //     // ...
    // }
}

// fn clone(&self) -> Self
// pub fn clone(v: &dyn Clone) {
//     let x = v.clone();
// }

pub fn it(v: &mut dyn Iterator<Item = bool>) {
    let _ = v.next();
}

pub fn drop(v: &mut dyn Drop) {
    // when v goes out of scope, Drop::drop is called
}

pub fn say_hei_box(s: Box<dyn AsRef<str>>) {
    // what happens when s goes out of scope?
    // Drop Trait is implemented for trait object and available in vtable by default
}

// dyn Trait -> * -> (*mut data, *mut vtable)
// [u8]      -> * -> (*mut data, usize length)
// str       -> * -> (*mut data, usize length)

fn foo_u8(s: &[u8]) {}

fn bar_u8() -> Box<[u8]> {
    // [][..]
    Box::new([]) as Box<[u8]>
}

struct Foo {
    f: bool,
    x: bool,
    t: [u8],
}
// &Foo (*Foo, length t)
// Box<[u8]> != Vec<u8>
// dyn Fn != fn

fn foo_fn(f: &dyn Fn()) {} // wide pointer including data pointer and vtable

fn bar_fn(f: fn()) {} // function

fn baz_fn(f: impl Fn()) {}

// struct Wrapper<F: Fn()> {
//     f: F,
// }

struct Wrapper {
    f: Box<dyn Fn()>,
}

trait X {
    fn foo(&self, f: impl Fn());
}

trait X2 {
    fn foo<F: Fn()>(&self, f: F);
}

// object safe
trait X3 {
    fn foo(&self, f: &dyn Fn());
}

fn quox(x: &dyn X3) {}

pub fn main() {
    let x = Box::new(String::from("hello"));
    let y: Box<dyn AsRef<str>> = x;
    strlen_dyn2(y);

    let z: Box<dyn AsRef<str>> = Box::new(String::from("hello"));
    strlen_dyn2(z);

    let a: &dyn AsRef<str> = &"world";
    strlen_dyn(a);

    let random = 4; // read from the user
    if random == 4 {
        say_hei(&"hello");
    } else {
        say_hei(&String::from("world"));
    }

    let x = "Hello";
    foo_fn(&|| {
        let _ = &x;
    });

    // bar_fn(&|| {
    //     let _ = &x;
    // })

    baz_fn(|| {
        let _ = &x;
    })
}
