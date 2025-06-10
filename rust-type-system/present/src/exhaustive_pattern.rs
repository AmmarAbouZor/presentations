use std::path::PathBuf;
mod bad {
    use super::*;

    struct CliArgs {
        input: PathBuf,
        interval: u64,
        separtor: String,
        custom_name: Option<String>,
    }

    impl CliArgs {
        fn validate(&self) -> Result<(), &'static str> {
            if !self.input.exists() {
                return Err("Input doesn't exist");
            }

            if self.interval == 0 {
                return Err("Interval must be positive");
            }

            // What happen if we extend `CliArgs` with new fields?

            return Ok(());
        }
    }

    struct Person {
        first_name: String,
        last_name: String,
        age: u16,
        height: f32,
    }

    impl PartialEq for Person {
        fn eq(&self, other: &Self) -> bool {
            // Comparing names is enough for now but what will happen when
            // `middle_name` is introduced
            self.first_name == other.first_name && self.last_name == other.last_name
        }
    }

    //
}

mod good {
    use super::*;

    struct CliArgs {
        input: PathBuf,
        interval: u64,
        separtor: String,
        custom_name: Option<String>,
    }

    impl CliArgs {
        fn validate(&self) -> Result<(), &'static str> {
            // We will get compiler error her on new types to remind
            // us to consider the new field
            let Self {
                input,
                interval,
                separtor: _,
                custom_name: _,
            } = self;

            if !input.exists() {
                return Err("Input doesn't exist");
            }

            if *interval == 0 {
                return Err("Interval must be positive");
            }

            return Ok(());
        }
    }

    struct Person {
        first_name: String,
        last_name: String,
        age: u16,
        height: f32,
    }

    impl PartialEq for Person {
        fn eq(&self, other: &Self) -> bool {
            // We will get compiler error her on new types to remind
            // us to consider the new field
            let Self {
                first_name,
                last_name,
                age: _,
                height: _,
            } = self;

            first_name == &other.first_name && last_name == &other.last_name
        }
    }

    //
}

mod enumss {

    #[derive(Debug, Clone, Copy, Eq, PartialEq)]
    enum Kinds {
        First,
        Second,
        Third,
    }

    impl Kinds {
        fn all() -> &'static [Kinds] {
            // Reminder to add new kinds below
            match Kinds::First {
                Kinds::First => {}
                Kinds::Second => {}
                Kinds::Third => {}
            }

            &[Kinds::First, Kinds::Second, Kinds::Third]
        }
    }

    #[test]
    fn test_all() {
        let all = Kinds::all();
        // This will pass if we forget to extend all function with the new item
        assert_eq!(all, &[Kinds::First, Kinds::Second, Kinds::Third])
    }
}

mod pitfalls {
    use super::*;

    struct CliArgs {
        input: PathBuf,
        interval: u64,
        separtor: String,
        custom_name: Option<String>,
    }

    struct Person {
        first_name: String,
        last_name: String,
        age: u16,
        height: f32,
    }

    impl CliArgs {
        fn validate(&self) -> Result<(), &'static str> {
            // With this wild card matching we have lost
            // the precious compiler error.
            let Self {
                input, interval, ..
            } = self;

            if !input.exists() {
                return Err("Input doesn't exist");
            }

            if *interval == 0 {
                return Err("Interval must be positive");
            }

            return Ok(());
        }
    }

    impl PartialEq for Person {
        fn eq(&self, other: &Self) -> bool {
            // With this wild card matching we have lost
            // the precious compiler error.
            let Self {
                first_name,
                last_name,
                ..
            } = self;

            first_name == &other.first_name && last_name == &other.last_name
        }
    }

    //
}

mod enumss {

    #[derive(Debug, Clone, Copy, Eq, PartialEq)]
    enum Kinds {
        First,
        Second,
        Third,
    }

    impl Kinds {
        fn all() -> &'static [Kinds] {
            // Reminder to add new kinds below
            match Kinds::First {
                Kinds::First => {}
                Kinds::Second => {}
                Kinds::Third => {}
            }

            &[Kinds::First, Kinds::Second, Kinds::Third]
        }
    }

    #[test]
    fn test_all() {
        let all = Kinds::all();
        // This will pass if we forget to extend all function with the new item
        assert_eq!(all, &[Kinds::First, Kinds::Second, Kinds::Third])
    }
}
