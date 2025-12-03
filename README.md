# Oversized shapefile reader
Rust library based on (https://github.com/tmontaigu/shapefile-rs) which allows
for reading .shp files > 2GB in size. These shapefiles are not spec compliant
as the file size cannot be stored in the header due to the 32bit restriction.
The shx support and writing support has been removed, and the file size is
now taken from the OS instead of the header field.
Additionally, the .shx files cannot be used, since the indexing cannot point
to a location past the 32bit limit. Thus, no indexing functionality is 
supported.

# shapefile-rs
Rust library to read & write shapefiles
.dbf files supported via the [dbase](https://crates.io/crates/dbase) crate

```rust
let mut reader = shapefile::Reader::from_path(filename).unwrap();

for result in reader.iter_shapes_and_records() {
    let (shape, record) = result.unwrap();
    println ! ("Shape: {}, records: ", shape);
    for (name, value) in record {
        println ! ("\t{}: {:?}, ", name, value);
    }
    println ! ();
}
```
You can check out examples in the [examples](https://github.com/tmontaigu/shapefile-rs/tree/master/examples/) folder

