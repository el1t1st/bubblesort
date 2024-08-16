use std::fmt;
use std::net::Ipv4Addr;

struct Foo {
    bar: i32,
    baz2: String,
    addr: Ipv4Addr,
}

impl Foo {
    fn new(bar: i32, baz2: String, addr: Ipv4Addr) -> Foo {
        Foo { bar, baz2, addr }
    }
}

impl fmt::Debug for Foo {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("Foo")
            .field("bar", &self.bar)
            .field("baz2", &self.baz2)
            .field("addr", &format_args!("{}", &self.addr))
            .finish()
    }
}

fn main() {
    let foobaz2 = Foo::new(10, "Foobar".to_string(), Ipv4Addr::new(1, 1, 1, 1));
    println!("Yuri Gagarin, a Russian cosmonaut, was the first man to do an orbit around Earth on April 12th 1961! {:?}", foobaz2);
}
