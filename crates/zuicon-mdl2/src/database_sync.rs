// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct DatabaseSync {}

#[derive(Properties, Debug, Clone, PartialEq, Eq)]
pub struct Props {
    #[prop_or_default]
    pub class: Option<&'static str>,

    #[prop_or_default]
    pub width: Option<&'static str>,

    #[prop_or_default]
    pub height: Option<&'static str>,

    #[prop_or_default]
    pub color: Option<&'static str>,

    #[prop_or_default]
    pub fill: Option<&'static str>,

    #[prop_or_default]
    pub spin: bool,

    #[prop_or_default]
    pub rotate: i16,
}

impl Component for DatabaseSync {
    type Properties = Props;
    type Message = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        // TODO(Shaohua): Do not generate style attribute if it is empty.
        let mut style = String::new();
        if props.rotate != 0 {
            style += &format!("transform: rotate({}deg);", props.rotate);
        }
        html! {
            <svg
                xmlns={ "http://www.w3.org/2000/svg" }
                class={ props.class.unwrap_or("") }
                width={ props.width.unwrap_or("16") }
                height={ props.height.unwrap_or("16") }
                focusable={ "false" }
                data-icon={ "DatabaseSync" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M252 569q-32-12-63-26t-61-33v706q0 19 14 37t35 33 43 26 36 18v136q-41-15-86-37t-83-52-62-71-25-90V320q0-48 22-86t60-70 84-54 96-41 96-29 84-18Q570 0 704 0q47 0 110 4t133 16 140 29 131 47 107 66 69 90q-50-9-99-15t-100-10q-51-28-112-47t-127-30-130-17-122-5q-58 0-129 5t-143 20-138 37-115 59q-17 12-34 30t-17 41q0 26 24 48t56 41 66 32 53 19q-23 24-42 51t-33 58zm836 455q-68 0-144-6t-153-22-149-41-130-61v706q0 23 18 44t48 38 66 32 72 26 64 18 46 11q49 9 98 14t100 8v128q-44-2-108-9t-136-22-142-39-127-59-92-82-35-108V704q0-47 22-86t58-69 83-54 95-41 96-29 83-18q66-12 133-17t134-6q67 0 134 5t133 18q36 7 83 18t95 28 95 41 83 54 59 70 22 86v320h-128V894q-59 36-130 61t-148 40-153 22-145 7zm0-512q-57 0-130 6t-148 20-143 40-115 63q-14 11-27 27t-13 36q0 19 13 35t27 28q46 38 114 63t143 39 148 21 131 6q57 0 130-6t148-20 143-40 114-63q14-11 27-27t14-36q0-19-13-35t-28-28q-46-38-114-63t-142-39-148-21-131-6zm832 640h128v384h-384v-128h190q-45-60-112-94t-142-34q-59 0-111 20t-95 55-70 85-38 107l-127-22q14-81 54-149t98-118 133-78 156-28q91 0 174 35t146 102v-137zm-320 768q59 0 111-20t95-55 70-85 39-107l126 22q-14 81-54 149t-98 118-133 78-156 28q-91 0-174-35t-146-102v137h-128v-384h384v128h-190q45 60 112 94t142 34z" />
            </svg>
        }
    }
}
