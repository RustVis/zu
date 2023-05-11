// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use yew::prelude::*;
use yew_router::prelude::Routable;

use crate::views::home_page::HomePage;
use crate::views::progress_page::ProgressPage;
use crate::views::skeleton_page::SkeletonPage;
use crate::views::typography_page::TypographyPage;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Routable)]
pub enum Route {
    #[at("/")]
    Home,

    #[at("/Skeleton")]
    Skeleton,

    #[at("/progress")]
    Progress,

    #[at("/typography")]
    Typography,
}

#[must_use]
#[allow(clippy::cognitive_complexity)]
#[allow(clippy::let_unit_value)]
pub fn switch_route(routes: Route) -> Html {
    match routes {
        Route::Home => html! {<HomePage />},
        Route::Skeleton => html! {<SkeletonPage />},
        Route::Progress => html! {<ProgressPage />},
        Route::Typography => html! {<TypographyPage />},
    }
}
