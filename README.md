## LiveJQ

`livejq` is JSON parser like `jq` but is designed to work in continuous input without crashing on invalid JSON

#### User Case

1. When you have a program that is printing logs which may have other formats in between like text along with JSON, and you want to parse JSON for better readability. You can use `livejq` to parse JSON without crashing on other formats.

It works on one assumption that the JSON is a single string like `'{ "name": "Kunal Singh", "age": 21 }'`.

> This is a limitation, but will be fixed in future

### Demo

https://github.com/KunalSin9h/livejq/assets/82411321/71907858-5150-4efe-8c0f-58bb1c0dc591

### Install

Install using `cargo`

```bash
cargo install livejq
```

#### or you can find `binaries` in the [Release page](https://github.com/KunalSin9h/livejq/releases/latest)

### Usage

```bash
./my_program | livejq
```

> Here `|` is for piping output of `my_program` into `livejq` as input.
