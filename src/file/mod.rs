mod format;
pub mod source;

use source::Source;
use error::*;
use value::Value;
use std::collections::HashMap;

use self::source::FileSource;
pub use self::format::FileFormat;

pub struct File<T>
    where T: FileSource
{
    source: T,

    /// Namespace to restrict configuration from the file
    namespace: Option<String>,

    /// Format of file (which dictates what driver to use).
    format: Option<FileFormat>,

    /// A required File will error if it cannot be found
    required: bool,
}

impl File<source::string::FileSourceString> {
    pub fn from_str(s: &str, format: FileFormat) -> Self {
        File {
            format: Some(format),
            required: true,
            namespace: None,
            source: s.into(),
        }
    }
}

impl File<source::file::FileSourceFile> {
    pub fn new(name: &str, format: FileFormat) -> Self {
        File {
            format: Some(format),
            required: true,
            namespace: None,
            source: source::file::FileSourceFile::new(name),
        }
    }
}

impl<T: FileSource> File<T> {
    pub fn required(&mut self, required: bool) -> &mut Self {
        self.required = required;
        self
    }

    pub fn namespace(&mut self, namespace: &str) -> &mut Self {
        self.namespace = Some(namespace.into());
        self
    }
}

impl<T: FileSource> Source for File<T> {
    fn collect(&self) -> Result<HashMap<String, Value>> {
        // Coerce the file contents to a string
        let (uri, contents) = self.source.resolve(self.format).map_err(|err| {
            ConfigError::Foreign(err)
        })?;

        // Parse the string using the given format
        self.format.unwrap().parse(uri.as_ref(), &contents, self.namespace.as_ref()).map_err(|cause| {
            ConfigError::FileParse {
                uri: uri,
                cause: cause
            }
        })
    }
}
