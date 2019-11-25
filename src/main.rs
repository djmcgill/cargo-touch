use cargo_toml::{Manifest, Product};
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
        trace!("Bin found: {:?}", bin);
        set_modified_time(&path, bin)?;
    }

    if let Some(lib) = &manifest.lib {
        trace!("Lib found: {:?}", lib);
        set_modified_time(&path, lib)?;
    }

    for test in &manifest.test {
        trace!("Test found: {:?}", test);
        set_modified_time(&path, test)?;
    }

    for example in &manifest.example {
        trace!("Example found: {:?}", example);
        set_modified_time(&path, example)?;
    }

    for bench in &manifest.bench {
        trace!("Bench found: {:?}", bench);
        set_modified_time(&path, bench)?;
    }

    if let Some(workspace) = &manifest.workspace {
        trace!("Workspace found: {:#?}", workspace);
        for workspace_member in &workspace.members {
            run_with_toml_path(workspace_member)?;
        }
    }

    Ok(())
}

fn set_modified_time(path: impl AsRef<Path>, product: &Product) -> Result<(), Box<dyn Error>> {
    if let Some(sub_path) = &product.path {
        let sub_toml_file = path.as_ref().join(sub_path);
        trace!("Sub path found: {:?}", sub_toml_file);

        // TODO: set access time too?
        set_file_mtime(sub_toml_file, SystemTime::now().into())?;
    }
    Ok(())
}
