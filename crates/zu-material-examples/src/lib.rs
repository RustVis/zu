// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

#![deny(
    warnings,
    clippy::all,
    clippy::cargo,
    clippy::nursery,
    clippy::pedantic
)]
// wasi v0.10 and v0.11
#![allow(clippy::multiple_crate_versions)]
// For <Link> and <BrowserRouter>
#![allow(clippy::let_underscore_untyped)]

mod app;
mod components;

use app::App;

pub fn run_main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
