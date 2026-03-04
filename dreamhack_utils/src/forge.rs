use std::{path::Path, process::Command};
use alloy::sol_types::SolConstructor;
use eyre::{bail, Context, Result};

pub fn build_contract_bytecode<C: SolConstructor>(
    project_root: &Path,
    file_name: &str,
    contract_name: &str,
    constructor_args: C,
) -> Result<Vec<u8>> {
    let bytecode = inspect_bytecode_bytes(project_root, file_name, contract_name)?;
    let constructor_args = constructor_args.abi_encode();

    let mut deploy_input = Vec::with_capacity(bytecode.len() + constructor_args.len());
    deploy_input.extend_from_slice(&bytecode);
    deploy_input.extend_from_slice(&constructor_args);
    Ok(deploy_input)
}

fn inspect_bytecode_bytes(
    project_root: &Path,
    source_path: &str,
    contract_name: &str,
) -> Result<Vec<u8>> {
    let contract_id = format!("{source_path}:{contract_name}");
    let output = Command::new("forge")
        .arg("inspect")
        .arg("--root")
        .arg(project_root)
        .arg("--contracts")
        .arg(project_root)
        .arg(&contract_id)
        .arg("bytecode")
        .output()
        .wrap_err_with(|| format!("failed to run `forge inspect` for `{contract_id}`"))?;

    if !output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_owned();
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_owned();
        bail!("`forge inspect` failed for `{contract_id}`.\nstdout:\n{stdout}\nstderr:\n{stderr}");
    }

    let bytecode = String::from_utf8(output.stdout)
        .wrap_err_with(|| format!("`forge inspect` output for `{contract_id}` is not UTF-8"))?;
    let bytecode = bytecode.trim().to_owned();
    validate_prefixed_hex(&bytecode, &contract_id)?;
    decode_prefixed_hex(&bytecode)
}

fn validate_prefixed_hex(value: &str, contract_id: &str) -> Result<()> {
    if value.len() <= 2 || !value.starts_with("0x") {
        bail!("invalid bytecode for `{contract_id}`: expected 0x-prefixed hex");
    }

    let hex_part = &value[2..];
    if hex_part.len() % 2 != 0 || !hex_part.chars().all(|c| c.is_ascii_hexdigit()) {
        bail!("invalid bytecode for `{contract_id}`: expected valid hex bytes");
    }

    Ok(())
}

fn decode_prefixed_hex(value: &str) -> Result<Vec<u8>> {
    let hex_part = value.trim_start_matches("0x");
    let mut bytes = Vec::with_capacity(hex_part.len() / 2);

    for i in (0..hex_part.len()).step_by(2) {
        let byte =
            u8::from_str_radix(&hex_part[i..i + 2], 16).wrap_err("failed to decode bytecode")?;
        bytes.push(byte);
    }

    Ok(bytes)
}
