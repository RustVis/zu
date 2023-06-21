// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use yew::prelude::*;
use zu::typography::{Typography, Variant};

use crate::components::demo_box::DemoBox;

#[function_component(TypographyPage)]
pub fn typography_page() -> Html {
    html! {
         <div class="container">
         <h1>{"Typography"}</h1>

         <h2>{"Component"}</h2>
         <p>{"The Typography component makes it easy to apply a default set of font weights and sizes in your application."}</p>
         <DemoBox style="width: 80%;">
             <Typography variant={Variant::H1} gutter_bottom={true}>{"h1. Heading"}</Typography>
             <Typography variant={Variant::H2} gutter_bottom={true}>{"h2. Heading"}</Typography>
             <Typography variant={Variant::H3} gutter_bottom={true}>{"h3. Heading"}</Typography>
             <Typography variant={Variant::H4} gutter_bottom={true}>{"h4. Heading"}</Typography>
             <Typography variant={Variant::H5} gutter_bottom={true}>{"h5. Heading"}</Typography>
             <Typography variant={Variant::H6} gutter_bottom={true}>{"h6. Heading"}</Typography>
             <Typography variant={Variant::Subtitle1} gutter_bottom={true}>
               {"subtitle1. Lorem ipsum dolor sit amet, consectetur adipisicing elit. Quos blanditiis tenetur"}
             </Typography>
             <Typography variant={Variant::Subtitle2} gutter_bottom={true}>
               {"subtitle2. Lorem ipsum dolor sit amet, consectetur adipisicing elit. Quos blanditiis tenetur"}
             </Typography>
             <Typography variant={Variant::Body1} gutter_bottom={true}>
               {"body1. Lorem ipsum dolor sit amet, consectetur adipisicing elit. Quos
                blanditiis tenetur unde suscipit, quam beatae rerum inventore consectetur,
                neque doloribus, cupiditate numquam dignissimos laborum fugiat deleniti? Eum
                quasi quidem quibusdam."}
             </Typography>
             <Typography variant={Variant::Body2} gutter_bottom={true}>
               {"body2. Lorem ipsum dolor sit amet, consectetur adipisicing elit. Quos
                blanditiis tenetur unde suscipit, quam beatae rerum inventore consectetur,
                neque doloribus, cupiditate numquam dignissimos laborum fugiat deleniti? Eum
                quasi quidem quibusdam."}
             </Typography>
             <Typography variant={Variant::Button} style={"display: 'block'"} gutter_bottom={true}>
               {"button text"}
             </Typography>
             <Typography variant={Variant::Caption} style={"display: 'block'"} gutter_bottom={true}>
               {"caption text"}
             </Typography>
             <Typography variant={Variant::Overline} style={"display: 'block'"} gutter_bottom={true}>
               {"overline text"}
             </Typography>
         </DemoBox>

         </div>
    }
}
