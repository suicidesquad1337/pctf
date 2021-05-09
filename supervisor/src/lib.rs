//!

#![deny(rust_2018_idioms, broken_intra_doc_links)]
#![forbid(unsafe_code)]

pub mod docker;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
