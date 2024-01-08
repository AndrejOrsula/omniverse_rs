use crate::{
    carb,
    utils::string::{from_cstring_array, to_cstring_array},
};

impl carb::PluginLoadingDesc {
    #[must_use]
    pub fn builder() -> PluginLoadingDescBuilder {
        PluginLoadingDescBuilder::default()
    }

    #[must_use]
    pub fn configure(self) -> PluginLoadingDescBuilder {
        self.into()
    }
}

impl From<carb::PluginLoadingDesc> for PluginLoadingDescBuilder {
    fn from(desc: carb::PluginLoadingDesc) -> Self {
        Self {
            search_paths: unsafe {
                from_cstring_array(desc.searchPaths as *mut *mut _, desc.searchPathCount).unwrap()
            },
            search_recursive: desc.searchRecursive,
            loaded_file_wildcards: unsafe {
                from_cstring_array(
                    desc.loadedFileWildcards as *mut *mut _,
                    desc.loadedFileWildcardCount,
                )
                .unwrap()
            },
            reloadable_file_wildcards: unsafe {
                from_cstring_array(
                    desc.reloadableFileWildcards as *mut *mut _,
                    desc.reloadableFileWildcardCount,
                )
                .unwrap()
            },
            unload_plugins: desc.unloadPlugins,
            excluded_file_wildcards: unsafe {
                from_cstring_array(
                    desc.excludedFileWildcards as *mut *mut _,
                    desc.excludedFileWildcardCount,
                )
                .unwrap()
            },
        }
    }
}

#[derive(Debug, Default)]
pub struct PluginLoadingDescBuilder {
    search_paths: Vec<String>,
    search_recursive: bool,
    loaded_file_wildcards: Vec<String>,
    reloadable_file_wildcards: Vec<String>,
    unload_plugins: bool,
    excluded_file_wildcards: Vec<String>,
}

impl PluginLoadingDescBuilder {
    pub fn build(&self) -> carb::PluginLoadingDesc {
        let search_path_count = self.search_paths.len();
        let loaded_file_wildcard_count = self.loaded_file_wildcards.len();
        let reloadable_file_wildcard_count = self.reloadable_file_wildcards.len();
        let excluded_file_wildcard_count = self.excluded_file_wildcards.len();

        carb::PluginLoadingDesc {
            searchPaths: to_cstring_array(self.search_paths.clone()).unwrap() as *const *const _,
            searchPathCount: search_path_count,
            searchRecursive: self.search_recursive,
            loadedFileWildcards: to_cstring_array(self.loaded_file_wildcards.clone()).unwrap()
                as *const *const _,
            loadedFileWildcardCount: loaded_file_wildcard_count,
            reloadableFileWildcards: to_cstring_array(self.reloadable_file_wildcards.clone())
                .unwrap() as *const *const _,
            reloadableFileWildcardCount: reloadable_file_wildcard_count,
            unloadPlugins: self.unload_plugins,
            excludedFileWildcards: to_cstring_array(self.excluded_file_wildcards.clone()).unwrap()
                as *const *const _,
            excludedFileWildcardCount: excluded_file_wildcard_count,
        }
    }

    pub fn search_paths(mut self, search_paths: Vec<String>) -> Self {
        self.search_paths = search_paths;
        self
    }

    pub fn add_search_path(mut self, search_path: &str) -> Self {
        self.search_paths.push(search_path.to_string());
        self
    }

    pub fn clear_search_paths(mut self) -> Self {
        self.search_paths.clear();
        self
    }

    pub fn search_recursive(mut self, search_recursive: bool) -> Self {
        self.search_recursive = search_recursive;
        self
    }

    pub fn loaded_file_wildcards(mut self, loaded_file_wildcards: Vec<String>) -> Self {
        self.loaded_file_wildcards = loaded_file_wildcards;
        self
    }

    pub fn add_loaded_file_wildcard(mut self, loaded_file_wildcard: &str) -> Self {
        self.loaded_file_wildcards
            .push(loaded_file_wildcard.to_string());
        self
    }

    pub fn clear_loaded_file_wildcards(mut self) -> Self {
        self.loaded_file_wildcards.clear();
        self
    }

    pub fn reloadable_file_wildcards(mut self, reloadable_file_wildcards: Vec<String>) -> Self {
        self.reloadable_file_wildcards = reloadable_file_wildcards;
        self
    }

    pub fn add_reloadable_file_wildcard(mut self, reloadable_file_wildcard: &str) -> Self {
        self.reloadable_file_wildcards
            .push(reloadable_file_wildcard.to_string());
        self
    }

    pub fn clear_reloadable_file_wildcards(mut self) -> Self {
        self.reloadable_file_wildcards.clear();
        self
    }

    pub fn unload_plugins(mut self, unload_plugins: bool) -> Self {
        self.unload_plugins = unload_plugins;
        self
    }

    pub fn excluded_file_wildcards(mut self, excluded_file_wildcards: Vec<String>) -> Self {
        self.excluded_file_wildcards = excluded_file_wildcards;
        self
    }

    pub fn add_excluded_file_wildcard(mut self, excluded_file_wildcard: &str) -> Self {
        self.excluded_file_wildcards
            .push(excluded_file_wildcard.to_string());
        self
    }

    pub fn clear_excluded_file_wildcards(mut self) -> Self {
        self.excluded_file_wildcards.clear();
        self
    }
}
