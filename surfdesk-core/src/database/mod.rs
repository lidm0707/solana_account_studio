//! # Database Module
//!
//! This module provides database integration for the SurfDesk application.
//! It includes schema definitions, migrations, and database service
//! implementations using Diesel ORM with SQLite backend.

pub mod migrations;
pub mod schema;
pub mod connection;
