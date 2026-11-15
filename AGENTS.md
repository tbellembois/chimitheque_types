# AGENTS.md - Chimithèque Types Library Agent Guide

This file helps AI agents and coding assistants understand the structure, patterns, and conventions of the `chimitheque_types` Rust library.

## 📋 Project Overview

**Project Name**: `chimitheque_types`
**Crates.io**: Not published (private library for Chimithèque application)
**GitHub**: https://github.com/tbellembois/chimitheque_types

**Purpose**: Chemical, biological, and cosmetic product database type definitions for the Chimithèque chemical inventory management system.

**Domain**: Chemistry, biology, toxicology, regulatory compliance (CAS numbers, hazard statements, EC numbers, regulatory symbols, etc.)


### Project Structure

```
chimitheque_types/
├── Cargo.toml          - Project configuration and dependencies
├── src/                - Main source code (43 modules)
│   ├── lib.rs          - Module declarations
│   ├── error.rs        - Error types (ParseError enum)
│   ├── requestfilter.rs - Query/filtering system for API endpoints
│   ├── pubchemproduct.rs - PubChem API integration (CAS number lookup, hazard statements parsing)
│   ├── product.rs      - Core Product type with validation
│   ├── person.rs       - User/identity management with email validation
│   ├── name.rs         - Product name handling with searchability
│   └── ... (40+ modules for chemical safety data)
├── src/testdata/      - Test data files (PubChem API responses)
└── tests/             - Integration tests (if any)
```

## 🎯 Core Purpose

`chimitheque_types` provides **strongly-typed data structures** for managing:
- Chemical products and substances
- Suppliers and producers
- Hazard classifications (GHS, CMR, symbols)
- Storage and inventory tracking
- Product identifiers (CAS, EC, PubChem CID)
- Regulatory compliance data
- Chemical formulas and physical states
- User permissions and access control

**Analogy**: This is the "nouns and verbs" vocabulary for a chemical inventory management system.

## 🗂️ Key Modules & Responsibilities

| Module | Purpose | Data Structure | Key Methods |
|--------|---------|----------------|------------|
| `product.rs` | Core chemical product representation | `Product` struct | `sanitize_and_validate()` |
| `casnumber.rs` | CAS Registry Number validation (Chemical Abstracts Service) | `CasNumber` struct | `sanitize_and_validate()`, format validation |
| `cenumber.rs` | EC Number validation (European Community number) | `CeNumber` struct | `sanitize_and_validate()` |
| `person.rs` | User identity and authentication | `Person` struct | `sanitize_and_validate()`, email parsing |
| `name.rs` | Product name handling and searchability | `Name` struct (implements `Searchable`) | `sanitize_and_validate()`, search indexing |
| `error.rs` | Centralized error definitions | `ParseError` enum | Implement `Display`, `Error` traits |
| `requestfilter.rs` | REST API query parameter parsing | `RequestFilter` struct | `try_from<&str>`, `Default` implementation |
| `pubchemproduct.rs` | PubChem API response parsing | `PubchemProduct` struct | `from_pubchem_json()`, safe parsing without panics |
| `hazardstatement.rs` | GHS hazard statement handling | `HazardStatement` struct | Regex-based parsing, symbol extraction |
| `precautionarystatement.rs` | GHS precautionary statements | `PrecautionaryStatement` struct | Regex-based parsing |
| `symbol.rs` | GHS pictogram/symbol handling | `Symbol` type | String parsing with regex |
| `empiricalformula.rs` | Chemical formula parsing | `EmpiricalFormula` struct | Formula validation, molecular weight calculation |
| `linearformula.rs` | SMILES and structural formulas | `LinearFormula` struct | Parser based on chemical grammar |
| `physicalstate.rs` | Physical state classification | enum with variants for solid/liquid/gas/etc. | Predefined constants |
| `signalword.rs` | GHS signal words (Danger/Warning) | enum | Parse from string |
| `category.rs` | Product category classification | `Category` struct | Taxonomy of product types |
| `storage.rs` & `storelocation.rs` | Inventory management | Structs with location tracking | Storage optimization data |
| `supplier.rs` & `producer.rs` | Supply chain management | Entity-like structs | Reference validation |
| `tag.rs` | Flexible categorization | `Tag` struct | Free-form tagging system |
| `bookmark.rs` | User bookmarking | `Bookmark` struct | Cross-reference to products/users |

