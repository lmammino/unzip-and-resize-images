# unzip-and-resize-images

A simple rust-based CLI that can read images from a zip file, resize them and save the resulting images in a local folder.

This is a simple project that was built to explore the crates [zip](https://crates.io/crates/zip) and [image](https://crates.io/crates/image). The built of this simple tool was [live-streamed on Twitch](https://twitch.tv/loige) (and you can find the recording on [YouTube](https://youtube.com/loige)).

If you need to generate a sample zip file with a bunch of images you can use [unsample.net](https://unsample.net/).


## Usage

```bash
cargo run -- <filename> [size]
```

or if you have compiled your own binary:


```bash
unzippy <filename> [size]
```

### Examples

Extract all the images from `myfiles.zip` and create thumbnails of 300x300 size:

```bash
cargo run -- myfiles.zip 300
```

Extract all the images from `myfiles.zip` and create thumbnails of default size (400x400):

```bash
cargo run -- myfiles.zip
```

> **Note**: you will need to have a `output` folder created in your path (the tool doesn't do that for us right now).


## Build

```bash
cargo build --release
```

Your compiled binary will be available in `target/release` (`unzippy` or `unzippy.exe` depending on your OS).

## Contributing

Everyone is very welcome to contribute to this project.
You can contribute just by submitting bugs or suggesting improvements by
[opening an issue on GitHub](https://github.com/lmammino/unzip-and-resize-images/issues).


## License

Licensed under [MIT License](LICENSE). © Luciano Mammino.