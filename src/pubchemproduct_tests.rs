#[cfg(test)]
mod tests {

    use crate::pubchemproduct::PubchemProduct;
    use std::{
        fs::{self},
        path::Path,
    };

    fn init_logger() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_from_pubchem() {
        init_logger();

        let json_file_path = Path::new("src/testdata/pubchem_pug_view.json");
        let json_string =
            fs::read_to_string(json_file_path).expect("error while opening json file");

        let _maybe_product = PubchemProduct::from_pubchem_json(json_string.as_str());
    }
}
