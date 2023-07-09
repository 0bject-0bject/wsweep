# WSWEEP

# Description ❤
A simple tool to clear module and target directories from your projects!

# Screenshots 📸

![Screenshot](https://github.com/0bject-0bject/wsweep/blob/main/screenshots/wsweep.png)

# Installation 📥

<details>

<summary> <strong>Cargo Install </strong> </summary>

```bash
cargo install wsweep
``` 

</details>

<details>

<summary> <strong>From Source 🔧</strong> </summary>

```bash
git clone https://github.com/0bject-0bject/wsweep.git
```

```bash
cd wsweep
```

```bash
cargo build --release
```

```bash
./target/release/wsweep
```

</details>

# Usage 📝

```bash
wsweep [options]
```

# Arguments 📝

| Argument | Description | Example | Required | Default |
| :---: | --- | --- | ---: | ---: |
| --dry-run | Target directory to clear | `wsweep --path <path> --dry-run` | No | target |
| --age | Minimum age for file to be deleted. (DAYS) | `wsweep --path <path> --age 1` | No | target |
| --verbose | Verbose ouptput | `wsweep --path <path> --verbose` | No | target |
| -h, --help | Prints help information | `wsweep -h` | No | No |
| -v, --version | Prints version information | `wsweep -v` | No | No |
| -p, --path | Path to directory | `wsweep -p C:\Users\user\projects` | No | No |


# Examples 📚

```bash
wbuster --path C:\Users\user\projects --age 1 --verbose
```

```bash
wbuster --path C:\Users\user\projects
```

# License 📜

Licensed under the MIT license ([LICENSE](https://github.com/0bject-0bject/wbuster/blob/main/LICENSE))