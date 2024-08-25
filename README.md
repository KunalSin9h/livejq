## LiveJQ

`livejq` is JSON parser like `jq` but is designed to work in continuous input without crashing on invalid JSON. With json filtering.

It uses `livejq.toml` file to specify [filter rules](#filter).

#### User Case

When you have a program that is printing logs which may have other formats in between like text along with JSON, and you want to parse JSON for better readability. You can use `livejq` to parse JSON without crashing on other formats.

Or when you want to apply `filters` when paring json.

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
./your_program | livejq
```

### Filter

To apply filtering, you need to create `livejq.toml` file in the project root.

It contains `labels`. _labels_ are filter labels which you can apply with `-f` / `--filter` flag.

#### Example config file:

> when not `label` is created, `default` is used. For each label, you can only give allow or disallow, not both.

```toml
#livejq.toml

allow = ["name"] # default

[network] # -f network
allow = ["net-failed"]

[memory] # -f memory
allow = ["memory-info"]

[not-console] # -f not-console
disallow = ["console"]
```

> for seeing the schema format, check `schema.toml` file in this repository.

#### Usage:

```bash
# If no flag is given, it will use default 
# i.e allow = ["name"]
# it will only allow json who have "name" key
node main.js | livejq

# you can combine different labels together
node main.js | livejq -f network memory

# or using example_data.txt from this repository
cat example_data.txt | livejq --filter not-console
```

> Here `|` is for piping output of `my_program` into `livejq` as input.
