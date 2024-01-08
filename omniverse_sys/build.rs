/// List of libraries to link against (`-l`)
const LINK_LIBS: &[&str] = &[
    "carb",
    "omni_spectree",
    "omni_usd_live_v1",
    "omni_usd_live_v2",
    "omni_usd_resolver",
    "omni.activity.ui.bar",
    "omni.kit.scene_view.opengl",
    "omni.kit.scene_view.usd",
    "omni.kit.ui",
    "omni.kit.viewport.scene_camera_model",
    "omni.kit.widget.inspector",
    "omni.kit.widget.opengl",
    "omni.kit.widget.text_editor",
    "omni.ui.scene",
    "omni.ui",
    "omni.usd",
    "omniAudioSchema",
    "omniclient",
    "omniGeospatialSchema",
    "omniGraphSchema",
    "omniGraphSchemaTools",
    "omniScriptingSchema",
    "omniScriptingSchemaTools",
    "omniSkelSchema",
    "omniverse_connection",
];

/// List of link paths (`-L`), either absolute or relative to the `CARB_APP_PATH`
const LINK_PATHS: &[&str] = &[
    ".",
    "exts/carb.audio/bin",
    "exts/carb.windowing.plugins/bin",
    "exts/omni.activity.core/bin",
    "exts/omni.activity.freeze_monitor/bin",
    "exts/omni.activity.profiler/bin",
    "exts/omni.activity.pump/bin",
    "exts/omni.activity.ui/bin",
    "exts/omni.activity.usd_resolver/bin",
    "exts/omni.appwindow/bin",
    "exts/omni.audioplayer/bin",
    "exts/omni.audiorecorder/bin",
    "exts/omni.datastore/bin",
    "exts/omni.datastore/bin/deps",
    "exts/omni.debugdraw/bin",
    "exts/omni.fabric.fabric_inspector/bin",
    "exts/omni.gpu_foundation/bin",
    "exts/omni.gpu_foundation/bin/deps",
    "exts/omni.gpucompute.plugins/bin",
    "exts/omni.gpucompute.plugins/bin/deps",
    "exts/omni.graph.action/bin",
    "exts/omni.graph.core/bin",
    "exts/omni.graph.examples.cpp/bin",
    "exts/omni.graph.exec/bin",
    "exts/omni.graph.expression/bin",
    "exts/omni.graph.image.core/bin",
    "exts/omni.graph.io/bin",
    "exts/omni.graph.nodes/bin",
    "exts/omni.graph.rtxtest/bin",
    "exts/omni.graph.test/bin",
    "exts/omni.graph.tutorials/bin",
    "exts/omni.graph.ui_nodes/bin",
    "exts/omni.graph.ui/bin",
    "exts/omni.hsscclient/bin",
    "exts/omni.hsscclient/bin/deps",
    "exts/omni.hydra.index_remote/bin",
    "exts/omni.hydra.index/bin",
    "exts/omni.hydra.index/bin/deps",
    "exts/omni.hydra.iray/bin",
    "exts/omni.hydra.iray/bin/deps",
    "exts/omni.hydra.pxr/bin",
    "exts/omni.hydra.rtx/bin",
    "exts/omni.hydra.rtx/bin/deps",
    "exts/omni.hydra.scene_api/bin",
    "exts/omni.hydra.scene_delegate/bin",
    "exts/omni.hydra.usdrt_delegate/bin",
    "exts/omni.index.compute/bin",
    "exts/omni.index.libs/bin",
    "exts/omni.index.usd/bin",
    "exts/omni.index/bin",
    "exts/omni.index/bin/deps",
    "exts/omni.inspect/bin",
    "exts/omni.iray.libs/bin",
    "exts/omni.kit.actions.core/bin",
    "exts/omni.kit.app_snippets/bin",
    "exts/omni.kit.audiodeviceenum/bin",
    "exts/omni.kit.clipboard/bin",
    "exts/omni.kit.commands/bin",
    "exts/omni.kit.exec.core/bin",
    "exts/omni.kit.exec.example-carb/bin",
    "exts/omni.kit.exec.example-omni/bin",
    "exts/omni.kit.hydra_texture/bin",
    "exts/omni.kit.loop-default/bin",
    "exts/omni.kit.multinode/bin",
    "exts/omni.kit.raycast.query/bin",
    "exts/omni.kit.renderer.capture/bin",
    "exts/omni.kit.renderer.core/bin",
    "exts/omni.kit.renderer.cuda_interop/bin",
    "exts/omni.kit.renderer.imgui/bin",
    "exts/omni.kit.renderer.imgui/bin/deps",
    "exts/omni.kit.renderer.init/bin",
    "exts/omni.kit.scene_view.opengl/bin",
    "exts/omni.kit.scene_view.usd/bin",
    "exts/omni.kit.telemetry/bin",
    "exts/omni.kit.test/bin",
    "exts/omni.kit.usd.layers/bin",
    "exts/omni.kit.viewport.legacy_gizmos/bin",
    "exts/omni.kit.viewport.scene_camera_model/bin",
    "exts/omni.kit.widget.inspector/bin",
    "exts/omni.kit.widget.opengl/bin",
    "exts/omni.kit.widget.text_editor/bin",
    "exts/omni.kit.window.console/bin",
    "exts/omni.kit.window.imguidebug/bin",
    "exts/omni.kit.window.script_editor/bin",
    "exts/omni.kit.window.splash/bin",
    "exts/omni.kit.window.status_bar/bin",
    "exts/omni.kit.window.viewport/bin",
    "exts/omni.mdl.neuraylib/bin",
    "exts/omni.mdl.neuraylib/bin/deps",
    "exts/omni.population/bin",
    "exts/omni.resourcemonitor/bin",
    "exts/omni.rtx.ovtextureconverter/bin",
    "exts/omni.rtx.ujitsoprocessors/bin",
    "exts/omni.stats/bin",
    "exts/omni.stats/bin/deps",
    "exts/omni.syntheticdata/bin",
    "exts/omni.taskagent/bin",
    "exts/omni.taskagent/bin/deps",
    "exts/omni.timeline/bin",
    "exts/omni.ui.scene/bin",
    "exts/omni.ui/bin",
    "exts/omni.uiaudio/bin",
    "exts/omni.ujitso.python/bin",
    "exts/omni.ujitso.python/bin/deps",
    "exts/omni.ujitso/bin",
    "exts/omni.ujitso/bin/deps",
    "exts/omni.usd_resolver/bin",
    "exts/omni.usd.core/bin",
    "exts/omni.usd.libs/bin",
    "exts/omni.usd.libs/bin/deps",
    "exts/omni.usd.schema.anim/bin",
    "exts/omni.usd.schema.audio/bin",
    "exts/omni.usd.schema.geospatial/bin",
    "exts/omni.usd.schema.omnigraph/bin",
    "exts/omni.usd.schema.omniscripting/bin",
    "exts/omni.usd.schema.semantics/bin",
    "exts/omni.usd/bin",
    "exts/omni.videoencoding/bin",
    "exts/omni.volume/bin",
    "exts/omni.volume/bin/deps",
    "exts/usdrt.scenegraph/bin",
    "extscore/omni.assets.plugins/bin",
    "extscore/omni.assets.plugins/bin/deps",
    "extscore/omni.client/bin",
    "extscore/omni.client/bin/deps",
    "kernel/plugins",
];

