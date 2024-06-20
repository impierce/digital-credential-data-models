## Macro derive crate

### GenPaths

To see what kind of paths will be generated, look at the `impierce-mapper` crate which will generate paths for all types.
To see what kind of code the macro's generate (requires `cargo expand`) go to a crate for instance:
```shell
cd crates/types-elm
cargo expand > test.rs
```

And now you can open the test.rs file with your editor of choice to see all the rust code the macro will generate.

This macro will implement the `AddSchemaTypes` trait that can be found in the `types_common` crate.

On the bottom of the traits.rs file there are empty implementations of the trait on a bunch of types:

```rust

// Implement empty traits impl for all types
macro_rules! impl_T {
    (for $($t:ty),+) => {
        $(impl AddSchemaTypes for $t {
            fn add_schema_types(_data: &mut Vec<SchemaData>) {}
        })*
    }
}

impl_T!(for usize, u8, u16, u32, u64, u128);
impl_T!(for isize, i8, i16, i32, i64, i128);
impl_T!(for f32, f64);
impl_T!(for String, bool, PathBuf);
impl_T!(for Utc, NaiveDate, EmailAddress);
impl_T!(for serde_json::Value);
```

In case you encounter types that should have a default impl as well please add there.

The GenPaths macro will only add the last argument of a path as a type. 
For example: 

```rust
 pub struct Dummy {
    pub person: Arc<Mutex<Box<Person>>>,
 }
```

Will add Person to the SchemaData and ignore Arc, Mutex and Box.


#### CSV Data

Output of the CSV from the AddSchemaTypes can look like this:

| source schema | field name | target schema | multiplicity | required |
|---------------|------------|---------------|--------------|----------|
| Amount        | id         | UriType       | 1            | false    |
| Amount        | value      | f32           | 1            | true     |
| Amount        | type       | AmountTag     | 1            | true     |

- Column 1: source schema
- Column 2: field name
- Column 3: target schema (in case this never starts as source you now its the end)
- Column 4: multiplicity (1, *, 1 | *), for a single, an array or both is allowed
- Column 5: required, Is this field required for the schema 

#### Enums

Enums will have add empty field names and choices: 

```csv
AgentOrPersonOrOrganization, , Agent, 1, true  
AgentOrPersonOrOrganization, , Person, 1, true  
AgentOrPersonOrOrganization, , Organization, 1, true  
```
