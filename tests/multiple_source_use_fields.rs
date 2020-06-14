use mapper::Mapper;

struct FirstSource {
    room_id: u32,
    name: String,
}

struct SecondSource {
    user_id: u32,
}

#[derive(Mapper)]
#[from(FirstSource, SecondSource)]
struct Destination {
    #[mapper(use_fields = [room_id, user_id])]
    id: u32,
}

fn main() {
    let first = FirstSource {
        room_id: 1,
        name: "kumarmo2".to_string(),
    };
    let second = SecondSource { user_id: 2 };
    let d1 = Destination::from(first);
    let d2 = Destination::from(second);
    assert_eq!(1, d1.id);
    assert_eq!(2, d2.id);
}