/// List of include paths (`-I`), either absolute or relative to the `CARB_APP_PATH`
const INCLUDE_PATHS_CARB_APP: &[&str] = &[
    "dev/include",
    "dev/fabric/include",
    "dev/gsl/include",
    "dev/ogn/include",
    "exts/usdrt.scenegraph/include",
];
/// List of include paths (`-I`), either absolute or relative to the `OPENUSD_PATH`
const INCLUDE_PATHS_OPENUSD: &[&str] = &["include"];

fn main() {
    // Configure rebuild triggers based on environment variables
    println!("cargo:rerun-if-env-changed=CARB_APP_PATH");

    // Get path to the OpenUSD installation
    let openusd_path = pxr::openusd_path();
    // Verify that the OpenUSD installation contains the required subdirectories
    for path in INCLUDE_PATHS_OPENUSD {
        let path = std::path::PathBuf::from(path);
        if path.is_absolute() {
            continue;
        }
        let include_path = openusd_path.join(path);
        assert!(
            include_path.is_dir(),
            "The OpenUSD installation does not contain the required include path: `{}`",
            include_path.display()
        );
    }

    // Locate Carbonite (either from environment variable or vendored)
    let carb_app_path = {
        if let Some(carb_app_path) = std::env::var_os("CARB_APP_PATH") {
            let carb_app_path = std::path::PathBuf::from(carb_app_path);
            assert!(
                carb_app_path.is_dir(),
                "Environment variable `CARB_APP_PATH` does not point to a valid directory: `{}`",
                carb_app_path.display()
            );
            carb_app_path
        } else {
            // Panic if the environment variable is unset and download is disabled
            #[cfg(not(feature = "vendored"))]
            panic!(
                "Unable to locate Carbonite application (Kit SDK). Please, specify the \
                `CARB_APP_PATH` environment variable. If you wish to automatically download \
                it during the build process, enable the `vendored` feature."
            );

            // If the `vendored` feature is enabled, download Carbonite
            #[cfg(feature = "vendored")]
            vendored::download_carb_app()
        }
    };
    // Verify that the Carbonite installation contains the required subdirectories
    for path in LINK_PATHS {
        let path = std::path::PathBuf::from(path);
        if path.is_absolute() {
            continue;
        }
        let link_path = carb_app_path.join(path);
        assert!(
            link_path.is_dir(),
            "The Carbonite installation does not contain the required link path: `{}`",
            link_path.display()
        );
    }
    for path in INCLUDE_PATHS_CARB_APP {
        let path = std::path::PathBuf::from(path);
        if path.is_absolute() {
            continue;
        }
        let include_path = carb_app_path.join(path);
        assert!(
            include_path.is_dir(),
            "The Carbonite installation does not contain the required include path: `{}`",
            include_path.display()
        );
    }

    // Configure link libraries
    for lib in LINK_LIBS {
        println!("cargo:rustc-link-lib=dylib={lib}");
    }

    // Cargo automatically adds link search paths to the dynamic library search path only for paths within OUT_DIR
    // Therefore, we create a symbolic link to the library within OUT_DIR instead if configuring link paths directly
    // In this way, dynamic library search paths do not need to be set manually when executing binaries through Cargo
    let carb_app_path_out_dir =
        std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap_or_else(|| unreachable!()))
            .join("deps")
            .join("lib")
            .join(carb_app_path.file_name().unwrap());
    if !carb_app_path_out_dir.is_dir() {
        built_different::create_symlink(&carb_app_path, &carb_app_path_out_dir, true).unwrap();
    }
    for path in LINK_PATHS {
        let mut path = std::path::PathBuf::from(path);
        if path.is_relative() {
            path = carb_app_path_out_dir.join(path);
        }
        println!("cargo:rustc-link-search={}", path.display());
    }

    // Apply patches to the headers in a duplicate `include_patched_rs` directory
    built_different::apply_file_patches(
        "patches/include",
        carb_app_path.join("dev").join("include"),
        carb_app_path.join("dev").join("include_patched_rs"),
        true,
    )
    .unwrap();

    // Configure includes
    let include_paths_carb_app_local = [std::path::PathBuf::from("./include")];
    let include_paths_carb_app_patched = [carb_app_path.join("dev").join("include_patched_rs")];
    let include_paths_carb_app = INCLUDE_PATHS_CARB_APP.iter().map(|path| {
        let path = std::path::PathBuf::from(path);
        if path.is_relative() {
            carb_app_path.join(path)
        } else {
            path
        }
    });
    let include_paths_openusd_patched = [openusd_path.join("include_patched_rs")];
    let include_paths_openusd = INCLUDE_PATHS_OPENUSD.iter().map(|path| {
        let path = std::path::PathBuf::from(path);
        if path.is_relative() {
            openusd_path.join(path)
        } else {
            path
        }
    });
    let include_paths_python = python_config::PythonConfig::new().include_paths().unwrap();
    let include_paths = include_paths_carb_app_local
        .into_iter()
        .chain(include_paths_carb_app_patched)
        .chain(include_paths_carb_app)
        .chain(include_paths_openusd_patched)
        .chain(include_paths_openusd)
        .chain(include_paths_python)
        .collect::<Vec<_>>();

    // Generate bindings with `bindgen`
    bindings::generate_bindgen(&include_paths);

    // Generate bindings with `autocxx`
    bindings::generate_autocxx(&include_paths);

    // Compile C++ code with `cpp`
    bindings::compile_cpp(&include_paths);
}

