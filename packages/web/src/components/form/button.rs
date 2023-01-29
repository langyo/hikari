use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(Properties, Debug, PartialEq)]
pub struct ButtonProps {
    #[prop_or_default]
    pub children: Children,
}

#[styled_component]
pub fn Button(props: &ButtonProps) -> Html {
    html! {
        <button
            class={css!(r#"
                display: flex;
                align-items: center;
                justify-content: center;
            "#)}
        >
            {props.children.clone()}
        </button>
    }
}
