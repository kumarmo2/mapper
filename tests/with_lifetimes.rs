use mapper::Mapper;

struct FirstSource<'b> {
    name: &'b str,
}

#[derive(Mapper)]
#[from(FirstSource<'a>)] //TODO: Remove the necessity of using the lifetime parameter in from attribute.
struct Destination<'a> {
    name: &'a str,
}

fn main() {
    let first = FirstSource { name: "kumarmo2" };
    let d1 = Destination::from(first);
    assert_eq!("kumarmo2", d1.name);
}
