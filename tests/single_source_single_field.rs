use mapper::Mapper;

struct Source {
    id: i32,
}

#[derive(Mapper)]
#[from(Source)]
struct Destination {
    id: i32,
}

fn main() {
    let source = Source { id: 1 };
    let dest = Destination::from(source);
    assert_eq!(1, dest.id);
}
