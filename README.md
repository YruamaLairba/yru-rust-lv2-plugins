# yru-rust-lv2-plugin

Personal plugin realisation using the
[rust-lv2](https://github.com/RustAudio/rust-lv2) framework

## Warning

At this time, these plugins are subject to change and can break backward
compatibility. It mean you shouldn't use them in a project since updating these
plugins can break you project.

## Building

Building for the first time require an internet connection.

 - `cargo xtask build` to build all plugins.
 - `cargo xtask build -p <plugin>` to build a particular plugin.
 - add `--release` flag to build in release mode.

Be patient, building for the first time is very long.

Built plugins are placed in the lv2 folder inside the cargo output
directory. For example if you used `cargo xtask build`, plugin are placed by
default in `target/debug/lv2`. See [cargo
documentation](https://doc.rust-lang.org/cargo/) for more details.

**Note about xtask**: it's a custom embedded script allowing to handle
post-build actions. This script may fail with some particular cargo
configuration (typically,file not found). Feel free to fill an issue if it
happen

## License

These plugins are released under the GPL V3 license, see the joined file
[gpl-3.0.txt](gpl-3.0.txt) for a copy.