## 🏗️ Architecture Patterns

### Validation Pattern

**All structs follow this validation lifecycle:**

```rust
pub struct ChemicalType {
    // Required fields
    pub id: Option<u64>,  // None = not yet persisted
    
    // Optional validated fields
    pub field_name: Option<String>,
}

impl ChemicalType {
    /// Sanitize and validate all fields
    /// - Removes unsafe characters
    /// - Validates format (email, CAS, EC, etc.)
    /// - Returns Result<(), Error>
    pub fn sanitize_and_validate(&mut self) -> Result<(), Box<dyn Error + Send + Sync>> {
        // Implementation validates nested structures
        if let Some(field) = &mut self.field_name {
            field = &clean(field, Transform::Uppercase); // Removes leading/trailing whitespace
            if field.is_empty() {
                return Err(Box::new(ParseError::EmptyInput));
            }
        }
        Ok(())
    }
}
```

**Key validation rules:**
- Email addresses: Must be valid format via `email_address` crate
- CAS numbers: Regex pattern matching `[0-9]{2,7}-[0-9]` with checksum validation
- EC numbers: Format `([0-9]{3}-)?[0-9]{3}-[0-9]`
- Names: Non-empty after trimming and uppercase normalization
- IDs: Positive integers (0 is reserved for "not set")

### Error Handling

**Error design principles:**
1. Use `ParseError` enum in `error.rs` for all library-specific errors
2. Wrap external errors in `Box<dyn Error + Send + Sync>`
3. Never use `unwrap()` or `expect()` in production code (see [fixes made to pubchemproduct.rs])
4. Use `?` operator for error propagation
5. Keep error messages user-friendly and actionable

### Type Safety

**Domain-specific types:**
- `CasNumber`: Ensures valid CAS registry format
- `CeNumber`: Ensures valid EC number format
- `PhysicalState`: Enum instead of string (SOLID, LIQUID, GAS, PLASMA, etc.)
- `HazardStatement`: Valid GHS statement codes (H200-H399 series)
- `Symbol`: Valid GHS pictogram codes

### External Dependencies

```toml
[dependencies]
chrono = { version = "0.4", features = ["serde", "alloc"] }
email_address = { version = "0.2", default-features = false }
jsonpath-rust = "0.3.5"    # PubChem JSON navigation
regex = "1.13"             # Input validation
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"          # JSON parsing
url = "2.5"                # URL parameter parsing

chimitheque_traits = { git = "https://github.com/tbellembois/chimitheque_traits.git" }
chimitheque_utils = { git = "https://github.com/tbellembois/chimitheque_utils.git" }
```

**External package responsibilities:**
- `chimitheque_traits`: Common traits used across projects (`Searchable`, validators, etc.)
- `chimitheque_utils`: String utilities, cleaning, transformations
- `chimitheque_defines`: Regex patterns and constants

## 🔍 Development Workflows

### Common Coding Tasks

#### 1. Adding a New Chemical Attribute

```bash
# Example: Adding a new `molecular_weight` field to Product
git checkout -b feat/add-molecular-weight-validation

# 1. Add field to Product struct in product.rs
# 2. Add validation in sanitize_and_validate()
# 3. Add tests in product_tests.rs
# 4. Update any structs that embed Product

cargo build
cargo test
```

#### 2. Adding a New Regex Pattern

```bash
# Update chimitheque_defines crate with new regex
# Examples: SPECIFIC_CHEMICAL_REGEX, new hazard statement patterns

# For PubChem parsing enhancements:
# Update pubchemproduct.rs with new safe_json_query() pattern
```

#### 3. API Query Parameter Support

