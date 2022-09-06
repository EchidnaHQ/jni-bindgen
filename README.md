# arabica
Bindings generator for JNI libraries for the Rust programming language!!!

This a fork is based on `jni-bindgen`. The difference betweent this project and upstream is because this fork uses [Robusta](https://github.com/waylovely-project/robusta) and the [jni-rs](https://github.com/waylovely-project/jni-rs/) crate, unlike upstream which uses its own solution!!

## How to use
After cloning this repository, run:
```sh
$ cargo install --path .
```

After that you can go to a folder with `jni-bindgen.toml` in it and run

```sh
$ jni-bindgen
```
