extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
#[macro_use] extern crate quote;

extern crate subtle;

#[proc_macro_derive(ConstantTimeEq)]
pub fn derive_cteq(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let typename = ast.ident;
    let generics = ast.generics;

    match ast.data {
        syn::Data::Struct(data_struct) => {
            let gen = match data_struct.fields {
                syn::Fields::Unit              => impl_cteq_unit(typename, generics),

                fields@syn::Fields::Named(_) |
                fields@syn::Fields::Unnamed(_) => impl_cteq_fields(typename, fields, generics),
            };
            gen.into()
        }

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
        // to force every variant to take as long as the longest-running variant, but that's pretty
        // unlikely.
        syn::Data::Enum(_) | syn::Data::Union(_) =>
            panic!("ConstantTimeEq can only be derived on struct, but {} is an enum", typename),
    }
}

fn impl_cteq_unit(typename: syn::Ident, generics: syn::Generics) -> proc_macro2::TokenStream {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    quote! {
        // Unit structs are always equal
        impl #impl_generics ConstantTimeEq for #typename #ty_generics #where_clause {
            #[inline(always)]
            fn ct_eq(&self, other: &#typename #ty_generics) -> subtle::Choice {
                subtle::Choice::from(1u8)
            }
        }
    }
}

fn impl_cteq_fields(
    typename: syn::Ident,
    fields: syn::Fields,
    generics: syn::Generics
) -> proc_macro2::TokenStream
{
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // Member names are either the field names (when we're in a normal struct) or numbers (when
    // we're in a tuple struct). Since the former are Idents and the latter are Literals, we branch
    // on that here and make a code fragment that does the right thing when accessing the struct
    // members. The code fragment does the obvious thing and ANDs all the equality values together.
    let and_fragment = match fields {
        syn::Fields::Named(named_fields) => {
            let idents = named_fields.named
                                     .into_iter()
                                     .map(|f| f.ident.unwrap())
                                     .collect::<Vec<syn::Ident>>();
            // We make a clone because the member names are mentioned twice in the repetition
            // clause in the impl. quote! doesn't like using the same identifier twice, but it will
            // happily zip two lists of the same length. So we make those two lists identical and
            // use that.
            let idents_copy = idents.clone();

            quote! {
                #(x = x & self.#idents.ct_eq(&other.#idents_copy);)*
            }
        }

        syn::Fields::Unnamed(unnamed_fields) => {
            let num_lits = (0..unnamed_fields.unnamed.len())
                               .map(|i| proc_macro2::Literal::usize_unsuffixed(i))
                               .collect::<Vec<proc_macro2::Literal>>();
            let num_lits_copy = num_lits.clone();

            quote! {
                #(x = x & self.#num_lits.ct_eq(&other.#num_lits_copy);)*
            }
        }

        syn::Fields::Unit => panic!("Unexpected error: impl_cteq_fields got a syn::Fields::Unit")
    };

    quote! {
        impl #impl_generics ConstantTimeEq for #typename #ty_generics #where_clause {
            #[inline(always)]
            fn ct_eq(&self, other: &#typename #ty_generics) -> subtle::Choice {
                let mut x = subtle::Choice::from(1u8);
                #and_fragment; // Insert the fragment from above here.
                x
            }
        }
    }
}
