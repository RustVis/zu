// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use yew::{function_component, html, Html};
use zu::breadcrumbs::Breadcrumbs;
use zu::link::{Link, Underline};
use zu::styles::color::Color;
use zu::typography::Typography;

use crate::components::demo_box::DemoBox;

#[function_component(BreadcrumbsPage)]
pub fn breadcrumbs_page() -> Html {
    html! {
        <div class="container">
        <h1>{"Breadcrumbs"}</h1>

        <h2>{"Basic breadcrumbs"}</h2>
        <DemoBox>
            <Breadcrumbs aria_label="breadcrumbs">
                <Link underline={Underline::Hover}
                    color={Color::Inherit}
                    href="/">
                    {"Zu"}
                </Link>
                <Link underline={Underline::Hover}
                    color={Color::Inherit}
                    href="/material-ui/getting-started/installation/">
                    {"Core"}
                </Link>
                <Typography style="color: var(--zu-typography-text-primary);">
                    {"Breadcrumbs"}
                </Typography>
            </Breadcrumbs>
        </DemoBox>

        </div>
    }
}
