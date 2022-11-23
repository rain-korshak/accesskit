// Copyright 2022 The AccessKit Authors. All rights reserved.
// Licensed under the Apache License, Version 2.0 (found in
// the LICENSE-APACHE file) or the MIT license (found in
// the LICENSE-MIT file), at your option.

#![deny(unsafe_op_in_unsafe_fn)]

mod appkit;
mod context;
mod node;

mod adapter;
pub use adapter::Adapter;

mod event;
pub use event::QueuedEvents;

mod subclass;
pub use subclass::SubclassingAdapter;
