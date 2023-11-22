// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

use yew::{function_component, html, Html};
use zu::boxed::Box;
use zu::container::Container;

use crate::components::demo_box::DemoBox;

#[function_component(ContainerPage)]
pub fn container_page() -> Html {
    let box_elm = html! {<Box style="background-color: #cfe8fc; height: 100vh" />};

    html! {
        <div class="container">
        <h1>{"Container"}</h1>
        <p>{"While containers can be nested, most layouts do not require a nested container."}</p>

        <h2>{"Fluid"}</h2>
        <p>{"A fluid container width is bounded by the maxWidth prop value."}</p>
        <DemoBox>
            <Container>
                {box_elm.clone()}
            </Container>
        </DemoBox>

        <h2>{"Disable gutters"}</h2>
        <DemoBox>
            <Container disable_gutters={true}>
                {box_elm.clone()}
            </Container>
        </DemoBox>

        <h2>{"Fixed"}</h2>
        <p>{"If you prefer to design for a fixed set of sizes instead of trying to accommodate a fully fluid viewport,
         you can set the fixed prop.
         The max-width matches the min-width of the current breakpoint."}</p>
        <DemoBox>
            <Container fixed={true}>
                {box_elm.clone()}
            </Container>
        </DemoBox>

        </div>
    }
}
