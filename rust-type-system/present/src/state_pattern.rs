mod bad {

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum PlaneState {
        OnGround,
        InFlight,
    }

    struct Plane {
        state: PlaneState,
        seats_count: u16,
    }

    impl Plane {
        fn new(seats: u16) -> Self {
            Self {
                state: PlaneState::OnGround,
                seats_count: seats,
            }
        }

        fn take_off(&mut self) {
            assert_eq!(
                self.state,
                PlaneState::OnGround,
                "Plan must be on ground before taking off"
            );
            // ... //
        }

        fn land(&mut self) {
            assert_eq!(
                self.state,
                PlaneState::InFlight,
                "Plan must be in flight before landing"
            );
            // ... //
        }

        fn get_seats_count(&self) -> u16 {
            self.seats_count
        }
    }

    fn example() {
        let mut plane = Plane::new(16);
        // Runtime error. Our goal to make it compile time error.
        plane.land();
    }
}

mod good {

    use std::marker::PhantomData;

    struct OnGround;
    struct InFlight;

    struct Plane<T> {
        state: PhantomData<T>,
        seats_count: u16,
    }

    impl<T> Plane<T> {
        fn get_seats_count(&self) -> u16 {
            self.seats_count
        }
    }

    impl Plane<OnGround> {
        fn new(seats: u16) -> Plane<OnGround> {
            Plane {
                state: PhantomData::<OnGround>,
                seats_count: seats,
            }
        }

        #[must_use]
        fn take_off(self) -> Plane<InFlight> {
            Plane {
                state: PhantomData::<InFlight>,
                seats_count: self.seats_count,
            }
        }
    }

    impl Plane<InFlight> {
        #[must_use]
        fn land(self) -> Plane<OnGround> {
            Plane {
                state: PhantomData::<OnGround>,
                seats_count: self.seats_count,
            }
        }
    }

    fn example() {
        let plane: Plane<OnGround> = Plane::new(16);

        // Compiler error. Yay!
        // plane.land();

        let plane: Plane<InFlight> = plane.take_off();

        // General methods can be always called.
        let _ = plane.get_seats_count();

        // Compiler error
        // plane.take_off();

        // Now `land()` can called.
        let plane: Plane<OnGround> = plane.land();

        // General methods can be always called.
        let _ = plane.get_seats_count();
    }
}
