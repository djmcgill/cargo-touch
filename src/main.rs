use cargo_toml::Manifest;
use env_logger::{Builder, Env};
use filetime::set_file_mtime;
use log::*;
use std::error::Error;
use std::path::Path;
use std::time::SystemTime;

fn main() -> Result<(), Box<dyn Error>> {
    Builder::from_env(Env::default().default_filter_or("info")).try_init()?;
    info!("Starting cargo-touch from current directory");
    run_with_toml_path(".")?;
    info!("Touching complete (n.b. touching of tests/examples/benchmarks not yet supported)");
    Ok(())
}

fn run_with_toml_path(path: impl AsRef<Path>) -> Result<(), Box<dyn Error>> {
    let toml_path = path.as_ref().join("Cargo.toml");
    debug!("Running for manifest path: {:?}", toml_path);

    let mut manifest = Manifest::from_path(&toml_path)?;
    trace!("Found manifest file");

    manifest.complete_from_path(&toml_path)?;
    trace!("Completed manifest");

    for bin in &manifest.bin {
        if let Some(bin_path) = &bin.path {
            trace!("Bin path found: {:?}", bin_path);
            set_file_mtime(path.as_ref().join(bin_path), SystemTime::now().into())?;
        }
    }

    if let Some(lib) = &manifest.lib {
        if let Some(lib_path) = &lib.path {
            trace!("Lib path found: {:?}", lib_path);
            set_file_mtime(path.as_ref().join(lib_path), SystemTime::now().into())?;
        }
    }

    if let Some(workspace) = &manifest.workspace {
        trace!("Workspace found: {:#?}", workspace);
        for workspace_member in &workspace.members {
            run_with_toml_path(workspace_member)?;
        }
    }

    // FIXME: tests, examples, benches
    Ok(())
}
