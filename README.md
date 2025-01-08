# blog.otterbuild.com

This repository contains the source code for Otterbuild's blog, which can be
found at [blog.otterbuild.com]. The blog is built with `otter`, the framework
for web and mobile applications that Otterbuild is developing.

## Development

To work on the blog, you need to have a recent version of [Rust] installed.
Check out <https://www.rust-lang.org/tools/install> for instructions on how to
install Rust.

The project also makes use of pre-commit hooks to ensure that the code follows
a consistent style and is free of common bugs. To install the pre-commit hooks,
first install [pre-commit] and then run the following command:

```shell
pre-commit install
```

The checks that the pre-commit hooks run will also be executed for each pull
request before it can be merged. To check that your code passes these checks,
you can run all of them locally with [Earthly]:

```shell
earthly +all
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE)
  or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT)
  or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

[blog.otterbuild.com]: https://blog.otterbuild.com

[earthly]: https://earthly.dev

[pre-commit]: https://pre-commit.com

[rust]: https://www.rust-lang.org
