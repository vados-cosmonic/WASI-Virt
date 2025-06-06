use std::sync::OnceLock;

use anyhow::Result;
use semver::Version;
use walrus::{FuncParams, FuncResults, Module, ValType};

use super::replace_or_insert_stub_for_exports;
use crate::WITInterfaceNameParts;

/// Functions that represent the environment functionality provided by WASI randoms
static WASI_RANDOM_FNS: OnceLock<Vec<(WITInterfaceNameParts, FuncParams, FuncResults)>> =
    OnceLock::new();

/// Retrieve or initialize the static list of functions related to randomness in WASI
fn get_wasi_random_fns() -> &'static Vec<(WITInterfaceNameParts, FuncParams, FuncResults)> {
    WASI_RANDOM_FNS.get_or_init(|| {
        Vec::from([
            (
                &("wasi", "random", "random", "get-random-bytes"),
                vec![ValType::I64],
                vec![ValType::I32],
            ),
            (
                &("cabi_post_wasi", "random", "random", "get-random-bytes"),
                vec![ValType::I32],
                vec![],
            ),
            (
                &("wasi", "random", "random", "get-random-u64"),
                vec![],
                vec![ValType::I64],
            ),
            (
                &("wasi", "random", "insecure", "get-insecure-random-bytes"),
                vec![ValType::I64],
                vec![ValType::I32],
            ),
            (
                &(
                    "cabi_post_wasi",
                    "random",
                    "insecure",
                    "get-insecure-random-bytes",
                ),
                vec![ValType::I32],
                vec![],
            ),
            (
                &("wasi", "random", "insecure", "get-insecure-random-u64"),
                vec![],
                vec![ValType::I64],
            ),
            (
                &("wasi", "random", "insecure-seed", "insecure-seed"),
                vec![],
                vec![ValType::I32],
            ),
        ])
    })
}

/// Replace exports related to randomness in WASI to deny access
///
/// # Arguments
///
/// * `module` - The module to deny
/// * `insert_wasi_version` - version of WASI to use when inserting stubs
///
pub(crate) fn deny_random_virt(module: &mut Module, insert_wasi_version: &Version) -> Result<()> {
    replace_or_insert_stub_for_exports(module, get_wasi_random_fns(), insert_wasi_version)
}
