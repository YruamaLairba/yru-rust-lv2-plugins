# yru-rust-lv2-plugin

Personal plugin realisation using the
[rust-lv2](https://github.com/RustAudio/rust-lv2) framework

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

## Install

 - `cargo xtask install` to build with optimisation and install all
   plugins.
 - `cargo xtask build -p <plugin>` to build a particular plugin with
   optimisation and install it.
 - **don't forget the `--release` flag, to enable optimisation!**
 - use `--install-dir` option to change the installation path. The default
   installation destination is the [LV2 user specific standard
path](https://lv2plug.in/pages/filesystem-hierarchy-standard.html).

Don't forget the `--release` flag, to enable optimisation!

## Note about xtask
_xtask_ is a custom embedded script written to handle post-build actions.
Sometimes, this script may fail with some particular cargo configuration.
Typical error is file not found. Feel free to fill an issue and describe cargo
configuration if it happen.

## License

These plugins are released under the GPL V3 license, see the joined file
[gpl-3.0.txt](gpl-3.0.txt) for a copy.

