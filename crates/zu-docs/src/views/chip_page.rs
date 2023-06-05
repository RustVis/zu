// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use web_sys::MouseEvent;
use yew::{function_component, html, Callback, Html};
use zu::avatar::Avatar;
use zu::chip::{Chip, Variant};
use zu::stack::Stack;
use zu::styles::color::Color;
use zu::styles::direction::Direction;
use zu::styles::size::Size;
use zu::styles::spacing::Spacing;
use zuicon_material::Delete as DeleteIcon;
use zuicon_material::Done as DoneIcon;
use zuicon_material::Face as FaceIcon;

use crate::components::demo_box::DemoBox;

#[function_component(ChipPage)]
pub fn chip_page() -> Html {
    let handle_click = Callback::from(|event: MouseEvent| {
        event.prevent_default();
        log::info!("clicked");
    });

    let handle_delete = Callback::from(|()| {
        log::info!("on delete");
    });

    html! {
        <div class="container">
        <h1>{"Chip"}</h1>
        <p>{"Chips allow users to enter information, make selections, filter content, or trigger actions."}</p>

        <h2>{"Basic chip"}</h2>
        <p>{"The Chip component supports outlined and filled styling."}</p>
        <DemoBox>
        <Chip label={html!{"Chip Filled"}} />
        <Chip label={html!{"Chip Outlined"}} variant={Variant::Outlined} />
        </DemoBox>

        <h2>{"Chip actions"}</h2>
        <p>{"You can use the following actions."}</p>
        <ul>
            <li>{"Chips with the on_click prop defined change appearance on focus, hover, and click."}</li>
            <li>{"Chips with the onDelete prop defined will display a delete icon which changes appearance on hover."}</li>
        </ul>

        <h3>{"Clickable"}</h3>
        <DemoBox>
        <Chip label={html!{"Clickable"}} on_click={handle_click.clone()} />
        <Chip label={html!{"Clickable"}}
            variant={Variant::Outlined}
            on_click={handle_click.clone()} />
        </DemoBox>

        <h3>{"Deletable"}</h3>
        <DemoBox>
        <Chip label={html!{"Deletable"}} on_delete={handle_delete.clone()} />
        <Chip label={html!{"Deletable"}}
            variant={Variant::Outlined}
            on_delete={handle_delete.clone()} />
        </DemoBox>

        <h3>{"Clickable and deletable"}</h3>
        <DemoBox>
        <Chip
            label={html!{"Clickable Deletable"}}
            on_click={handle_click.clone()}
            on_delete={handle_delete.clone()}
            />
        <Chip
            label={html!{"Clickable Deletable"}}
            variant={Variant::Outlined}
            on_click={handle_click.clone()}
            on_delete={handle_delete.clone()}
            />
        </DemoBox>

        <h3>{"Clickable link"}</h3>
        <DemoBox>
        <Chip label={html!{"Clickable Link"}}
            component="a"
            href="#basic-chip"
            clickable={true} />
        <Chip label={html!{"Clickable Link"}}
            component="a"
            href="#basic-chip"
            variant={Variant::Outlined}
            clickable={true}
            />
        </DemoBox>

        <h3>{"Custom delete icon"}</h3>
        <DemoBox>
        <Chip
            label={html!{"Custom delete icon"}}
            on_click={handle_click.clone()}
            on_delete={handle_delete.clone()}
            delete_icon={html!{<DoneIcon />}}
            />
        <Chip
            label={html!{"Custom delete icon"}}
            on_click={handle_click}
            on_delete={handle_delete}
            delete_icon={html!{<DeleteIcon />}}
            variant={Variant::Outlined}
            />
        </DemoBox>

        <h2>{"Chip adornments"}</h2>
        <p>{"You can add ornaments to the beginning of the component."}</p>

        <h3>{"Avatar chip"}</h3>
        <DemoBox>
        <Chip avatar={html!{<Avatar>{"M"}</Avatar>}} label={html!{"Avatar"}} />
        <Chip
            avatar={html!{<Avatar alt="Natacha" src="/static/images/avatar/1.jpg" />}}
            label={html!{"Avatar"}}
            variant={Variant::Outlined}
            />
        </DemoBox>

        <h3>{"Icon chip"}</h3>
        <DemoBox>
        <Chip icon={html!{<FaceIcon />}} label={html!{"With Icon"}} />
        <Chip icon={html!{<FaceIcon />}}
                label={html!{"With Icon"}}
                variant={Variant::Outlined} />
        </DemoBox>

        <h2>{"Color chip"}</h2>
        <p>{"You can use the color prop to define a color from theme palette."}</p>
        <DemoBox>
        <Stack direction={Direction::Row} spacing={Spacing::Small}>
            <Chip label={html!{"primary"}} color={Color::Primary} />
            <Chip label={html!{"success"}} color={Color::Success} />
        </Stack>
        <Stack direction={Direction::Row} spacing={Spacing::Small}>
            <Chip label={html!{"primary"}}
                color={Color::Primary}
                variant={Variant::Outlined} />
            <Chip label={html!{"success"}}
                color={Color::Success}
                variant={Variant::Outlined} />
        </Stack>
        </DemoBox>

        <h2>{"Size chip"}</h2>
        <p>{"You can use the size prop to define a small Chip."}</p>
        <DemoBox>
        <Chip label={html!{"Small"}} size={Size::Small} />
        <Chip label={html!{"Small"}}
            size={Size::Small}
            variant={Variant::Outlined} />
        </DemoBox>

        </div>
    }
}
