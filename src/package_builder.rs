#![allow(dead_code)]

#[derive(Debug)]
pub enum Language {
    Rust,
    Java,
    Perl,
}

#[derive(Clone, Debug)]
pub struct Dependency {
    pub name: String,
    pub version_expression: String,
}

/// A representation of a software package.
#[derive(Debug)]
pub struct Package {
    name: String,
    version: String,
    authors: Vec<String>,
    dependencies: Vec<Dependency>,
    language: Option<Language>,
}

impl Package {
    /// Return a representation of this package as a dependency, for use in
    /// building other packages.
    fn as_dependency(&self) -> Dependency {
        Dependency {
            name: self.name.clone(),
            version_expression: self.version.clone(),
        }
    }
}

/// A builder for a Package. Use `build()` to create the `Package` itself.
pub struct PackageBuilder(Package);

impl PackageBuilder {
    pub fn new(name: impl Into<String>) -> Self {
        Self(Package {
            name: name.into(),
            version: String::new(),
            authors: Vec::new(),
            dependencies: Vec::new(),
            language: None,
        })
    }

    /// Set the package version.
    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.0.version = version.into();
        self
    }

    /// Set the package authors.
    pub fn authors(mut self, authors: Vec<String>) -> Self {
        self.0.authors = authors;
        self
    }

    /// Add an additional dependency.
    pub fn dependency(mut self, dependency: Dependency) -> Self {
        self.0.dependencies.push(dependency);
        self
    }

    /// Set the language. If not set, language defaults to None.
    pub fn language(mut self, language: Language) -> Self {
        self.0.language = language.into();
        self
    }

    pub fn build(self) -> Package {
        self.0
    }
}
