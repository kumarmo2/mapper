# mapper

mapper is a simple mapping library to generate code `From<T> for S`.

It is loosely inspired by `c# library` [Automapper](https://github.com/AutoMapper/AutoMapper).

## Inspiration

Although this library can be used in any context, but specially in Web Development, many times there are

lots of converstion from `models` and `dtos`. Writing manual code is a bit tedious. This library tries helping

by generating the `From<T>` trait implementations for the types.

## Installation

Add the below in `Cargo.toml`

```
mapper = {git = "https://github.com/kumarmo2/mapper.git", branch = "v0.1"}
```

## Usage

### Quickstart

```rust
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

```

### Custom Field Name Mapping

Custom Field mapping can be configured using `mapper` attribute with `use_fields` key and in value passing

the Source's field name.

```rust

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
    #[mapper(use_fields = [first_name])]
    name: String,
}

fn main() {
    let source = Source {
        id: 1,
        first_name: "Chuck".to_string(),
        last_name: "Norris".to_string(),
    };
    let _dest = Destination::from(source);
    println!("dest's name: {}", _dest.name);
}

```

### Field Type Conversion

If a field's type needs to be converted, it can be configured using `mapper` attribute with `use_fns` key and

and in value pass the function.

```rust

use mapper::Mapper;

struct FirstSource {
    id: u64,
    name: String,
}

mod utils {
    pub fn from_u64_to_i32(num: u64) -> i32 {
        num as i32
    }
    pub fn from_i64_to_i32(num: i64) -> i32 {
        num as i32
    }

    pub fn to_u32(from: u32) -> i32 {
        from as i32
    }
}
#[derive(Mapper)]
#[from(FirstSource)]
struct Destination {
    #[mapper(use_fns = [utils::from_u64_to_i32])]
    id: i32,
    name: String,
}

fn main() {
    let source = FirstSource {
        id: 12,
        name: "Chuck Norris".to_string(),
    };
    let _dest = Destination::from(source);
    println!("name: {}", _dest.name);
}

```

For More complex scenarios, checkout the **examples** and **tests** directories.

## Caveats/Pending

- Currently, Only Named Structs are Supported right now.

- The fields in the source needs to be `public` right now.

- No Generic Types are supported right now.

- Use of structs with LifeTimes is a bit tedious and non-idiomatic.
