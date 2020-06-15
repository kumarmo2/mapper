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
    let _d1 = Destination::from(first);
    let _d2 = Destination::from(second);
}
