use mapper::Mapper;

struct FirstSource {
    id: i32,
}

struct SecondSource {
    user_id: i32,
}

#[derive(Mapper)]
#[from(FirstSource, SecondSource)]
struct FirstDestination {
    #[mapper(use_fields = [id, user_id])]
    id: i32,
}

#[derive(Mapper)]
#[from(SecondSource, FirstSource)]
struct SecondDestination {
    #[mapper(use_fields = [user_id])]
    // NOTE: we only gave for the SecondSource and skipped for FirstSource as it has same name.
    id: i32,
}

fn main() {
    let s1 = FirstSource { id: 1 };
    let s2 = SecondSource { user_id: 2 };
    let s3 = FirstSource { id: 3 };
    let s4 = SecondSource { user_id: 4 };

    let d1 = FirstDestination::from(s1);
    let d2 = FirstDestination::from(s2);
    let d3 = SecondDestination::from(s3);
    let d4 = SecondDestination::from(s4);

    assert_eq!(1, d1.id);
    assert_eq!(2, d2.id);
    assert_eq!(3, d3.id);
    assert_eq!(4, d4.id);
}
