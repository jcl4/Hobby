mod util;

use std::ffi::{CString, CStr};

use ash::{version::EntryV1_2, vk, Entry};

use crate::config::Config;
use crate::Result;

pub(crate) struct Renderer {
    _entry: Entry,
}

impl Renderer {
    pub(crate) fn new(config: Config) -> Result<Renderer> {
        let entry = Entry::new()?;

        let app_name = CString::new(config.application.name)?;
        let app_version = vk::make_version(
            config.application.version[0],
            config.application.version[1],
            config.application.version[2],
        );

        let major = env!("CARGO_PKG_VERSION_MAJOR").parse::<u32>()?;
        let minor = env!("CARGO_PKG_VERSION_MINOR").parse::<u32>()?;
        let patch = env!("CARGO_PKG_VERSION_PATCH").parse::<u32>()?;
        let engine_version = vk::make_version(major, minor, patch);

        let api_version = vk::make_version(1, 2, 0);

        let app_info = vk::ApplicationInfo::builder()
            .application_name(&app_name)
            .application_version(app_version)
            .engine_name(&CString::new("Hobby")?)
            .engine_version(engine_version)
            .api_version(api_version)
            .build();
        
        let required_extensions = util::required_extension_names();


        Ok(Renderer { _entry: entry })
    }
}
