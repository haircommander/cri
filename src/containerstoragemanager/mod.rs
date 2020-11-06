//! Manager of On-disk Storage of Containers
//! This package was heavily based on https://github.com/containers/storage/blob/master/store.go
//! Credit goes to the authors of containers/storage

use anyhow::Result;
use bitflags::bitflags;
use derive_builder::Builder;
use getset::Getters;
use std::collections::HashMap;
use std::fmt;
use std::path::PathBuf;

#[derive(Builder)]
#[builder(pattern = "owned", setter(into))]

/// This is the main data structure for a container store. The implementation `T` can vary and is being
/// defined in the `Store` trait. Responsibility of the `ContainerStorageManager` is the track the
/// necessary information for the implementation and not modify it in anyway.
pub struct ContainerStorageManager<T>
where
    T: Default,
{
    data: ContainerStorageData,

    #[builder(default = "T::default()")]
    /// Trait implementation for creating the sandbox.
    implementation: T,
}

#[derive(Builder, Getters)]
#[builder(pattern = "owned", setter(into, strip_option))]
/// ContainerStorageData holds all the data which will be passed to the `Store` trait.
pub struct ContainerStorageData {
    #[get = "pub"]
    ///	run_root is the filesystem path under which we can store run-time
	/// information, such as the locations of active mount points, that we
	/// want to lose if the host is rebooted.
    run_root: String,

    #[get = "pub"]
    /// graph_root is the filesystem path under which we will store the
	/// contents of layers, images, and containers.
    graph_root: String,

    #[get = "pub"]
    /// rootless_storage_path is the storage path for rootless users
	/// default $HOME/.local/share/containers/storage
    rootless_storage_path: String,

    #[get = "pub"]
    /// graph_driver_name is the underlying storage driver that we'll be
	/// using.  It only needs to be specified the first time a Store is
	/// initialized for a given RunRoot and GraphRoot.
    graph_driver_name: String,

    #[get = "pub"]
    /// graph_driver_name is the underlying storage driver that we'll be
	/// using.  It only needs to be specified the first time a Store is
	/// initialized for a given RunRoot and GraphRoot.
    graph_driver_options: vec![],

    /// TODO userns support
}

pub trait Store {
    /// Run a previously created sandbox.
    fn run(&mut self, _: &SandboxData) -> Result<()> {
        Ok(())
    }

    /// Stop a previously started sandbox.
    fn stop(&mut self, _: &SandboxData) -> Result<()> {
        Ok(())
    }

    /// Remove a stopped sandbox.
    fn remove(&mut self, _: &SandboxData) -> Result<()> {
        Ok(())
    }

    // Returns whether a sandbox is ready or not. A sandbox should be `ready()` if running, which
    // means that a previous call to `run()` was successful and it has not been neither `stopped()`
    // nor already `removed()`.
    fn ready(&mut self, _: &SandboxData) -> Result<bool> {
        Ok(false)
    }
}
