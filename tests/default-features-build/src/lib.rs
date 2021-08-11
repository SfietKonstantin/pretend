//! Tests for default features
//!
//! This crate tests if the default features (with Send + Sync errors) works well.
//! It compiles all client crates that supports errors with Send and Sync bounds,
//! to verify that they effectively supports these bounds. It also compiles a test
//! client with an error that does not support these bounds, to make sure that it
//! fails to compile.
