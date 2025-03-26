# InputEventsReplay

This is a simple rust cli application that records your keyboard and mouse inputs and then replace them back. This is useful when you have a repetitive task on your computer that can be solved with the same action over and over.

This uses the Rust Crate [rdev](https://docs.rs/rdev/latest/rdev/), which means that the available platforms that this program can run on is dependent on the platforms available to rdev, specifically Windows, Linux X11, MacOS.
