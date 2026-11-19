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
