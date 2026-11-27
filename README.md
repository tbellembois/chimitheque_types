[![Rust](https://github.com/tbellembois/chimitheque_types/actions/workflows/rust.yml/badge.svg)](https://github.com/tbellembois/chimitheque_types/actions/workflows/rust.yml) [![codecov](https://codecov.io/github/tbellembois/chimitheque_types/graph/badge.svg?token=82SDGZM7BH)](https://codecov.io/github/tbellembois/chimitheque_types)

Types of [Chimithèque](https://github.com/tbellembois/chimitheque_back), a management application for chemical products, biological reagents and laboratory consumables.

## JSON conventions

- `RequestFilter.order` is a `SortOrder` (`Asc` or `Desc`) and serializes as `"asc"` or
  `"desc"`. An omitted value defaults to `"asc"`, matching `RequestFilter::default()`.
  Input is case-insensitive in JSON and in the `?order=` URL parameter (`"DESC"` is
  accepted), but output is always lowercase. Any other value, including an empty
  string, is rejected.
- `match_exact_search` defaults to `false` when omitted on any type that has this field.
  Explicit values are preserved, and the field remains present in serialized output.
  Labels and the other required fields stay required.
- `UnitType` serializes as `"quantity"`, `"concentration"`, `"temperature"`, or
  `"molecular_weight"`, matching its string formatting and parsing conventions.
  Legacy JSON spellings (`"Quantity"`, `"Concentration"`, `"Temperature"`, and
  `"MolecularWeight"`) remain accepted as input, but output always uses snake_case.

## Request filters from URLs

`RequestFilter::try_from(url)` reads a filter from a request URL:

- A number at the end of the path is the id, e.g. `12` in `/storages/12?limit=1`.
  An `?id=` parameter overrides it.
- Each query parameter is parsed into its field. An invalid value, including an
  id too big for a `u64`, returns a `RequestFilterError` naming the parameter.
- `Display` writes the filter back as a query string, e.g. `?order=desc`.

## Validation

The types sent by clients have a `sanitize_and_validate(&mut self)` method, to
call before saving them. It cleans their text fields: each line is trimmed and
its whitespaces collapsed, line breaks are kept, and an empty optional field
becomes `None`. It then returns a `ValidationError` for:

- an empty required field;
- an invalid format: CAS and CE numbers (with their check digit), formulas, GHS
  hazard and precautionary references, CMR codes, HTML colors, `InChI`,
  `InChIKey` and SMILES;
- a number out of range, e.g. a negative quantity, or a quantity without its unit;
- dates before the entry date of a storage, or a store location that is its own
  parent.

The error names the field, e.g. `tag_label: empty input`. Some fields are also
normalized: a `Name` label is uppercased, an `EmpiricalFormula` label is stored
in the Hill order, and an email is lowercased (see below). Nested records
without an id are validated too; records with an id are already in the database
and left unchanged.

A request may give a unit by its id only. `Product::validate_unit_types` and
`Storage::validate_unit_types` check, after `sanitize_and_validate`, that each
unit has the right type (e.g. a temperature unit for `unit_temperature`), with
a function returning the type of a unit id from the database.

## Searchable types

The types made of an id and a text, each stored in its own table, implement
`chimitheque_traits::searchable::Searchable`, so the database code searches and
creates them with the same functions. Their text field is their label, except
for the hazard and precautionary statements, searched by their reference:

`CasNumber`, `CeNumber`, `Category`, `ClassOfCompound`, `EmpiricalFormula`,
`HazardStatement`, `LinearFormula`, `Name`, `PhysicalState`,
`PrecautionaryStatement`, `Producer`, `SignalWord`, `Supplier`, `Symbol` and
`Tag`.

## Account email normalization

`Person::sanitize_and_validate()` applies this explicit policy:

- Trim surrounding whitespace, but never collapse whitespace inside an address.
- Accept a bare mailbox only; reject display names (`Alice <alice@example.com>`)
  and angle-bracket wrappers (`<alice@example.com>`).
- Lowercase the entire address, including the local part, using Rust's Unicode
  lowercasing. This is an application-level account identity policy, even though
  email local parts can technically be case-sensitive.
- Preserve plus tags, quotes, and significant whitespace inside quotes. Do not
  apply IDNA conversion or provider-specific rules such as removing dots or plus tags.
- Retain support for single-label domains and domain literals. Validation checks
  syntax, not DNS existence, deliverability, or mailbox ownership.
- Leave the original value unchanged if validation fails. Repeated successful
  normalization is idempotent.

For example, ` User+Lab@EXAMPLE.COM ` becomes `user+lab@example.com`, while
`"A  B"@EXAMPLE.COM` becomes `"a  b"@example.com` without changing the quoted spaces.
Normalization is explicit; Serde deserialization alone does not apply it.

Consumers using emails for login or uniqueness must apply the same lowercasing
policy and check for collisions before normalizing existing stored addresses.
Unlike generic string cleaning, this policy does not collapse quoted whitespace.
Display-name inputs are rejected.

## PubChem parsing

`PubchemProduct::from_pubchem_json` returns
`Result<PubchemProduct, chimitheque_types::error::PubchemParseError>`: invalid
JSON is an error, never an empty product.

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
- Missing optional paths remain `None`. Scalar fields select the first
  match, but all matched values are checked. Extraction fails as a whole rather
  than returning a partially populated product.

This is a selected-field extractor, not a complete PubChem schema validator.
Missing paths (including unsupported intermediate structures) are treated as
absent.
