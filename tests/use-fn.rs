use mapper::Mapper;

struct Source {
    user_id: u32,
}

struct FirstSource {
    room_id: u64,
    name: String,
}

mod utils {
    pub fn from_u64_to_i32(num: u64) -> i32 {
        num as i32
    }
    pub fn from_i64_to_i32(num: i64) -> i32 {
        num as i32
    }

    pub fn to_u32(from: u32) -> i32 {
        from as i32
    }
}
#[derive(Mapper)]
#[from(Source, FirstSource)]
struct Destination {
    #[mapper(use_fns = [utils::to_u32, utils::from_u64_to_i32], use_fields=[user_id, room_id])]
    id: i32,
}

fn main() {
    let source = Source { user_id: 12 };
    let dest = Destination::from(source);
    assert_eq!(dest.id, 12);
}
