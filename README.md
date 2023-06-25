# gokz.rs

A rust library for working with anything [GOKZ](https://github.com/KZGlobalTeam/gokz)!

This crate features basic types I didn't want to write yourself over and over again for every
project, as well as some easy to use wrapper functions for calling popular APIs such as the
[GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2).

## Features

By default, only some types are included. API wrappers and integrations with popular crates such as
[chrono](https://docs.rs/chrono) and [serde](https://docs.rs/serde) are hidden behind
[feature flags](https://doc.rust-lang.org/cargo/reference/features.html).

