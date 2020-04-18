# Roll-Server

This is a simple Rocket application that rolls dice for role-playing games. It is meant to be used as a teaching tool for learning Rocket.

If you run the server and visit `localhost:8000/roll/<number>d<size>` it will roll `number` dice of size `size`, where `size` may be 4, 6, 8, 10, 12, 20, or 100 (the common RPG dice sizes). The maximum number of dice that may be rolled is 255, which is chosen mostly arbitrarily.

For example, if you visit `/roll/4d6` the output may be
```text
2 + 5 + 4 + 3 = 14
```

If you visit `/roll/crit/<dice>` it will roll critical hit damage equal to a normal dice roll plus a dice roll at maximum damage.

Using `/roll/crit/4d6`:
```text
1 + 1 + 5 + 5 + 24 = 36
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.