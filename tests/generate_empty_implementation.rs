use mapper::Mapper;

struct Model {}

#[derive(Mapper)]
#[from(Model)]
struct Dto {}

fn main() {}
