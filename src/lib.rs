#![allow(dead_code, unused_variables)]

mod address_space;
mod cacher;
mod data_source;

pub use address_space::AddressSpace;
pub use data_source::{DataSource, FileDataSource, FlagBuilder};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructors() {
        let _a = AddressSpace::new("my first address space");
        let _ds: FileDataSource = FileDataSource::new("Cargo.toml").unwrap(); // a little silly, but why not?
    }

    // more tests here - add mappings, read data, remove mappings and add more, make sure the
    // address space has what we expect in it after each operation
}
