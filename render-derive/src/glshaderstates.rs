use quote;
use syn;


#[derive(Debug)]
pub enum StateKey {
    Depth,
    WriteMask,
    Cull,
}

#[derive(Debug)]
pub struct State {
    pub key: StateKey,
    pub field_tokens: Option<quote::Tokens>,
    pub apply_tokens: quote::Tokens,
}

impl State {
    fn new_depth(value: &syn::LitStr) -> Option<State> {
        match value.value().to_string().as_ref() {
            "disable" => Some(State {
                key: StateKey::Depth,
                field_tokens: None,
                apply_tokens: quote_call_site! {context.ll.states.set_depth(_shine_render_core::DepthFunction::Disable);},
            }),
            "always" => Some(State {
                key: StateKey::Depth,
                field_tokens: None,
                apply_tokens: quote_call_site! {context.ll.states.set_depth(_shine_render_core::DepthFunction::Always);},
            }),
            "never" => Some(State {
                key: StateKey::Depth,
                field_tokens: None,
                apply_tokens: quote_call_site! {context.ll.states.set_depth(_shine_render_core::DepthFunction::Never);},
            }),
            "less" => Some(State {
                key: StateKey::Depth,
                field_tokens: None,
                apply_tokens: quote_call_site! {context.ll.states.set_depth(_shine_render_core::DepthFunction::Less);},
            }),
            "less_equal" => Some(State {
                key: StateKey::Depth,
                field_tokens: None,
                apply_tokens: quote_call_site! {context.ll.states.set_depth(_shine_render_core::DepthFunction::LessEqual);},
            }),
            "greater" => Some(State {
                key: StateKey::Depth,
                field_tokens: None,
                apply_tokens: quote_call_site! {context.ll.states.set_depth(_shine_render_core::DepthFunction::Greater);},
            }),
            "greater_equal" => Some(State {
                key: StateKey::Depth,
                field_tokens: None,
                apply_tokens: quote_call_site! {context.ll.states.set_depth(_shine_render_core::DepthFunction::GreaterEqual);},
            }),
            "equal" => Some(State {
                key: StateKey::Depth,
                field_tokens: None,
                apply_tokens: quote_call_site! {context.ll.states.set_depth(_shine_render_core::DepthFunction::Equal);},
            }),
            "?" => Some(State {
                key: StateKey::Depth,
                field_tokens: Some(quote_call_site! {depth: _shine_render_core::DepthFunction}),
                apply_tokens: quote_call_site! {context.ll.states.set_depth(self.depth);},
            }),
            _ => panic!("Unknown depth state: {:?}", value.value()),
        }
    }

    fn new_write_mask(value: &syn::LitStr) -> Option<State> {
        let value = value.value().to_string();
        if value == "?" {
            Some(State {
                key: StateKey::WriteMask,
                field_tokens: Some(quote_call_site! {write_mask: _shine_render_core::WriteMask}),
                apply_tokens: quote_call_site! {context.ll.states.set_write_mask(self.write_mask);},
            })
        } else {
            panic!("un-implemented write mask: {:?}", value);
        }
    }

    fn new_cull(value: &syn::LitStr) -> Option<State> {
        match value.value().to_string().as_ref() {
            "disable" => Some(State {
                key: StateKey::Cull,
                field_tokens: None,
                apply_tokens: quote_call_site! {context.ll.states.set_cull(_shine_render_core::CullFunction::Disable);},
            }),
            "clockwise" | "cw" => Some(State {
                key: StateKey::Cull,
                field_tokens: None,
                apply_tokens: quote_call_site! {context.ll.states.set_cull(_shine_render_core::CullFunction::Clockwise);},
            }),
            "counterclockwise" | "ccw" => Some(State {
                key: StateKey::Cull,
                field_tokens: None,
                apply_tokens: quote_call_site! {context.ll.states.set_cull(_shine_render_core::CullFunction::CounterClockwise);},
            }),
            "?" => Some(State {
                key: StateKey::Cull,
                field_tokens: Some(quote_call_site! {cull: _shine_render_core::CullFunction}),
                apply_tokens: quote_call_site! {context.ll.states.set_cull(self.cull);},
            }),
            _ => panic!("Unknown depth state: {:?}", value.value()),
        }
    }

    pub fn from_meta(meta: &syn::Meta) -> Option<State> {
        if let syn::Meta::NameValue(syn::MetaNameValue { ref ident, lit: syn::Lit::Str(ref value), .. }) = *meta {
            match ident.to_string().as_ref() {
                //"viewport" => Self::new_viewport(value),
                "depth" => Self::new_depth(value),
                "write_mask" => Self::new_write_mask(value),
                //"blend" => Self::new_blend(value),
                "cull" => Self::new_cull(value),
                _ => None,
            }
        } else {
            None
        }
    }
}