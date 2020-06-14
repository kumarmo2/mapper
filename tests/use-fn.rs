use mapper::Mapper;

struct Source {
    user_id: i32,
}

struct FirstSource {
    room_id: i32,
    name: String,
}

#[derive(Mapper)]
#[from(Source, FirstSource)]
struct Destination {
    #[mapper(use_fns = [to_u32], use_fields=[user_id, room_id])]
    id: i32,
}

/*

    FieldModifier {
        modifiers: Vec<MapperOptions>
    }

    enum MapperOptions{
        UseFields()
        UseFns()
    }

    UseField {
        pub key: Ident
        pub use_fields: Punctuated<Ident, Comma>,
    }

    UseFn {
        pub key: Ident,
        pub use_fns: Punctuated<Path, Comma>,
    }
*/

fn to_u32(from: u32) -> i32 {
    from as i32
}

fn main() {
    let source = Source { user_id: 12 };
    let x: i32 = 0;
    let y: u32 = x as u32;
    let dest = Destination::from(source);
    assert_eq!(dest.id, 12);
}
