use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    let chars: Vec<char> = s.chars().collect();

    for (i, &ch) in chars.iter().enumerate() {
        if ch.is_uppercase() {
            let is_first = i == 0;
            let prev_is_lower = i > 0 && chars[i - 1].is_lowercase();
            let next_is_lower = i + 1 < chars.len() && chars[i + 1].is_lowercase();

            // Add underscore if:
            // 1. Previous char is lowercase (camelCase -> snake_case)
            // 2. This is uppercase, next is lowercase, and we're not first (handles acronyms like "FSM" -> "_fsm")
            if !is_first && (prev_is_lower || next_is_lower) {
                result.push('_');
            }

            result.push(ch.to_lowercase().next().unwrap());
        } else {
            result.push(ch);
        }
    }
    result
}

/// Derive macro for FSMState trait.
///
/// Generates variant-specific event types and implements FSMState trait.
/// Transition logic is provided via a separate trait implementation.
///
/// # Example
/// ```rust
/// use bevy::prelude::*;
/// use bevy_fsm::{FSMState, FSMTransition};
///
/// #[derive(Component, FSMState, Clone, Copy, Debug, PartialEq, Eq, Hash)]
/// enum LifeFSM {
///     Alive,
///     Dying,
///     Dead,
/// }
///
/// impl FSMTransition for LifeFSM {
///     fn can_transition(from: Self, to: Self) -> bool {
///         matches!((from, to),
///             (LifeFSM::Alive, LifeFSM::Dying) |
///             (LifeFSM::Dying, LifeFSM::Dead)) || from == to
///     }
/// }
/// ```
#[proc_macro_derive(FSMState)]
pub fn derive_fsm_state(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_name = &input.ident;

    // Extract variants from enum
    let variants = match &input.data {
        Data::Enum(data_enum) => &data_enum.variants,
        _ => panic!("FSMState can only be derived for enums"),
    };

    // Verify all variants are unit variants
    for variant in variants {
        if !matches!(variant.fields, Fields::Unit) {
            panic!("FSMState enum variants must be unit variants (no fields)");
        }
    }

    let variant_idents: Vec<_> = variants.iter().map(|v| &v.ident).collect();

    // Generate the module structure with variant event types
    // Convert EnumName to snake_case for module name
    let module_name_str = to_snake_case(&enum_name.to_string());
    let fsm_module_name = syn::Ident::new(&module_name_str, enum_name.span());

    // Generate all pairs of transition types
    let mut transition_triggers = Vec::new();
    for from_variant in &variant_idents {
        for to_variant in &variant_idents {
            transition_triggers.push(quote! {
                (#enum_name::#from_variant, #enum_name::#to_variant) => {
                    ec.trigger(bevy_fsm::Transition::<#fsm_module_name::#from_variant, #fsm_module_name::#to_variant> {
                        from: #fsm_module_name::#from_variant,
                        to: #fsm_module_name::#to_variant,
                    });
                }
            });
        }
    }

    let expanded = quote! {
        // Generate variant-specific event types directly in module
        pub mod #fsm_module_name {
            use bevy::prelude::Event;

            #(
                #[derive(Event, Clone, Copy, Debug)]
                pub struct #variant_idents;
            )*
        }

        // Implement FSMState trait
        impl bevy_fsm::FSMState for #enum_name {
            fn can_transition(from: Self, to: Self) -> bool {
                <Self as bevy_fsm::FSMTransition>::can_transition(from, to)
            }

            fn trigger_enter_variant(ec: &mut bevy::prelude::EntityCommands, state: Self) {
                match state {
                    #(
                        #enum_name::#variant_idents => {
                            ec.trigger(bevy_fsm::Enter::<#fsm_module_name::#variant_idents> {
                                state: #fsm_module_name::#variant_idents,
                            });
                        }
                    )*
                }
            }

            fn trigger_exit_variant(ec: &mut bevy::prelude::EntityCommands, state: Self) {
                match state {
                    #(
                        #enum_name::#variant_idents => {
                            ec.trigger(bevy_fsm::Exit::<#fsm_module_name::#variant_idents> {
                                state: #fsm_module_name::#variant_idents,
                            });
                        }
                    )*
                }
            }

            fn trigger_transition_variant(ec: &mut bevy::prelude::EntityCommands, from: Self, to: Self) {
                // Generate variant-specific transition events
                match (from, to) {
                    #(#transition_triggers)*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
