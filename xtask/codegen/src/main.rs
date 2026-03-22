mod batch;
mod online;

use std::{env, path::PathBuf};

fn main() {
    let mut family = None;
    let mut sign = None;

    for arg in env::args().skip(1) {
        match arg.as_str() {
            "--batch" => family = Some(Family::Batch),
            "--online" => family = Some(Family::Online),
            "--unsigned" => sign = Some(Sign::Unsigned),
            "--signed" => sign = Some(Sign::Signed),
            other => {
                panic!(
                    "unsupported arg: {other}, expected flags from: \
                     `--batch`, `--online`, `--unsigned`, `--signed`"
                )
            }
        }
    }

    let family =
        family.unwrap_or_else(|| panic!("missing family, expected `--batch` or `--online`"));
    let sign = sign.unwrap_or_else(|| panic!("missing sign, expected `--unsigned` or `--signed`"));

    let root = workspace_root();
    let src = root.join("src");

    match (family, sign) {
        (Family::Batch, Sign::Unsigned) => {
            batch::generate_unsigned(&src);
            batch::write_batch_mod(&src);
        }
        (Family::Batch, Sign::Signed) => {
            batch::generate_signed(&src);
            batch::write_batch_mod(&src);
        }
        (Family::Online, Sign::Unsigned) => online::generate_unsigned(&src),
        (Family::Online, Sign::Signed) => online::generate_signed(&src),
    }
}

#[derive(Clone, Copy, Debug)]
enum Family {
    Batch,
    Online,
}

#[derive(Clone, Copy, Debug)]
enum Sign {
    Unsigned,
    Signed,
}

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
}
