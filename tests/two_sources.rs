use mapper::Mapper;

struct FirstModel {
    id: u32,
}
struct SecondModel {
    id: u32,
}

#[derive(Mapper)]
#[from(FirstModel, SecondModel)]
struct Destination {
    id: u32,
}

fn main() {
    let first_model = FirstModel { id: 1 };
    let second_model = SecondModel { id: 2 };

    let d1 = Destination::from(first_model);
    let d2 = Destination::from(second_model);

    assert_eq!(1, d1.id);
    assert_eq!(2, d2.id);
}
