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
                    <Link<Route> to={Route::Home} classes={link_cls(Route::Home)}>
                        {"Home"}
                    </Link<Route>>
                </li>
                <li class="nav-item">
                    <Link<Route> to={Route::Alert} classes={link_cls(Route::Alert)}>
                    {"Alert"}
                    </Link<Route>>
                </li>
                <li class="nav-item">
                    <Link<Route> to={Route::Avatar} classes={link_cls(Route::Avatar)}>
                    {"Avatar"}
                    </Link<Route>>
                </li>
                <li class="nav-item">
                    <Link<Route> to={Route::Backdrop} classes={link_cls(Route::Backdrop)}>
                    {"Backdrop"}
                    </Link<Route>>
                </li>
                <li class="nav-item">
                    <Link<Route> to={Route::Badge} classes={link_cls(Route::Badge)}>
                    {"Badge"}
                    </Link<Route>>
                </li>
                <li class="nav-item">
                    <Link<Route> to={Route::BottomNavigation} classes={link_cls(Route::BottomNavigation)}>
                    {"Bottom Navigation"}
                    </Link<Route>>
                </li>
                <li class="nav-item">
                    <Link<Route> to={Route::Box} classes={link_cls(Route::Box)}>
                    {"Box"}
                    </Link<Route>>
                </li>
                <li class="nav-item">
                    <Link<Route> to={Route::Breadcrumbs} classes={link_cls(Route::Breadcrumbs)}>
                    {"Breadcrumbs"}
                    </Link<Route>>
                </li>
                <li class="nav-item">
                    <Link<Route> to={Route::Container} classes={link_cls(Route::Container)}>
                    {"Container"}
                    </Link<Route>>
                </li>
                <li class="nav-item">
                    <Link<Route> to={Route::Divider} classes={link_cls(Route::Divider)}>
                    {"Divider"}
                    </Link<Route>>
                </li>
                <li class="nav-item">
                    <Link<Route> to={Route::Icons} classes={link_cls(Route::Icons)}>
                    {"Icons"}
                    </Link<Route>>
                </li>
                <li class="nav-item">
                    <Link<Route> to={Route::MaterialIcons} classes={link_cls(Route::MaterialIcons)}>
                    {"Material Icons"}
                    </Link<Route>>
                </li>
                <li class="nav-item">
                    <Link<Route> to={Route::Paper} classes={link_cls(Route::Paper)}>
                    {"Paper"}
                    </Link<Route>>
                </li>
                <li class="nav-item">
                    <Link<Route> to={Route::Progress} classes={link_cls(Route::Progress)}>
                        {"Progress"}
                    </Link<Route>>
                </li>
                <li class="nav-item">
                    <Link<Route> to={Route::Skeleton} classes={link_cls(Route::Skeleton)}>
                        {"Skeleton"}
                    </Link<Route>>
                </li>
                <li class="nav-item">
                    <Link<Route> to={Route::Stack} classes={link_cls(Route::Stack)}>
                        {"Stack"}
                    </Link<Route>>
                </li>
                <li class="nav-item">
                    <Link<Route> to={Route::Typography} classes={link_cls(Route::Typography)}>
                        {"Typography"}
                    </Link<Route>>
                </li>


            </ul>
        </div>
    }
}
