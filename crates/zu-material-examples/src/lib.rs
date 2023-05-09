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
// For <Link> and <BrowserRouter>
#![allow(clippy::let_underscore_untyped)]

mod app;
mod components;
mod router;
mod views;

use app::App;

pub fn run_main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("hello");
    yew::Renderer::<App>::new().render();
}
