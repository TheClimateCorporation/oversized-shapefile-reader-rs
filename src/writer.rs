//! Module with the definition of the [Writer] that allows writing shapefile
//!
//! # Writer
//!
//! [Writer] is the struct that writes a complete shapefile (_.shp_, _.shx_, _.dbf_).
//!
//! # ShapeWriter
//!
//! The [ShapeWriter] can be used if you only want to write the .shp
//! and .shx files, however since it does not write the .dbf file, it is not recommended.


pub(crate) fn f64_min(a: f64, b: f64) -> f64 {
    if a < b {
        a
    } else {
        b
    }
}

pub(crate) fn f64_max(a: f64, b: f64) -> f64 {
    if a > b {
        a
    } else {
        b
    }
}

