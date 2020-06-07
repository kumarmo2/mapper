use dtos::FirstDto;
struct FirstModel {
    name: String,
}

struct SecondModel {
    name: String,
}

impl SecondModel {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl FirstModel {
    fn new(name: String) -> Self {
        Self { name }
    }
}
fn main() {
    let model = FirstModel::new("kumarmo2".to_string());
    let dto: FirstDto = FirstDto::from(model);

    println!("dto.name{}", dto.name);

    // First Test: return nothing.
    // Second Test: must send the <from> attribute.
}

mod dtos {
    use super::{FirstModel, SecondModel};
    use mapper::Mapper;

    #[derive(Mapper)]
    // TODO: Handle multiple Types as well like below
    // #[from(FirstModel, SecondModel)]
    #[from(FirstModel)]
    //TODO: use fully qualified name for FirstModel as well.
    pub struct FirstDto {
        pub name: String,
    }
}
