use std::fs;
use xtask::*;

const PACKAGES_CONF: &[PackageConf] = &[
    PackageConf {
        name: "yru-echo-rs-mono",
        post_build: |conf| {
            let built_bin_name =
                [&conf.lib_prefix(), "yru_echo_rs_mono", &conf.lib_suffix()].concat();
            let lib_file_name =
                [&conf.lib_prefix(), "yru-echo-rs-mono", &conf.lib_suffix()].concat();
            let subs: &[(&str, &str)] = &[("@LIB_FILE_NAME@", &lib_file_name)];
            let src_dir = workspace_root().join("yru-echo-rs-mono");
            let out_dir = conf.build_dir().join("lv2").join("yru-echo-rs-mono");
            fs::create_dir_all(&out_dir).unwrap();
            subst(
                src_dir.join("manifest.ttl"),
                out_dir.join("manifest.ttl"),
                subs,
            )
            .unwrap();
            for e in &["yru-echo-rs-mono.ttl"] {
                fs::copy(src_dir.join(e), out_dir.join(e)).unwrap();
            }
            fs::copy(
                conf.build_dir().join(&built_bin_name),
                out_dir.join(&lib_file_name),
            )
            .unwrap();
            Ok(())
        },
        install: |conf| {
            let src_dir = conf.build_dir().join("lv2").join("yru-echo-rs-mono");
            let dest_dir = conf.install_dir().join("yru-echo-rs-mono");
            copy_dir(src_dir, dest_dir)
        },
        uninstall: |conf| {
            let rm_dir = conf.install_dir().join("yru-echo-rs-mono");
            fs::remove_dir_all(rm_dir).unwrap();
            Ok(())
        },
    },
    PackageConf {
        name: "yru-echo-rs-stereo",
        post_build: |conf| {
            let built_bin_name =
                [&conf.lib_prefix(), "yru_echo_rs_stereo", &conf.lib_suffix()].concat();
            let lib_file_name =
                [&conf.lib_prefix(), "yru-echo-rs-stereo", &conf.lib_suffix()].concat();
            let subs: &[(&str, &str)] = &[("@LIB_FILE_NAME@", &lib_file_name)];
            let src_dir = workspace_root().join("yru-echo-rs-stereo");
            let out_dir = conf.build_dir().join("lv2").join("yru-echo-rs-stereo");
            fs::create_dir_all(&out_dir).unwrap();
            subst(
                src_dir.join("manifest.ttl"),
                out_dir.join("manifest.ttl"),
                subs,
            )
            .unwrap();
            for e in &["yru-echo-rs-stereo.ttl"] {
                fs::copy(src_dir.join(e), out_dir.join(e)).unwrap();
            }
            fs::copy(
                conf.build_dir().join(&built_bin_name),
                out_dir.join(&lib_file_name),
            )
            .unwrap();
            Ok(())
        },
        install: |conf| {
            let src_dir = conf.build_dir().join("lv2").join("yru-echo-rs-stereo");
            let dest_dir = conf.install_dir().join("yru-echo-rs-stereo");
            copy_dir(src_dir, dest_dir)
        },
        uninstall: |_| {
            todo!();
        },
    },
    PackageConf {
        name: "yru-chorus-rs-mono",
        post_build: |conf| {
            let built_bin_name =
                [&conf.lib_prefix(), "yru_chorus_rs_mono", &conf.lib_suffix()].concat();
            let lib_file_name =
                [&conf.lib_prefix(), "yru-chorus-rs-mono", &conf.lib_suffix()].concat();
            let subs: &[(&str, &str)] = &[("@LIB_FILE_NAME@", &lib_file_name)];
            let src_dir = workspace_root().join("yru-chorus-rs-mono");
            let out_dir = conf.build_dir().join("lv2").join("yru-chorus-rs-mono");
            fs::create_dir_all(&out_dir).unwrap();
            subst(
                src_dir.join("manifest.ttl"),
                out_dir.join("manifest.ttl"),
                subs,
            )
            .unwrap();
            for e in &["yru-chorus-rs-mono.ttl"] {
                fs::copy(src_dir.join(e), out_dir.join(e)).unwrap();
            }
            fs::copy(
                conf.build_dir().join(&built_bin_name),
                out_dir.join(&lib_file_name),
            )
            .unwrap();
            Ok(())
        },
        install: |_| {
            todo!();
        },
        uninstall: |_| {
            todo!();
        },
    },
    PackageConf {
        name: "yru-chorus-rs-stereo",
        post_build: |conf| {
            let built_bin_name = [
                &conf.lib_prefix(),
                "yru_chorus_rs_stereo",
                &conf.lib_suffix(),
            ]
            .concat();
            let lib_file_name = [
                &conf.lib_prefix(),
                "yru-chorus-rs-stereo",
                &conf.lib_suffix(),
            ]
            .concat();
            let subs: &[(&str, &str)] = &[("@LIB_FILE_NAME@", &lib_file_name)];
            let src_dir = workspace_root().join("yru-chorus-rs-stereo");
            let out_dir = conf.build_dir().join("lv2").join("yru-chorus-rs-stereo");
            fs::create_dir_all(&out_dir).unwrap();
            subst(
                src_dir.join("manifest.ttl"),
                out_dir.join("manifest.ttl"),
                subs,
            )
            .unwrap();
            for e in &["yru-chorus-rs-stereo.ttl"] {
                fs::copy(src_dir.join(e), out_dir.join(e)).unwrap();
            }
            fs::copy(
                conf.build_dir().join(&built_bin_name),
                out_dir.join(&lib_file_name),
            )
            .unwrap();
            Ok(())
        },
        install: |_| {
            todo!();
        },
        uninstall: |_| {
            todo!();
        },
    },
    PackageConf {
        name: "yru-flanger-rs-mono",
        post_build: |conf| {
            let built_bin_name = [
                &conf.lib_prefix(),
                "yru_flanger_rs_mono",
                &conf.lib_suffix(),
            ]
            .concat();
            let lib_file_name = [
                &conf.lib_prefix(),
                "yru-flanger-rs-mono",
                &conf.lib_suffix(),
            ]
            .concat();
            let subs: &[(&str, &str)] = &[("@LIB_FILE_NAME@", &lib_file_name)];
            let src_dir = workspace_root().join("yru-flanger-rs-mono");
            let out_dir = conf.build_dir().join("lv2").join("yru-flanger-rs-mono");
            fs::create_dir_all(&out_dir).unwrap();
            subst(
                src_dir.join("manifest.ttl"),
                out_dir.join("manifest.ttl"),
                subs,
            )
            .unwrap();
            for e in &["yru-flanger-rs-mono.ttl"] {
                fs::copy(src_dir.join(e), out_dir.join(e)).unwrap();
            }
            fs::copy(
                conf.build_dir().join(&built_bin_name),
                out_dir.join(&lib_file_name),
            )
            .unwrap();
            Ok(())
        },
        install: |_| {
            todo!();
        },
        uninstall: |_| {
            todo!();
        },
    },
    PackageConf {
        name: "yru-flanger-rs-stereo",
        post_build: |conf| {
            let built_bin_name = [
                &conf.lib_prefix(),
                "yru_flanger_rs_stereo",
                &conf.lib_suffix(),
            ]
            .concat();
            let lib_file_name = [
                &conf.lib_prefix(),
                "yru-flanger-rs-stereo",
                &conf.lib_suffix(),
            ]
            .concat();
            let subs: &[(&str, &str)] = &[("@LIB_FILE_NAME@", &lib_file_name)];
            let src_dir = workspace_root().join("yru-flanger-rs-stereo");
            let out_dir = conf.build_dir().join("lv2").join("yru-flanger-rs-stereo");
            fs::create_dir_all(&out_dir).unwrap();
            subst(
                src_dir.join("manifest.ttl"),
                out_dir.join("manifest.ttl"),
                subs,
            )
            .unwrap();
            for e in &["yru-flanger-rs-stereo.ttl"] {
                fs::copy(src_dir.join(e), out_dir.join(e)).unwrap();
            }
            fs::copy(
                conf.build_dir().join(&built_bin_name),
                out_dir.join(&lib_file_name),
            )
            .unwrap();
            Ok(())
        },
        install: |_| {
            todo!();
        },
        uninstall: |_| {
            todo!();
        },
    },
    PackageConf {
        name: "yru-tremolo-rs-mono",
        post_build: |conf| {
            let built_bin_name = [
                &conf.lib_prefix(),
                "yru_tremolo_rs_mono",
                &conf.lib_suffix(),
            ]
            .concat();
            let lib_file_name = [
                &conf.lib_prefix(),
                "yru-tremolo-rs-mono",
                &conf.lib_suffix(),
            ]
            .concat();
            let subs: &[(&str, &str)] = &[("@LIB_FILE_NAME@", &lib_file_name)];
            let src_dir = workspace_root().join("yru-tremolo-rs-mono");
            let out_dir = conf.build_dir().join("lv2").join("yru-tremolo-rs-mono");
            fs::create_dir_all(&out_dir).unwrap();
            subst(
                src_dir.join("manifest.ttl"),
                out_dir.join("manifest.ttl"),
                subs,
            )
            .unwrap();
            for e in &["yru-tremolo-rs-mono.ttl"] {
                fs::copy(src_dir.join(e), out_dir.join(e)).unwrap();
            }
            fs::copy(
                conf.build_dir().join(&built_bin_name),
                out_dir.join(&lib_file_name),
            )
            .unwrap();
            Ok(())
        },
        install: |_| {
            todo!();
        },
        uninstall: |_| {
            todo!();
        },
    },
    PackageConf {
        name: "yru-tremolo-rs-stereo",
        post_build: |conf| {
            let built_bin_name = [
                &conf.lib_prefix(),
                "yru_tremolo_rs_stereo",
                &conf.lib_suffix(),
            ]
            .concat();
            let lib_file_name = [
                &conf.lib_prefix(),
                "yru-tremolo-rs-stereo",
                &conf.lib_suffix(),
            ]
            .concat();
            let subs: &[(&str, &str)] = &[("@LIB_FILE_NAME@", &lib_file_name)];
            let src_dir = workspace_root().join("yru-tremolo-rs-stereo");
            let out_dir = conf.build_dir().join("lv2").join("yru-tremolo-rs-stereo");
            fs::create_dir_all(&out_dir).unwrap();
            subst(
                src_dir.join("manifest.ttl"),
                out_dir.join("manifest.ttl"),
                subs,
            )
            .unwrap();
            for e in &["yru-tremolo-rs-stereo.ttl"] {
                fs::copy(src_dir.join(e), out_dir.join(e)).unwrap();
            }
            fs::copy(
                conf.build_dir().join(&built_bin_name),
                out_dir.join(&lib_file_name),
            )
            .unwrap();
            Ok(())
        },
        install: |_| {
            todo!();
        },
        uninstall: |_| {
            todo!();
        },
    },
];

fn main() {
    do_job(PACKAGES_CONF).unwrap();
}
