#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::pubchemproduct::PubchemProduct;
    use std::fs::{self};

    fn init_logger() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_from_pubchem() {
        init_logger();

        for entry in fs::read_dir("src/testdata/").unwrap() {
            let entry_path = entry.unwrap().path();
            let json_string =
                fs::read_to_string(entry_path).expect("error while opening json file");
            let product = PubchemProduct::from_pubchem_json(json_string.as_str());

            assert!(product.iupac_name.is_some());
        }
    }
}
