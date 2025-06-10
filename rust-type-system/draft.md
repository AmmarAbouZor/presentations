The talk will go in three major sections:

# Using the compiler and type-system:

## Using new-type idiom

* Will be quick recap since we've talked about it in the last meeting.
* New examples like the one giving the units of the numbers.

### Bad:
```rust
/// Calculate the speed depending on distance and time.
///
/// * `distance`: distance in meter.
/// * `time`: time in seconds.
///
/// # Return:
/// Speed as meter per second.
fn calc_speed(distance: f32, time: f32) -> f32 {
    distance / time
}
```

### Good: 
```rust
struct Meter(f32);
struct Second(f32);
struct Speed {
    distance: Meter,
    time: Second,
}

impl Speed {
    fn new(distance: Meter, time: Second) -> Self {
        Self { distance, time }
    }

    #[inline]
    fn meter_per_second(&self) -> f32 {
        self.distance.0 / self.time.0
    }

    #[inline]
    fn kilometer_per_hour(&self) -> f32 {
        (self.distance.0 / 1000.0) / (self.time.0 / 3600.0)
    }
}
```

## Favor Enums over Booleans:
I can start with a function having two booleans as arguments and then show the change by using enums instead.
Then I can show the case were we can even bake the struct into the enum itself.


## Group arguments with enums:
We see function that gets multiple arguments that can be grouped together with enums to get better APIs ensuring code correctness.

### Bad:
```rust
fn entries_count(main_db: DatabaseService, cache: CacheSerivce, use_cache: bool) -> usize {
    if use_cache {
       return cache.count()
    }

    //... Query database to get count 
}
```

### Still bad:

```rust
fn entries_count(
    main_db: Option<DatabaseService>,
    cache: Option<CachSerivce>,
    use_cache: bool,
) -> usize {
    if use_cache {
        return cache.expect("cach must provided when used cache").count();
    }

    let main_db = main_db.expect("Database must be provided when no cache is used.");

    //... Query database to get count
}
```

### Good
```rust
enum DataSource {
    Database(DatabaseService),
    Cache(CacheSerivce),
}

fn entries_count(source: DataSource) -> usize {
    match source{
        DataSource::Database(db) => //.. Query data base ,
        DataSource::Cache(cache) => //.. Query cache,
    }
}
```

## The state pattern.

### Bad:
```rust
enum PlaneState {
    OnGround,
    InFlight,
}

struct Plane {
    state: PlaneState
    ...
}

impl Plane {
    fn take_off(&mut self) {
        // What will happen if this is called during flight?
    }

    fn land(&mut self) {
        // What will happen if this is called while plane on ground?
    }
}
```

### Still bad:
```rust

impl Plane {
    fn take_off(&mut self) {
        assert_eq!(self.state, PlaneState::OnGround, "Plan must be on ground before taking off");
        // ...
    }

    fn land(&mut self) {
        assert_eq!(self.state, PlaneState::InFlight, "Plan must be in flight before landing");
        // ...
    }
}
```

### Good:
```rust
use std::marker::PhantomData;

struct OnGround;
struct InFlight;

struct Plane<T> {
    state: PhantomData<T>,
}

// General functions
impl<T> Plane<T> {
    fn engine_name(&self) -> &str {
        //...
    }
}

impl Plane<OnGround> {
    fn take_off(self) -> Plane<InFlight> {
        // .. take off
        Plane {
            state: PhantomData::<InFlight>,
        }
    }
}

impl Plane<InFlight> {
    fn land(self) -> Plane<OnGround> {
        // ... land
        Plane {
            state: PhantomData::<OnGround>,
        }
    }
}
```

## Exhaustive pattern matching

### Modelling the code with enums.
Modelling the code with enums to use exhaustive pattern matching to get compiler assistant when extending the code in the future.
Live example and coding session using the project `build cli tool` in chipmunk.

### Trick: Compiler error on changes in data structure:

#### New fields on structs -> Compiler error:

```rust
struct Data {
    name: String,
    length: usize,
}

fn valida(data: &Data) {
    let Data { name, length } = data;
    //.. Validate
}
```
#### New option on enums -> Compiler error:
What will happen when extend the enum with new option?
```rust
enum Var {
    Foo1,
    Foo2,
    Foo3,
    Bar,
}

impl Var {
    fn all() -> &'static [Var] {
        &[Var::Foo1, Var::Foo2, Var::Foo3, Var::Bar]
    }

    fn foo_related() -> &'static [Var] {
        &[Var::Foo1, Var::Foo2, Var::Foo3]
    }
}
```

Trick to get compiler errors on changes:
```rust
enum Var {
    Foo1,
    Foo2,
    Foo3,
    Bar,
}

impl Var {
    fn all() -> &'static [Var] {
        // Reminder to add the new enum here.
        _ = match Var::Foo1 {
            Var::Foo1 => {}
            Var::Foo2 => {}
            Var::Foo3 => {}
            Var::Bar => {}
        };

        &[Var::Foo1, Var::Foo2, Var::Foo3, Var::Bar]
    }

    fn foo_related() -> &'static [Var] {
        // Reminder to consider adding the new enum here.
        _ = match Var::Foo1 {
            Var::Foo1 => {}
            Var::Foo2 => {}
            Var::Foo3 => {}
            Var::Bar => {}
        };
        &[Var::Foo1, Var::Foo2, Var::Foo3]
    }
}
```



-------------------------------------------------------------------------------------

# Testing:

## Snapshot testing:

* Short recap since we talked about it previously.
* Example from Chipmunk from where it has been used.

## Fuzzy testing & Prop testing.

* Talking about the concept in general and its benefits
* Talk about support in the eco-system 
  - `[cargo-fuzz](https://github.com/rust-fuzz/cargo-fuzz)`.
  - '[Proptest](https://github.com/proptest-rs/proptest)'
* Examples from Chipmunk with proptests.

-------------------------------------------------------------------------------------

# Invariant testing (Ensure correctness in production)

* Short recap since we talked about it previously.