mod bindings {
    pub fn generate_bindgen(include_paths: impl IntoIterator<Item = impl AsRef<std::ffi::OsStr>>) {
        bindgen::Builder::default()
            .header("include/omniverse_sys/omni.h")
            .clang_args(&[
                "-x",
                "c++",
                "-std=c++17",
                "-stdlib=libstdc++",
                "-pthread",
                "-D_GLIBCXX_USE_CXX11_ABI=0",
                "-Wno-everything",
            ])
            .clang_args(
                include_paths
                    .into_iter()
                    .map(|path| format!("-I{}", path.as_ref().to_string_lossy()))
                    .collect::<Vec<_>>(),
            )
            .dynamic_link_require_all(true)
            .enable_cxx_namespaces()
            .allowlist_type("carb::PluginLoadingDesc")
            .allowlist_type("omni::kit::AppDesc")
            .allowlist_type("omni::kit::PlatformInfo")
            .allowlist_type("omni::kit::RunLoop")
            .rustified_enum("omni::kit::RestartArgsPolicy")
            .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
            .generate()
            .unwrap()
            .write_to_file(
                std::path::PathBuf::from(
                    std::env::var_os("OUT_DIR").unwrap_or_else(|| unreachable!()),
                )
                .join("bindgen.rs"),
            )
            .unwrap();
    }

