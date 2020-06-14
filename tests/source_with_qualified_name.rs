mod sources {
    pub struct Source {
        pub user_id: i32,
    }
}

mod destinations {
    use mapper::Mapper;
    #[derive(Mapper)]
    #[from(super::sources::Source)]
    pub struct Destination {
        #[mapper(use_fields = [user_id])]
        pub id: i32,
    }
}

fn main() {
    let source = crate::sources::Source { user_id: 1 };
    let destination = crate::destinations::Destination::from(source);
    assert_eq!(1, destination.id);
}
