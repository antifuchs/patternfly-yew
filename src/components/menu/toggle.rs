use crate::prelude::*;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct MenuToggleProperties {
    #[prop_or_default]
    pub r#ref: NodeRef,

    #[prop_or_default]
    pub text: Option<String>,

    #[prop_or_default]
    pub icon: Option<Html>,

    #[prop_or_default]
    pub aria_label: AttrValue,

    #[prop_or_default]
    pub expanded: bool,

    #[prop_or_default]
    pub disabled: bool,

    #[prop_or_default]
    pub full_height: bool,

    #[prop_or_default]
    pub variant: MenuToggleVariant,

    #[prop_or_default]
    pub ontoggle: Callback<()>,

    #[prop_or_default]
    pub children: Children,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub enum MenuToggleVariant {
    #[default]
    Default,
    Plain,
    Primary,
    Secondary,
}

impl AsClasses for MenuToggleVariant {
    fn extend_classes(&self, classes: &mut Classes) {
        match self {
            Self::Default => {}
            Self::Primary => classes.push(classes!("pf-m-primary")),
            Self::Secondary => classes.push(classes!("pf-m-secondary")),
            Self::Plain => classes.push(classes!("pf-m-plain")),
        }
    }
}

/// Menu toggle component
///
/// See: <https://pf5.patternfly.org/components/menus/menu-toggle>
///
/// ## Properties
///
/// Defined by [`MenuToggleProperties`].
#[function_component(MenuToggle)]
pub fn menu_toggle(props: &MenuToggleProperties) -> Html {
    let mut class = classes!("pf-v5-c-menu-toggle");

    if props.expanded {
        class.push(classes!("pf-m-expanded"));
    }

    if props.full_height {
        class.push(classes!("pf-m-full-height"));
    }

    if matches!(props.variant, MenuToggleVariant::Plain) {
        if props.text.is_some() {
            class.push(classes!("pf-m-text"));
        }
    }

    class.extend_from(&props.variant);

    let plain = matches!(props.variant, MenuToggleVariant::Plain);

    html!(
        <button
            ref={props.r#ref.clone()}
            {class}
            type="button"
            disabled={ props.disabled }
            aria-expanded={ props.expanded.to_string() }
            aria-label={ &props.aria_label }
            onclick={ props.ontoggle.reform(|_|()) }
        >
            if let Some(icon) = &props.icon {
                if plain {
                    { icon.clone() }
                } else {
                    <span class="pf-v5-c-menu-toggle__icon">{ icon.clone() }</span>
                }
            }
            if let Some(text) = &props.text {
                <span class="pf-v5-c-menu-toggle__text">{ text }</span>
            }

            if !plain || props.text.is_some() {
                // if we have more than just a plain icon, add the toggle control
                <span class="pf-v5-c-menu-toggle__controls">
                    <span class="pf-v5-c-menu-toggle__toggle-icon">
                        <i class="fas fa-caret-down" aria-hidden="true"></i>
                    </span>
                </span>
            }

            { for props.children.iter() }

        </button>
    )
}
