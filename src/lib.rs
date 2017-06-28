extern crate proc_macro;
extern crate subtle;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(CTEq)]
pub fn derive_cteq(input: TokenStream) -> TokenStream {
    let s = input.to_string();

    let ast = syn::parse_derive_input(&s).unwrap();
    let typename = ast.ident;
    let generics = ast.generics;

    if let syn::Body::Struct(vdata) = ast.body {
        let gen = match vdata {
            syn::VariantData::Unit => impl_cteq_unit(typename, generics),
            syn::VariantData::Tuple(fields) => impl_cteq_fields(typename, fields, generics),
            syn::VariantData::Struct(fields) => impl_cteq_fields(typename, fields, generics),
        };
        gen.parse().unwrap()
    }
    else {
        // Enums can't be compared in constant time. Consider
        //
        // enum Foo {
        //     A,
        //     B(usize, usize, usize),
        // }
        //
        // If a = A, and b = B(0,0,0). Then comparing a to b is immediately false, since a and b
        // are different variants. Then consider a = B(0,0,1) and b = B(0,0,0). Now a and b are
        // compared field-by-field, which breaks the constant-time guarantee. A workaround might be
        // to force every variant to take as long as the longest-running variant, but that's
        // pretty unlikely.
        panic!("Cannot derive CTEq on anything but structs");
    }
}

fn impl_cteq_unit(typename: syn::Ident, generics: syn::Generics) -> quote::Tokens {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    quote! {
        // Unit structs are always equal
        impl #impl_generics CTEq for #typename #ty_generics #where_clause {
            fn ct_eq(&self, other: &#typename #ty_generics) -> subtle::Mask {
                1u8
            }
        }
    }
}

fn impl_cteq_fields(typename: syn::Ident, fields: Vec<syn::Field>, generics: syn::Generics)
        -> quote::Tokens {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    // Member names are either the field names (when we're in a struct) or numbers (when we're in a
    // tuple struct)
    let membernames1 = fields.into_iter()
                             .enumerate()
                             .map(|(i, f)| f.ident.unwrap_or(syn::Ident::from(&*i.to_string())))
                             .collect::<Vec<syn::Ident>>();
    let membernames2 = membernames1.clone();

    quote! {
        impl #impl_generics CTEq for #typename #ty_generics #where_clause {
            #[inline(always)]
            fn ct_eq(&self, other: &#typename #ty_generics) -> subtle::Mask {
                // Go through each member, ANDing together their equality results
                let mut x = 1u8;
                #(x &= self.#membernames1.ct_eq(&other.#membernames2);)*
                x
            }
        }
    }
}
