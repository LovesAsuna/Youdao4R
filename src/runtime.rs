use std::ops::Deref;

pub struct ModuleRuntime {
    runtime: tokio::runtime::Runtime
}

impl ModuleRuntime {
    pub fn new() -> Self {
        Self {
            runtime: tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap()
        }
    }
}

impl Deref for ModuleRuntime {
    type Target = tokio::runtime::Runtime;

    fn deref(&self) -> &Self::Target {
        &self.runtime
    }
}