    pub fn generate_autocxx(include_paths: impl IntoIterator<Item = impl AsRef<std::ffi::OsStr>>) {
        autocxx_build::Builder::new("src/ffi/bindings.rs", include_paths)
            .extra_clang_args(&[
                "-x",
                "c++",
                "-std=c++17",
                "-stdlib=libstdc++",
                "-pthread",
                "-D_GLIBCXX_USE_CXX11_ABI=0",
                "-Wno-everything",
            ])
            .build()
            .unwrap()
            .compiler("clang")
            .flag("-x")
            .flag("c++")
            .flag("-std=c++17")
            .cpp_set_stdlib(Some("stdc++"))
            .flag("-pthread")
            .define("_GLIBCXX_USE_CXX11_ABI", Some("0"))
            .flag("-Wno-everything")
            .compile("omniverse_sys_autocxx");
    }

    pub fn compile_cpp(include_paths: impl IntoIterator<Item = impl AsRef<std::path::Path>>) {
        // Configure rebuild triggers based on file changes
        walkdir::WalkDir::new("src/ffi")
            .into_iter()
            .filter(|entry| {
                entry
                    .as_ref()
                    .map(|entry| {
                        entry.file_type().is_file()
                            && entry.path().extension() == Some("rs".as_ref())
                    })
                    .unwrap_or(false)
            })
            .for_each(|entry| {
                println!("cargo:rerun-if-changed={}", entry.unwrap().path().display());
            });

        let mut cpp_builder = cpp_build::Config::new();
        let mut cpp_builder = cpp_builder
            .compiler("clang")
            .flag("-x")
            .flag("c++")
            .flag("-std=c++17")
            .cpp_set_stdlib(Some("stdc++"))
            .define("_GLIBCXX_USE_CXX11_ABI", Some("0"))
            .flag("-pthread")
            .flag("-Wno-everything");
        for path in include_paths {
            cpp_builder = cpp_builder.include(path);
        }
        cpp_builder.build("src/lib.rs");
    }
}

