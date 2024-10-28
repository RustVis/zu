// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

use stylist::Style;
use yew::{classes, function_component, html, Children, Html, Properties};
use zu::paper::Paper;
use zu::stack::Stack;
use zu::styles::flex_direction::FlexDirection;
use zu::styles::spacing::Spacing;

use crate::components::demo_box::DemoBox;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub children: Children,
}

#[function_component(Item)]
pub fn item(props: &Props) -> Html {
    let style = Style::new(
        r"
        padding: var(--zu-spacing-xs);
        text-align: center;
        color: var(--zu-palette-text-secondary);
        background-color: white;
    ",
    )
    .unwrap();
    html! {
        <Paper classes={classes!(style.get_class_name().to_owned())}>
        {props.children.clone()}
        </Paper>
    }
}

#[function_component(StackPage)]
pub fn stack_page() -> Html {
    html! {
        <div class="container">
        <h1>{"Stack"}</h1>

        <h2>{"Basics"}</h2>
        <p>{"The Stack component acts as a generic container, wrapping around the elements to be arranged."}</p>
        <DemoBox>
            <Stack spacing={Spacing::MiddleNudge}>
                <Item>{"Item 1"}</Item>
                <Item>{"Item 2"}</Item>
                <Item>{"Item 3"}</Item>
            </Stack>
        </DemoBox>

        <h2>{"Stack v.s. Grid"}</h2>
        <p>{"Stack is concerned with one-dimensional layouts,
          while Grid handles two-dimensional layouts.
          The default direction is column which stacks children vertically."}</p>

        <h2>{"Direction"}</h2>
        <p>{"By default, Stack arranges items vertically in a column.
         Use the direction prop to position items horizontally in a row:"}</p>

        <DemoBox>
            <Stack direction={FlexDirection::Row} spacing={Spacing::Small}>
                <Item>{"Item 1"}</Item>
                <Item>{"Item 2"}</Item>
                <Item>{"Item 3"}</Item>
            </Stack>
        </DemoBox>

        <h2>{"Dividers"}</h2>
        <p>{"Use the divider prop to insert an element between each child.\
         This works particularly well with the Divider component, as shown below:"}</p>
        <DemoBox>
            <Stack direction={FlexDirection::Row} spacing={Spacing::Small}>
                <Item>{"Item 1"}</Item>
                <Item>{"Item 2"}</Item>
                <Item>{"Item 3"}</Item>
            </Stack>
        </DemoBox>

        </div>
    }
}
