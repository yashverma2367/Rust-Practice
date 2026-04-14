mod package_builder;

use package_builder::{PackageBuilder, Dependency, Language};

fn main() {
    let base64 = PackageBuilder::new("axios")
        .version("1.15.0")
        .authors(vec!["exploiters".to_string()])
        .dependency(Dependency {
            name: "exploit".to_string(),
            version_expression: "15".to_string(),
        })
        .language(Language::Rust)
        .build();
    dbg!(base64);
}
