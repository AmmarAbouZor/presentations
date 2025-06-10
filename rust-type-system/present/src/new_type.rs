// Bad:

mod bad {

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

    fn example() {
        let distance_meter = 10.0;
        let time_seconds = 20.0;
        let speed = calc_speed(distance_meter, time_seconds);
    }
}

mod change {

    /// Calculate the speed depending on distance and time.
    ///
    /// * `distance`: distance in millimeter.
    /// * `time`: time in milliseconds.
    ///
    /// # Return:
    /// Speed as meter per second.
    fn calc_speed(distance: f32, time: f32) -> f32 {
        (distance / 1000.0) / (time / 1000.0)
    }

    fn example() {
        let distance_meter = 10.0;
        let time_seconds = 20.0;
        // No Errors!
        let speed = calc_speed(distance_meter, time_seconds);
    }
}

// Good:

mod good {

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

        fn meter_per_second(&self) -> f32 {
            self.distance.0 / self.time.0
        }

        fn kilometer_per_hour(&self) -> f32 {
            (self.distance.0 / 1000.0) / (self.time.0 / 3600.0)
        }
    }

    fn uses() {
        let distance = Meter(10.0);
        let time = Second(20.0);
        let meter_per_second = Speed::new(distance, time).meter_per_second();
    }

    // Pitfalls:

    impl From<f32> for Meter {
        fn from(value: f32) -> Self {
            Self(value)
        }
    }

    impl From<f32> for Second {
        fn from(value: f32) -> Self {
            Self(value)
        }
    }

    fn use_pitfall() {
        let distance_meter = 10.0;
        let time_seconds = 20.0;
        let meter_per_second =
            Speed::new(distance_meter.into(), time_seconds.into()).meter_per_second();
    }
}

mod pitfall {

    struct Kilometer(f32);
    struct Hour(f32);
    struct Speed {
        distance: Kilometer,
        time: Hour,
    }

    impl From<f32> for Kilometer {
        fn from(value: f32) -> Self {
            Self(value)
        }
    }

    impl From<f32> for Hour {
        fn from(value: f32) -> Self {
            Self(value)
        }
    }

    impl Speed {
        fn new(distance: Kilometer, time: Hour) -> Self {
            Self { distance, time }
        }
    }

    fn use_pitfall() {
        let distance_meter = 10.0;
        let time_seconds = 20.0;
        // No compiler error.
        let meter_per_second = Speed::new(distance_meter.into(), time_seconds.into());
    }
}

mod generics {

    struct Kilometer(f32);
    struct Meter(f32);

    struct Hour(f32);
    struct Second(f32);

    struct Speed<Dist, Time> {
        distance: Dist,
        time: Time,
    }

    impl<Dist, Time> Speed<Dist, Time> {
        fn new(distance: Dist, time: Time) -> Self {
            Self { distance, time }
        }
    }

    fn use_pitfall() {
        let distance_meter = Kilometer(10.0);
        let time_seconds = Hour(20.0);
        let meter_per_second = Speed::new(distance_meter, time_seconds);
    }
}
