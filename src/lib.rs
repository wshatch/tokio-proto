//! Tokio is a network application framework for rapid development and highly
//! scalable production deployments of clients and servers.
//!
//! Tokio consists of multiple layers.
//!
//! # Service
//!
//! At a high level, Tokio provides a `Service` trait which provides a unified
//! API for writing clients and servers as well as the ability to build
//! reusable middleware components.
//!
//! The service trait is decoupled from any notion of a runtime.
//!
//! # Protocol building blocks
//!
//! Tokio aims to provide all the pieces necessary for rapidly developing
//! protocol implementations. These components exist in the `proto` module.

#![deny(warnings, missing_docs)]

extern crate bytes;
extern crate futures;
extern crate slab;
extern crate take;
extern crate tokio_core;
extern crate tokio_service;

#[macro_use]
extern crate log;

pub mod io;
pub mod proto;
pub mod server;

mod service;

pub use self::service::{Service, NewService, SimpleService, simple_service};
