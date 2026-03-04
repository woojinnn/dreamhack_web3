use std::{
    path::{Path, PathBuf},
    process::Command,
};

use eyre::{bail, eyre, Context, Result};

#[derive(Debug, Clone, Copy)]
pub struct ForgeBytecodeSpec<'a> {
    pub source_path: &'a str,
    pub contract_name: &'a str,
    pub env_var: &'a str,
}

#[derive(Debug, Clone, Copy)]
pub struct ForgeEmitConfig<'a> {
    pub project_root: &'a Path,
    pub contracts_dir: &'a Path,
}

pub fn emit_bytecodes_to_env(config: ForgeEmitConfig<'_>, specs: &[ForgeBytecodeSpec<'_>]) -> Result<()> {
    println!("cargo:rerun-if-env-changed=PATH");

    let out_dir = std::env::var_os("OUT_DIR").ok_or_else(|| eyre!("OUT_DIR is not set"))?;
    let out_path = PathBuf::from(&out_dir).join("forge-out");
    let cache_path = PathBuf::from(&out_dir).join("forge-cache");

    for spec in specs {
        let source_full_path = config.project_root.join(spec.source_path);
        println!("cargo:rerun-if-changed={}", source_full_path.display());

        let bytecode = inspect_bytecode(config, spec, &out_path, &cache_path)?;
        println!("cargo:rustc-env={}={}", spec.env_var, bytecode);
    }

    Ok(())
}

fn inspect_bytecode(
    config: ForgeEmitConfig<'_>,
    spec: &ForgeBytecodeSpec<'_>,
    out_path: &Path,
    cache_path: &Path,
) -> Result<String> {
    let contract_id = format!("{}:{}", spec.source_path, spec.contract_name);
    let output = Command::new("forge")
        .arg("inspect")
        .arg("--root")
        .arg(config.project_root)
        .arg("--contracts")
        .arg(config.contracts_dir)
        .arg("--out")
        .arg(out_path)
        .arg("--cache-path")
        .arg(cache_path)
        .arg(&contract_id)
        .arg("bytecode")
        .output()
        .wrap_err_with(|| format!("failed to run `forge inspect` for `{contract_id}`"))?;

    if !output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_owned();
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_owned();
        bail!(
            "`forge inspect` failed for `{contract_id}`.\nstdout:\n{stdout}\nstderr:\n{stderr}"
        );
    }

    let bytecode = String::from_utf8(output.stdout)
        .wrap_err_with(|| format!("`forge inspect` output for `{contract_id}` is not UTF-8"))?;
    let bytecode = bytecode.trim().to_owned();

    validate_prefixed_hex(&bytecode, spec.env_var)?;
    Ok(bytecode)
}

fn validate_prefixed_hex(value: &str, env_var: &str) -> Result<()> {
    if value.len() <= 2 || !value.starts_with("0x") {
        bail!("invalid bytecode for `{env_var}`: expected 0x-prefixed hex");
    }

    let hex_part = &value[2..];
    if hex_part.len() % 2 != 0 || !hex_part.chars().all(|c| c.is_ascii_hexdigit()) {
        bail!("invalid bytecode for `{env_var}`: expected valid hex bytes");
    }

    Ok(())
}
