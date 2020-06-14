use mapper::Mapper;

struct Source {
    user_id: u32,
}

#[derive(Mapper)]
#[from(Source)]
struct Destination {
    #[mapper(use_fields = [user_id])]
    id: u32,
}

fn main() {
    let source = Source { user_id: 12 };
    let dest = Destination::from(source);
    assert_eq!(dest.id, 12);
}
