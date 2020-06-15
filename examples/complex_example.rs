use mapper::Mapper;

mod utils {
    pub fn from_u64_to_i32(num: u64) -> i32 {
        num as i32
    }
    pub fn from_i64_to_i32(num: i64) -> i32 {
        num as i32
    }

    fn to_u32(from: u32) -> i32 {
        from as i32
    }

    pub fn convert_string_i32_list(input: Vec<i32>) -> Vec<String> {
        input.into_iter().map(|val| val.to_string()).collect()
    }
}
struct InnerSource {
    inner_id: i64,
    inner_list: Vec<i32>,
}

struct OuterSource {
    outer_id: i32,
    inner_source: InnerSource,
}

#[derive(Mapper)]
#[from(InnerSource)]
struct InnerDest {
    #[mapper(use_fields = [inner_list], use_fns = [utils::convert_string_i32_list])]
    list: Vec<String>,
    #[mapper(use_fields = [inner_id], use_fns = [utils::from_i64_to_i32])]
    id: i32,
}

#[derive(Mapper)]
#[from(OuterSource)]
struct OuterDest {
    #[mapper(use_fields = [outer_id])]
    id: i32,
    #[mapper(use_fields = [inner_source], use_fns= [InnerDest::from])]
    source: InnerDest,
}

fn main() {
    let inner_source = InnerSource {
        inner_id: 1,
        inner_list: vec![1, 2, 3],
    };

    let outer_source = OuterSource {
        outer_id: 2,
        inner_source,
    };

    let _outer_dest = OuterDest::from(outer_source);
}
