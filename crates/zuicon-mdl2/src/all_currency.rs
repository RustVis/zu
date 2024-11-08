// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct AllCurrency {}

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

impl Component for AllCurrency {
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
                data-icon={ "AllCurrency" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M959 1026q-48-2-99-12t-94-31q-17-8-27-23t-10-35q0-26 19-45t45-19q13 0 27 6 66 29 138 30V678q-37-13-71-32t-61-45-43-61-16-77q0-41 16-74t43-57 61-41 71-25q0-29 16-50t48-22q27 0 45 18t19 46q35 2 66 9t65 17l-44 120q-21-8-43-12t-44-6v200q34 12 68 32t62 47 45 61 17 74q0 44-15 80t-40 63-61 45-76 28q0 28-17 48t-47 20q-25 0-43-17t-20-41zm127-143q28-11 46-30t18-52q0-12-6-23t-16-20-21-17-21-14v156zM895 463q0 28 20 46t43 30V400q-3 1-13 5t-12 7q-14 5-26 20t-12 31zm745 351q92 36 167 96t128 137 83 169 30 191q0 88-23 170t-64 153-100 129-130 101-153 65-170 23q-104 0-202-33t-181-96q-84 63-182 96t-203 33q-89 0-170-23t-153-64-129-100-100-130-65-153-23-171q0-99 29-190t83-169 129-138 167-96q-24-84-24-173 0-88 23-170t64-153 100-129T701 88t153-65 170-23q88 0 170 23t153 65 129 100 100 130 65 153 23 170q0 89-24 173zm-616-686q-106 0-199 40T662 279 552 442t-40 199q0 106 40 199t109 163 163 110 200 41q106 0 199-40t163-110 110-163 40-200q0-106-40-199t-109-163-163-110-200-41zM640 1920q106 0 199-40t163-110 110-163 40-200q0-68-20-134-54 9-108 9-105 0-203-33t-183-97q-7 4-13 5-88 26-133 91t-46 157v2h126q26 0 45 19t19 45q0 31-17 44t-41 18-51 4-46-2q35 59 93 92t128 34q29 0 54-6t52-17q13-5 26-5 26 0 45 18t19 45q0 29-22 47t-54 28-65 14-55 4q-61 0-117-18t-104-52-84-80-57-104q-16 0-31-3t-26-10-19-19-7-32q0-26 17-44t44-20q0-56 14-109t41-100 67-84 94-60q-23-29-43-60t-38-64q-74 29-133 77t-103 111-66 136-23 153q0 106 40 199t109 163 163 110 200 41zm768 0q106 0 199-40t163-110 110-163 40-200q0-79-23-152t-66-136-103-110-133-78q-38 74-93 136t-124 108l93 126 140-187q19-26 51-26 27 0 46 19t19 46q0 20-13 38l-180 239v105h41q22 0 42 4t33 18 14 42q0 28-13 41t-33 19-42 5-42-1v41q0 22-4 42t-18 33-42 14q-28 0-41-13t-19-33-5-42 1-42h-41q-22 0-41-4t-33-18-13-42q0-28 13-41t32-19 42-5 41 1v-105l-142-194-6 2q12 41 17 84t5 85q0 118-40 226t-119 197q64 44 136 67t151 23z" />
            </svg>
        }
    }
}
