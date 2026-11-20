[![Rust](https://github.com/tbellembois/chimitheque_types/actions/workflows/rust.yml/badge.svg)](https://github.com/tbellembois/chimitheque_types/actions/workflows/rust.yml) [![codecov](https://codecov.io/github/tbellembois/chimitheque_types/graph/badge.svg?token=82SDGZM7BH)](https://codecov.io/github/tbellembois/chimitheque_types)

Types package for the Chimitheque application.

## JSON conventions

- An omitted `RequestFilter.order` defaults to `"asc"`, matching `RequestFilter::default()`.
  Explicit values, including an empty string, are preserved.
- `match_exact_search` defaults to `false` when omitted on any type that has this field.
  Explicit values are preserved, and the field remains present in serialized output.
  Required labels and other required domain fields are unchanged.
- `UnitType` serializes as `"quantity"`, `"concentration"`, `"temperature"`, or
  `"molecular_weight"`, matching its string formatting and parsing conventions.
  Legacy JSON spellings (`"Quantity"`, `"Concentration"`, `"Temperature"`, and
  `"MolecularWeight"`) remain accepted as input, but output always uses snake_case.
  Consumers expecting PascalCase output must update to accept snake_case.

## PubChem parsing

`PubchemProduct::from_pubchem_json` now returns
`Result<PubchemProduct, chimitheque_types::error::PubchemParseError>` instead of
silently returning an empty product on invalid JSON. Callers must propagate the
error with `?` or handle it explicitly:

```rust
use chimitheque_types::{error::PubchemParseError, pubchemproduct::PubchemProduct};

fn parse_response(body: &str) -> Result<PubchemProduct, PubchemParseError> {
    PubchemProduct::from_pubchem_json(body)
}
```

- Malformed JSON preserves the original `serde_json::Error` as its error source.
- The response must be a JSON object with an object-valued `Record`; API fault
  responses and unrelated JSON are rejected.
- Query failures report the product field, query, and underlying diagnostic.
- Matched values must be strings. Nulls and other types produce a field-specific
  error rather than being discarded, including invalid entries in lists.
- Missing optional paths remain `None`. Scalar fields still select the first
  match, but all matched values are checked. Extraction fails as a whole rather
  than returning a partially populated product.

This is a selected-field extractor, not a complete PubChem schema validator.
Missing paths (including unsupported intermediate structures) are treated as
absent. Existing heading and measurement-selection rules are unchanged.

sample product list:

- 1,4-butanediol
- 1,4-dioxane
- acetate d'ethyl
- acetic acid
- acetone
- Aluminium oxide
- ammonium chloride
- celite
- cesium carbonate
- chloroform
- chlorotrimethylsilane
- cyclohexane
- dichloromethane
- diisopropyl ether
- diisopropylethylamine
- ethanol
- formic acid
- heptane
- hexane
- hydrochloric acid
- hydrogen peroxide
- iodine
- lithium aluminium hydride
- methanol
- n-butyllithium
- nitric acid
- palladium acetate
- palladium on carbon Pd/C
- pentane
- phosphorous pentoxide
- potassium carbonate
- potassium hydroxide
- pyridine
- silica
- sodium bicarbonate
- sodium borohydride
- sodium carbonate
- sodium hydroxide
- sodium iodide
- sodium sulfate
- sulfuric acid
- tert-butyl carbonate
- Tetrabutylammonium fluoride solution
- tetrahydrofuran
- tetrakis (triphenylphosphine) palladium(0)
- trifluoroacetic acid
- trifluoromethanesulfonic anhydride
- trimethylsilyl acetylene
- Tris(dibenzylideneacetone)dipalladium(0)
- triéthylamine
