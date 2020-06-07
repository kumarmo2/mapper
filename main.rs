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
    let model = SecondModel::new("kumarmo2".to_string());
    let dto = FirstDto::from(model);

    println!("dto.name{}", dto.name);

    // First Test: return nothing.
    // Second Test: must send the <from> attribute.
    // Third Test: Must generate From Definition for single type
    // Fourth Test: Must generate From Definiton for multiple types.
    // Fifth Test: If fields with same name are of not same type, but implements From<S'>, use that to
    // generated the value.
    // TEST: Can we make it work for the Enums ?
    // TEST: Make it work with fully Qualified paths in from attribute.
}

mod dtos {
    use super::{FirstModel, SecondModel};
    use mapper::Mapper;

    #[derive(Mapper)]
    // TODO: Handle multiple Types as well like below
    #[from(FirstModel, SecondModel)]
    // #[from(FirstModel)]
    //TODO: use fully qualified name for FirstModel as well.
    pub struct FirstDto {
        pub name: String,
    }
}