```rust
// In requestfilter.rs, add new parameter:
std::borrow::Cow::Borrowed("new_filter") => {
    // Validate using: if !VALIDATION_REGEX.is_match(&value) { return Err(...) }
    request_filter.new_field = Some(value.to_string());
    Ok(modified_request_filter)
}
```

### Validation Checks

**Always add these checks:**
- Field is not empty after sanitization
- Format matches expected pattern (email, CAS, EC, etc.)
- Numeric ranges are valid (e.g., molecular_weight > 0)
- Cross-references exist (e.g., if cas_number is Some, verify it's in the database)

## 📝 Testing Strategy

### Testing Patterns

**Module structure:**
```
src/
  product.rs          - Main implementation
  product_tests.rs    - Unit tests module ([[path]] attribute)
```

**Test contents:**
- `sanitize_and_validate()` tests for valid/invalid inputs
- Parsing tests for structured formats (JSON, URLs)
- Edge cases (empty strings, NULL values, malformed data)
- Regex pattern matching tests

**Example test pattern:**
```rust
#[test]
fn test_cas_number_validation() {
    let mut cas = CasNumber {
        cas_number_label: "123-45-6".to_string(),
        ..Default::default()
    };
    
    assert!(cas.sanitize_and_validate().is_ok());
    
    cas.cas_number_label = "".to_string();
    assert!(cas.sanitize_and_validate().is_err()); // Empty input error
}
```

### Test Data Sources

- **src/testdata/pubchem_*.json**: Real PubChem API responses for testing parsing logic
- Test edge cases (missing fields, malformed JSON, partial data)

### Contract Testing

When updating `chimitheque_traits` or consuming libraries:
- Check that trait implementations haven't changed
- Verify all trait bounds and method signatures
- Test integration with external types

## ⚠️ Critical Anti-Patterns

### Never Use These in Production Code

❌ **Don't:**
```rust
.unwrap()           // Panics on error
.expect("message")  // Panics with message
.match .unwrap()    // Even in "validated" code paths
```

✅ **Do:**
```rust
match value {
    Some(v) => process(v),
    None => return Err("expected Some value".into()),
}

// OR for parsing:
if let Ok(parsed) = str.parse() {
    // use parsed
}

// OR for regex captures:
if let Some(cap) = regex.captures(input) {
    if let Ok(num) = cap["id"].parse::<u64>() {
        // process
    }
}
```

**Applied fixes:**
- Fixed `pubchemproduct.rs`: Replaced 23 `unwrap()` calls with safe patterns
- Fixed `requestfilter.rs`: Replaced 6 `unwrap()` calls in TryFrom implementation

### Type Safety Violations

❌ **Stringly-typed APIs:**
```rust
pub person_email: String  // Not validated

// Instead, use EmailAddress type or validate in setter
```

✅ **Safe alternatives:**
```rust
pub person_email: EmailAddress  // Validated at construction

// OR in Person::sanitize_and_validate():
impl Person {
    pub fn sanitize_and_validate(&mut self) -> Result<(), _> {
        self.person_email = clean(&self.person_email, Transform::Lowercase);
        EmailAddress::parse(&self.person_email)?;
        Ok(())
    }
}
```

### State Validation

❌ **Missing validation:**
```rust
// Bad: No validation of field relationships
let mut product = Product {
    hazard_statements: Some(vec!["H200".to_string()]),
    signal_word: Some(SignalWord::Warning),  // Inconsistent! H200 requires Danger
    ..Default::default()
};
```

✅ **Consistency checks:**
```rust
impl Product {
    pub fn validate_consistency(&self) -> Result<(), _> {
        if let (Some(hs), Some(sw)) = (&self.hazard_statements, &self.signal_word) {
            // Check: Serious hazards require Danger signal word
            let has_serious = hs.iter().any(|h| h.starts_with('H2') || h.starts_with('H3'));
            if has_serious && *sw == SignalWord::Warning {
                return Err("Serious hazards require Danger signal word".into());
            }
        }
        Ok(())
    }
}
```

## 🛠️ Tools & Workflow

### Essential Commands

```bash
# Build and test
cargo build --all-features
cargo test --all-targets

# Code quality
cargo clippy --all-targets --all-features
cargo fmt --all

# Coverage (already configured)
cargo tarpaulin --out Html

# Update dependencies
cd ../chimitheque_traits && git pull
cd ../chimitheque_utils && git pull
```

### Development Setup

```bash
# Clone all related repositories
cd ~/workspace/workspace_rust
git clone https://github.com/tbellembois/chimitheque_types.git
git clone https://github.com/tbellembois/chimitheque_traits.git
git clone https://github.com/tbellembois/chimitheque_utils.git

# Patch local dependencies (as configured in Cargo.toml)
cargo build
```

### Branch Strategy

- `main`: Production-ready code, deployed to users
- `dev`: Development branch, merges approved via PR
- Feature branches: `feat/...`, `fix/...` based on `main`

### Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):
- `feat:` prefix for new features
- `fix:` prefix for bug fixes
- `docs:` for documentation changes
- `refactor:` for code improvements without behavior change
- `test:` for test additions
- `chore:` for build system, dependency updates

## 🤖 AI Agent Instructions

### Working with chimitheque_types

**Understand before editing:**
1. ✅ Read `AGENTS.md` (this file)
2. ✅ Review the module's purpose from the table above
3. ✅ Check the existing test patterns
4. ✅ Run `cargo test` to establish baseline

**When making changes:**
1. 🔍 First, run `cargo clippy` to see if there are obvious issues
2. 📝 Add or update tests for new functionality
3. 🔒 Never add `.unwrap()` or `.expect()` - ask if unsure
4. ✍️ Add `///` documentation to all public items
5. ⚠️ Update `AGENTS.md` if you discover new patterns

**For module-specific guidance:**
- Need to parse numbers? → `use regex::Regex; try_parse_isize()`
- Need to validate email? → `email_address::EmailAddress::parse()`
- Adding string fields? → Ensure they're trimmed and uppercased
- Working with dates? → Use `chrono::NaiveDate`

### Common AI Tasks

#### Task: Add a New Product Field

```markdown
Steps for AI agent:
1. Identify which module owns the data (likely product.rs or a related module)
2. Add field to the appropriate struct
3. Add field to serialization (derive Serialize, Deserialize)
4. Implement sanitization and validation in `sanitize_and_validate()`
5. Add tests for valid and invalid inputs
6. Update any modules that embed the modified struct
7. Bump minor version in Cargo.toml (but you don't have access)

Checklist:
[ ] Field added to struct
[ ] Validation implemented
[ ] Tests added/updated
[ ] No unwrap() calls added
[ ] clippy is happy
```

#### Task: Refactor Error Handling

```markdown
Preferred pattern:

OLD (bad):
```rust
let json: Value = serde_json::from_str(s).unwrap();
```

NEW (good):
```rust
let json: Value = match serde_json::from_str(s) {
    Ok(v) => v,
    Err(_) => return Err("Invalid JSON".into()),
};

// OR even simpler if returning Option:
let json: Option<Value> = serde_json::from_str(s).ok();
```

Refactor goals:
- [ ] Replace unwrap() with match/if-let
- [ ] Replace expect() with proper error handling
- [ ] Ensure all error messages are user-friendly
- [ ] Keep error type ergonomics (Box<dyn Error + ...>)
```

#### Task: Add Regex Pattern

```markdown
Steps:
1. Prefer defining patterns in `chimitheque_defines` crate
2. Add to the appropriate regex!() macro or const
3. Test pattern with sample inputs:
   - Valid cases
   - Edge cases
   - Malformed inputs
4. Update any modules using the regex

Example pattern types:
- CAS: `^(?:\d{2,7}-\d{1,2}|(?:\d{1,7}))$"`
- EC: `^(\d{3}-\d{3}-\d{1}|\d{3}-\d{5}-\d{1})$`
- GHS Hazard: `^H\d{3}$`

Safety note: Use `Regex::new()` at struct initialization or lazy_static!
```

### Knowledge Base

**Keep these references handy:**

- [Rust Book](https://doc.rust-lang.org/book/)
- [Serde Documentation](https://serde.rs/)
- [Regex Patterns for Chemistry](https://www.cas.org/support/documentation/chemical-substances)
- [GHS Hazard Statements](https://en.wikipedia.org/wiki/Globally_Harmonized_System_of_Classification_and_Labelling_of_Chemicals#Hazard_statements)
- [Chimithèque Application](https://github.com/tbellembois/chimitheque) (the consumer app)

## 🔬 Chemistry-Specific Gotchas

### CAS Numbers

**Properties:**
- Format: `12345-67-8` (up to 7 digits before dash)
- Last digit is a checksum: `(d1 x 1 + d2 x 2 + ... + dn x n) mod 10`
- Leading zeros are significant

**Example validation:**
```rust
// In casnumber.rs
lazy_static! {
    static ref CAS_REGEX: Regex = Regex::new(r"^\d{2,7}-\d$").unwrap();
}

fn validate_checksum(cas: &str) -> bool {
    let parts: Vec<&str> = cas.split('-').collect();
    // Calculate checksum...
    // Return true if valid
}
```

### EC Numbers

**Format:** `XXXX-XXXX-X` where:
- First part is optional assignment number
- Second part is substance identifier
- Last digit is checksum

### Hazard Statements

**Categories:**
- `H2XX`: Physical hazards
- `H3XX`: Health hazards
- `H4XX`: Environmental hazards
- Example: `H225` = "Highly flammable liquid and vapor"

**Formats:**
- Single statement: `H225`
- Multiple: `H225, H315, H319`

### Storage Compatibility

**Chemical incompatibilities:**
- Acids vs Bases
- Oxidizers vs Reducing agents
- Water-reactive chemicals

**Never hardcode policies in this library** - store compatibility matrices in database via product.tags or related tables.

## 🎓 Quick Reference

### Common Struct Patterns

**Type 1: Value Object (with validation)**
```rust
#[derive(Default, Serialize, Deserialize, Clone)]
pub struct SpeciesName {
    pub name_id: Option<u64>,
    pub name_label: String,
}

impl SpeciesName {
    pub fn sanitize_and_validate(&mut self) -> Result<(), _> {
        self.name_label = clean(&self.name_label, Transform::Uppercase);
        if self.name_label.is_empty() {
            return Err(ParseError::EmptyInput);
        }
        Ok(())
    }
}
```

**Type 2: Reference (entity ID only)**
```rust
#[derive(Default, Serialize, Deserialize, Clone)]
pub struct EntityId {
    pub entity_id: u64,  // Must be > 0, never None
}

impl EntityId {
    /// Always valid if created properly
    pub fn new(id: u64) -> Result<Self, _> {
        if id == 0 {
            return Err("Entity ID cannot be zero".into());
        }
        Ok(Self { entity_id: id })
    }
}
```

**Type 3: Option Wrapper (conditional fields)**
```rust
#[derive(Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct OptionalField<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<T>,
    pub is_set: bool,
}

impl<T> OptionalField<T> {
    // Implementation...
}
```

### Helper Functions

**String Cleaning:**
```rust
use chimitheque_utils::string::{Transform, clean};

fn clean_string(input: &str) -> String {
    clean(input, Transform::Uppercase)  // Uppercase, trim, no leading/trailing spaces
}
```

**Validation Macros:**
```rust
#[macro_export]
macro_rules! ensure {
    ($cond:expr, $err:expr) => {
        if !$cond {
            return Err($err.into());
        }
    };
}
```

## 📚 Additional Resources

- **[API Documentation (generated)]**: Run `cargo doc --open`
- **[Chimithèque Application]**]: The consumer of this library
- **[PubChem REST API]**]: Data source for chemical properties
- **[GHS Classification Manual]**]: Hazard statement definitions

---

**Update History:**
- v1.0 - Initial version documenting patterns in chimitheque_types v0.1.0
- v1.1 - Added fixes for unwrap() patterns in pubchemproduct.rs and requestfilter.rs
- v1.2 - Added module responsibility table and architecture diagrams

Last updated: 2024
