// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use yew::prelude::*;
use yew_router::prelude::{use_location, Link, Routable};

use crate::router::Route;

#[function_component(LeftPanel)]
pub fn left_panel() -> Html {
    let location = use_location().unwrap();
    let location_path = location.path();

    let link_cls = |r: Route| -> &'static str {
        let route_path = r.to_path();
        if route_path == location_path {
            "nav-link active"
        } else {
            "nav-link"
        }
    };

    html! {
        <div class="col-sm-4 col-lg-2">
            <ul class="nav nav-pills flex-column mb-auto bg-light">
                <li class="nav-item">
                    <Link<Route> to={ Route::Home } classes={ link_cls(Route::Home) }>
                        { "Home" }
                    </Link<Route>>
                </li>

                <li class="nav-item">
                    <Link<Route> to={ Route::Skeleton } classes={ link_cls(Route::Skeleton) }>
                        { "Skeleton" }
                    </Link<Route>>
                </li>
            </ul>
        </div>
    }
}