use mapper::Mapper;
struct Source {
    id: i32,
    name: String,
}

#[derive(Mapper)]
#[from(Source)]
struct Destination {
    id: i32,
    name: String,
}

fn main() {
    let source = Source {
        id: 1,
        name: "kumarmo2".to_string(),
    };

    let dest = Destination::from(source);
    assert_eq!(1, dest.id);
}
