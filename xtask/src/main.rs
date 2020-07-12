use std::fs;
use xtask::*;

macro_rules! pkg_model {
    ( $name:literal, $bname:literal, $subs:expr , $copies:expr) => {{
        PackageConf {
            name: $name,
            post_build: |conf| {
                let built_bin_name = [&conf.lib_prefix(), $bname, &conf.lib_suffix()].concat();
                let lib_file_name = [&conf.lib_prefix(), $name, &conf.lib_suffix()].concat();
                let subs: &[(&str, &str)] = &[("@LIB_FILE_NAME@", &lib_file_name)];
                let src_dir = workspace_root().join($name);
                let out_dir = conf.build_dir().join("lv2").join($name);
                fs::create_dir_all(&out_dir).unwrap();
                for e in &$subs {
                    subst(src_dir.join(e), out_dir.join(e), subs).unwrap();
                }
                for e in &$copies {
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
                let src_dir = conf.build_dir().join("lv2").join($name);
                let dest_dir = conf.install_dir().join($name);
                copy_dir(src_dir, dest_dir)
            },
            uninstall: |conf| {
                let rm_dir = conf.install_dir().join($name);
                uninstall_dir(rm_dir).unwrap();
                Ok(())
            },
        }
    }};
}

const PACKAGES_CONF: &[PackageConf] = &[
    pkg_model!(
        "yru-echo-rs-mono",
        "yru_echo_rs_mono",
        ["manifest.ttl"],
        ["yru-echo-rs-mono.ttl"]
    ),
    pkg_model!(
        "yru-echo-rs-stereo",
        "yru_echo_rs_stereo",
        ["manifest.ttl"],
        ["yru-echo-rs-stereo.ttl"]
    ),
    pkg_model!(
        "yru-chorus-rs-mono",
        "yru_chorus_rs_mono",
        ["manifest.ttl"],
        ["yru-chorus-rs-mono.ttl"]
    ),
    pkg_model!(
        "yru-chorus-rs-stereo",
        "yru_chorus_rs_stereo",
        ["manifest.ttl"],
        ["yru-chorus-rs-stereo.ttl"]
    ),
    pkg_model!(
        "yru-flanger-rs-mono",
        "yru_flanger_rs_mono",
        ["manifest.ttl"],
        ["yru-flanger-rs-mono.ttl"]
    ),
    pkg_model!(
        "yru-flanger-rs-stereo",
        "yru_flanger_rs_stereo",
        ["manifest.ttl"],
        ["yru-flanger-rs-stereo.ttl"]
    ),
    pkg_model!(
        "yru-tremolo-rs-mono",
        "yru_tremolo_rs_mono",
        ["manifest.ttl"],
        ["yru-tremolo-rs-mono.ttl"]
    ),
    pkg_model!(
        "yru-tremolo-rs-stereo",
        "yru_tremolo_rs_stereo",
        ["manifest.ttl"],
        ["yru-tremolo-rs-stereo.ttl"]
    ),
];

fn main() {
    do_job(PACKAGES_CONF).unwrap();
}
