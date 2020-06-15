use mapper::Mapper;
struct Source {
    id: i32,
    first_name: String,
    last_name: String,
}

#[derive(Mapper)]
#[from(Source)]
struct Destination {
    id: i32,
    first_name: String,
}

fn main() {
    let source = Source {
        id: 1,
        first_name: "Chuck".to_string(),
        last_name: "Norris".to_string(),
    };
    let _dest = Destination::from(source);
    println!("dest's name: {}", _dest.first_name);
}
