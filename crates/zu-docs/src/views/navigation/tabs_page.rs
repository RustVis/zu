// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

mod basic_tabs;

use yew::{function_component, html, Html};

use crate::components::demo_box::DemoBox;
use basic_tabs::BasicTabs;

#[function_component(TabsPage)]
pub fn tabs_page() -> Html {
    html! {
        <div class="container">

        <h1>{"Tabs"}</h1>
        <p>{"Tabs organize and allow navigation between groups of content that are related
            and at the same level of hierarchy."}</p>

        <h2>{"Basic tabs"}</h2>
        <p>{"A basic example with tab panels."}</p>
        <DemoBox>
            <BasicTabs />
        </DemoBox>
        </div>
    }
}
