use extism_pdk::*;
use proto_pdk::*;
use std::collections::HashMap;

static NAME: &str = "Archgate";

#[plugin_fn]
pub fn register_tool(Json(_): Json<RegisterToolInput>) -> FnResult<Json<RegisterToolOutput>> {
    Ok(Json(RegisterToolOutput {
        name: NAME.into(),
        type_of: PluginType::CommandLine,
        minimum_proto_version: Some(Version::new(0, 46, 0)),
        plugin_version: Version::parse(env!("CARGO_PKG_VERSION")).ok(),
        ..RegisterToolOutput::default()
    }))
}

#[plugin_fn]
pub fn load_versions(Json(_): Json<LoadVersionsInput>) -> FnResult<Json<LoadVersionsOutput>> {
    let tags = load_git_tags("https://github.com/archgate/cli")?
        .into_iter()
        .filter_map(|tag| tag.strip_prefix("v").map(|t| t.to_owned()))
        .collect::<Vec<_>>();

    Ok(Json(LoadVersionsOutput::from(tags)?))
}

#[plugin_fn]
pub fn download_prebuilt(
    Json(input): Json<DownloadPrebuiltInput>,
) -> FnResult<Json<DownloadPrebuiltOutput>> {
    let env = get_host_environment()?;

    check_supported_os_and_arch(
        NAME,
        &env,
        permutations! [
            HostOS::Linux => [HostArch::X64],
            HostOS::MacOS => [HostArch::Arm64],
            HostOS::Windows => [HostArch::X64],
        ],
    )?;

    let version = &input.context.version;

    let (os, arch) = match (&env.os, &env.arch) {
        (HostOS::MacOS, HostArch::Arm64) => ("darwin", "arm64"),
        (HostOS::Linux, HostArch::X64) => ("linux", "x64"),
        (HostOS::Windows, HostArch::X64) => ("win32", "x64"),
        _ => unreachable!(),
    };

    let ext = if env.os == HostOS::Windows {
        "zip"
    } else {
        "tar.gz"
    };

    let filename = format!("archgate-{os}-{arch}.{ext}");

    Ok(Json(DownloadPrebuiltOutput {
        download_url: format!(
            "https://github.com/archgate/cli/releases/download/v{version}/{filename}"
        ),
        download_name: Some(filename),
        ..DownloadPrebuiltOutput::default()
    }))
}

#[plugin_fn]
pub fn locate_executables(
    Json(_): Json<LocateExecutablesInput>,
) -> FnResult<Json<LocateExecutablesOutput>> {
    let env = get_host_environment()?;

    Ok(Json(LocateExecutablesOutput {
        exes: HashMap::from_iter([(
            "archgate".into(),
            ExecutableConfig::new_primary(env.os.get_exe_name("archgate")),
        )]),
        ..LocateExecutablesOutput::default()
    }))
}
