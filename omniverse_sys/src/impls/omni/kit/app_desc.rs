use crate::{carb_app_path, omni, result::Result, OmniverseSysError};
use builder_derive_more::IntoBuilder;
use derive_builder::Builder;
use once_cell::sync::Lazy;
use std::{collections::HashMap, env, ffi::CString, path::PathBuf};

impl omni::kit::AppDesc {
    #[must_use]
    pub fn builder() -> AppDescBuilder {
        AppDescBuilder::default()
    }

    #[must_use]
    pub fn kit_args_builder() -> KitArgsBuilder {
        KitArgsBuilder::default()
    }
}

#[derive(Debug, Clone, Builder)]
#[builder(
    default,
    name = "AppDescBuilder",
    build_fn(private, name = "intermediate_build", error = "OmniverseSysError")
)]
pub struct AppDescBuilderInternal {
    #[builder(setter(into))]
    carb_app_name: String,
    #[builder(setter(into))]
    carb_app_path: PathBuf,
    #[builder(setter(into))]
    experience: PathBuf,
    kit_args: KitArgs,
    #[builder(setter(each(name = "option", into)))]
    options: HashMap<String, String>,
}

impl Default for AppDescBuilderInternal {
    fn default() -> Self {
        let carb_app_path = carb_app_path();
        Self {
            carb_app_name: env::current_exe()
                .unwrap_or_default()
                .file_name()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default()
                .to_string(),
            carb_app_path: carb_app_path.clone(),
            experience: carb_app_path.join("apps").join("omni.app.full.kit"),
            kit_args: KitArgs::default(),
            options: DEFAULT_OPTIONS.clone(),
        }
    }
}

static DEFAULT_OPTIONS: Lazy<HashMap<String, String>> = Lazy::new(|| {
    let mut m: HashMap<String, String> = HashMap::with_capacity(12);

    // hardware
    m.insert(
        "/renderer/multiGpu/autoEnable".to_string(),
        true.to_string(),
    );
    m.insert("/physics/cudaDevice".to_string(), 0.to_string());

    // sync_loads
    let sync_loads = true;
    m.insert(
        "/omni.kit.plugin/syncUsdLoads".to_string(),
        sync_loads.to_string(),
    );
    m.insert(
        "/rtx/materialDb/syncLoads".to_string(),
        sync_loads.to_string(),
    );
    m.insert(
        "/rtx/hydra/materialSyncLoads".to_string(),
        sync_loads.to_string(),
    );
    // App
    let carb_app_path = carb_app_path().display().to_string();
    m.insert("/app/tokens/exe-path".to_string(), carb_app_path);
    m.insert("/app/fastShutdown".to_string(), true.to_string());
    m.insert(
        "/persistent/app/viewport/displayOptions".to_string(),
        3094.to_string(), // 3286.to_string(),
    );
    // Resolution
    m.insert(
        "/app/renderer/resolution/width".to_string(),
        1280.to_string(),
    );
    m.insert(
        "/app/renderer/resolution/height".to_string(),
        720.to_string(),
    );
    m.insert("/app/window/width".to_string(), 1440.to_string());
    m.insert("/app/window/height".to_string(), 900.to_string());

    m.shrink_to_fit();

    m
});

