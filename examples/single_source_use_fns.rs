use mapper::Mapper;

struct FirstSource {
    id: u64,
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
#[from(FirstSource)]
struct Destination {
    #[mapper(use_fns = [utils::from_u64_to_i32])]
    id: i32,
    name: String,
}

fn main() {
    let source = FirstSource {
        id: 12,
        name: "Chuck Norris".to_string(),
    };
    let _dest = Destination::from(source);
    println!("name: {}", _dest.name);
}
