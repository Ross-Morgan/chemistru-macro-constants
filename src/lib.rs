extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

use chemistru_elements::raw::RawElement;

static DATA: &str = include_str!("../periodic-table-data/periodic-table.json");

#[proc_macro]
pub fn elements_consts(_: TokenStream) -> TokenStream {
    let elements: Vec<RawElement> = serde_json::from_str(DATA).expect("Failed to load elements");

    let elements_init = elements.into_iter().map(generate_const_init);

    let tokens = quote! {
        #( #elements_init )*
    };

    TokenStream::from(tokens)
}

fn generate_const_init(element: RawElement) -> proc_macro2::TokenStream {
    let assignment_name = proc_macro2::Ident::new(&element.name.to_uppercase().replace(' ', "_"), proc_macro2::Span::call_site());
    let name = element.name;
    let symbol = element.symbol;
    let proton_number = element.number;
    let mass_number = element.atomic_mass;

    let inner = element.into_inner();

    quote! {
        pub const #assignment_name: chemistru_elements::element::Element = chemistru_elements::element::Element::new(
            #name,
            #symbol,
            #mass_number,
            #proton_number,
            #inner
        );
    }
}
