mod utils;
use utils::{get_property_field_name, implement, is_named_struct, is_struct};

use proc_macro::TokenStream;
use syn::{DeriveInput, Type, parse_macro_input, parse_quote};

#[proc_macro_derive(Exception)]
pub fn make_exception(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident.clone();
    let target_type: Type = parse_quote! {
        Box<Property<#struct_name>>
    };

    let data_struct = is_struct(&input).unwrap();

    let fields = is_named_struct(&data_struct).unwrap();

    let matching_fields: Vec<_> = fields.iter().filter(|f| f.ty == target_type).collect();

    let property_name = get_property_field_name(matching_fields, &struct_name).unwrap();

    implement(&struct_name, &property_name)
}