#[cfg(feature = "vendored")]
mod vendored {
    pub fn download_carb_app() -> std::path::PathBuf {
        // ENV: Determine if the download should be forced regardless of the cache validity
        println!("cargo:rerun-if-env-changed=CARB_APP_DOWNLOAD_FORCE");
        let force_download = built_different::parse_bool_env("CARB_APP_DOWNLOAD_FORCE", false);

        // ENV: Determine the version of Carbonite to download
        const DEFAULT_CARB_APP_DOWNLOAD_VERSION: &str =
            "105.1+release.130815.04872a84.tc.linux-x86_64.release";
        println!("cargo:rerun-if-env-changed=CARB_APP_DOWNLOAD_VERSION");
        let carb_app_version_full = std::env::var("CARB_APP_DOWNLOAD_VERSION")
            .unwrap_or_else(|_| DEFAULT_CARB_APP_DOWNLOAD_VERSION.to_string());
        let carb_app_version_short = format!(
            "{}_{}",
            carb_app_version_full
                .split('+')
                .next()
                .unwrap_or(&carb_app_version_full),
            if carb_app_version_full.contains("release") {
                "release"
            } else {
                "debug"
            }
        );

        // ENV: Determine the URL from which download Carbonite
        // TODO: Find a better URL for downloading Carbonite
        const DEFAULT_CARB_APP_DOWNLOAD_URL: &str =
            "https://d4i3qtqj3r0z5.cloudfront.net/kit-sdk%40${{VERSION}}.7z";
        println!("cargo:rerun-if-env-changed=CARB_APP_DOWNLOAD_URL");
        let download_url = std::env::var("CARB_APP_DOWNLOAD_URL")
            .unwrap_or_else(|_| DEFAULT_CARB_APP_DOWNLOAD_URL.to_string());
        // Replace a potential version placeholder in the URL
        let download_url =
            download_url.replace("${{VERSION}}", &urlencoding::encode(&carb_app_version_full));

        // ENV: Determine whether to symlink or move the installation to the cache
        println!("cargo:rerun-if-env-changed=CARB_APP_DOWNLOAD_SYMLINK_CACHE");
        let symlink_cache =
            built_different::parse_bool_env("CARB_APP_DOWNLOAD_SYMLINK_CACHE", true);

        // Determine the cache path based on the version of Carbonite
        let cache_path = determine_cache_path_carb_app(&carb_app_version_short);

        // Make CARB_APP_PATH environment variable point to the cache (used for default location of Carbonite if CARB_APP_PATH is not set during runtime)
        println!("cargo:rustc-env=CARB_APP_PATH={}", cache_path.display());

        // Skip download if the cache is valid and download is not forced
        if !force_download && (std::fs::read_link(&cache_path).is_ok() || cache_path.is_dir()) {
            return cache_path;
        }

        // Determine the path where the Carbonite will be extracted
        let carb_app_path =
            std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap_or_else(|| unreachable!()))
                .join(format!("kit-sdk{carb_app_version_short}"));

        // Download and uncompress
        built_different::download_and_uncompress(&download_url, &carb_app_path, force_download)
            .unwrap();

        // Move Carbonite to the cache or create a symlink
        if symlink_cache && cache_path.exists() {
            built_different::create_symlink(&carb_app_path, &cache_path, true).unwrap();
        } else if !cache_path.is_symlink() {
            if let Some(parent) = cache_path.parent() {
                std::fs::create_dir_all(parent).unwrap();
            }
            std::fs::rename(&carb_app_path, &cache_path).unwrap();
        }

        cache_path
    }

    pub fn determine_cache_path_carb_app(version: &str) -> std::path::PathBuf {
        // The root of the cache path is in the system's cache directory (or /tmp if not available)
        let cache_root = built_different::cache_dir().unwrap_or(std::path::PathBuf::from("/tmp"));

        // Use different path for each Carbonite version and build configuration
        cache_root
            .join("omniverse_rs")
            .join(format!("kit-sdk{version}"))
    }
}
