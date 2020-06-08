use dtos::FirstDto;
mod models {
    pub struct FirstModel {
        pub name: String,
    }

    pub struct SecondModel {
        pub name: String,
    }

    impl SecondModel {
        pub fn new(name: String) -> Self {
            Self { name }
        }
    }

    impl FirstModel {
        pub fn new(name: String) -> Self {
            Self { name }
        }
    }
}
fn main() {
    let model = models::SecondModel::new("kumarmo2".to_string());
    let model2 = models::FirstModel::new("mohit".to_string());

    let dto = FirstDto::from(model);
    let dto2 = FirstDto::from(model2);

    println!("dto.name{}", dto.name);
    println!("dto.name{}", dto2.name);

    // First Test: return nothing.
    // Second Test: must send the <from> attribute.
    // Third Test: Must generate From Definition for single type
    // Fourth Test: Must generate From Definiton for multiple types.
    // Fifth Test: If fields with same name are of not same type, but implements From<S'>, use that to generated the value.
    // TEST: Can we make it work for the Enums ?
    // TEST: Make it work with fully Qualified paths in from attribute.
}

mod dtos {
    use crate::models;
    use mapper::Mapper;

    #[derive(Mapper)]
    #[from(models::FirstModel, models::SecondModel)]
    pub struct FirstDto {
        pub name: String,
    }
}