impl AppDescBuilder {
    pub fn build(&self) -> Result<omni::kit::AppDesc> {
        let app_desc = self.intermediate_build().unwrap_or_else(|_| unreachable!());

        if !app_desc.carb_app_path.is_dir() {
            return Err(OmniverseSysError::ValueError(format!(
                "Carbonite path is not a valid directory: {}",
                app_desc.carb_app_path.display()
            )));
        }
        if !app_desc.experience.is_file() {
            return Err(OmniverseSysError::ValueError(format!(
                "Experience path is not a valid file: {}",
                app_desc.experience.display()
            )));
        }

        let carb_app_name_ptr = CString::new(app_desc.carb_app_name.clone())
            .unwrap()
            .into_raw();
        let carb_app_path_ptr = CString::new(app_desc.carb_app_path.to_str().unwrap())
            .unwrap()
            .into_raw();

        // let current_exe = env::current_exe().unwrap_or_default().display().to_string();
        let experience = app_desc.experience.display().to_string();

        // Construct args
        let args1 = vec![
            app_desc.carb_app_path.join("kit").display().to_string(),
            experience,
        ];
        let args2 = app_desc.kit_args.clone().into_args();
        let args3 = app_desc
            .options
            .clone()
            .into_iter()
            .map(|(k, v)| format!("--{k}={v}"))
            .collect::<Vec<_>>();

        // Convert to argv + argc
        let mut argv = args1
            .into_iter()
            .map(|s| CString::new(s).unwrap().into_raw())
            .chain(
                args2
                    .into_iter()
                    .map(|s| CString::new(s).unwrap().into_raw()),
            )
            .chain(
                args3
                    .into_iter()
                    .map(|s| CString::new(s).unwrap().into_raw()),
            )
            .collect::<Vec<_>>();
        let argc = argv.len().try_into().unwrap();
        let argv_ptr = argv.as_mut_ptr();
        std::mem::forget(argv);

        Ok(omni::kit::AppDesc {
            carbAppName: carb_app_name_ptr,
            carbAppPath: carb_app_path_ptr,
            argc,
            argv: argv_ptr,
        })
    }
}

#[derive(Debug, Clone, Builder, IntoBuilder)]
#[builder(
    default,
    build_fn(private, name = "fallible_build", error = "OmniverseSysError")
)]
pub struct KitArgs {
    allow_root: bool,
    clear_cache: bool,
    clear_data: bool,
    disable_ext_startup: bool,
    #[builder(setter(each(name = "enable_ext", into)))]
    enable_exts: Vec<String>,
    #[builder(setter(each(name = "ext_folder", into)))]
    ext_folders: Vec<PathBuf>,
    #[builder(setter(each(name = "ext_path", into)))]
    ext_paths: Vec<PathBuf>,
    headless: bool,
    #[builder(setter(each(name = "merge_config_append", into)))]
    merge_config: Vec<PathBuf>,
    portable_root: Option<PathBuf>,
    portable: bool,
    reset_user: bool,
    verbose: bool,
    wait_debugger: bool,
}

impl KitArgsBuilder {
    pub fn build(&self) -> KitArgs {
        self.fallible_build().unwrap_or_else(|_| unreachable!())
    }
}

impl Default for KitArgs {
    fn default() -> Self {
        let carb_app_path = carb_app_path();

        Self {
            allow_root: false,
            clear_cache: false,
            clear_data: false,
            disable_ext_startup: false,
            enable_exts: Vec::default(),
            ext_folders: vec![carb_app_path.join("apps"), carb_app_path.join("exts")],
            ext_paths: Vec::default(),
            headless: false,
            merge_config: Vec::default(),
            portable_root: None,
            portable: false,
            reset_user: false,
            verbose: false,
            wait_debugger: false,
        }
    }
}

impl KitArgs {
    fn into_args(self) -> Vec<String> {
        let mut vec = Vec::with_capacity(8 + 2 * self.enable_exts.len());
        if self.allow_root {
            vec.push("--allow-root".to_string());
        }
        if self.clear_cache {
            vec.push("--clear-cache".to_string());
        }
        if self.clear_data {
            vec.push("--clear-data".to_string());
        }
        if self.disable_ext_startup {
            vec.push("--disable-ext-startup".to_string());
        }
        for ext in self.enable_exts {
            vec.push("--enable".to_string());
            vec.push(ext);
        }
        for ext in self.ext_folders {
            vec.push("--ext-folder".to_string());
            vec.push(ext.display().to_string());
        }
        for ext in self.ext_paths {
            vec.push("--ext-path".to_string());
            vec.push(ext.display().to_string());
        }
        if self.headless {
            vec.push("--no-window".to_string());
        }
        for ext in self.merge_config {
            vec.push(format!("--merge-config={}", ext.display()));
        }
        if let Some(portable_root) = self.portable_root {
            vec.push("--portable-root".to_string());
            vec.push(portable_root.display().to_string());
        }
        if self.portable {
            vec.push("--portable".to_string());
        }
        if self.reset_user {
            vec.push("--reset-user".to_string());
        }
        if self.verbose {
            vec.push("--verbose".to_string());
        }
        if self.wait_debugger {
            vec.push("--wait-debugger".to_string());
        }
        vec.shrink_to_fit();
        vec
    }
}
