#[macro_use]
mod utils;
use proc_macro::TokenStream;
use proc_macro2;
#[allow(unused_imports)]
use proc_macro2::Span;
use quote::{format_ident, quote, quote_spanned};
use syn::{
    parse::{discouraged::Speculative, Parse, ParseStream, Result},
    parse_macro_input, parse_quote,
    punctuated::Punctuated,
    spanned::Spanned,
    Data, DeriveInput, Error, Field, Fields, GenericParam, Generics, Ident, Index, WhereClause,
};
#[allow(unused_imports)]
use utils::*;

type TokenStream2 = proc_macro2::TokenStream;

/// derive ToBytes
/// # usage
///
/// `#[to_bytes(asis tags)]`  
/// - `#[to_bytes]`  
///   - `bytes![fields.to_bytes() ...]`
///   - impl TransmuteBack
/// - `#[asis]`
///   -  like AsIs
///   - impl ReadBack
///   - impl ReadBackMut
/// - `#[asis copy]`
///   - like AsIs but no drops
///   - impl ReadBack
///   - impl ReadBackMut
///   - impl TransmuteBack
/// ## Example
/// - `#[to_bytes]`
/// - `#[to_bytes(asis)]`
/// - `#[to_bytes(asis copy)]`
#[proc_macro_attribute]
pub fn to_bytes(attr: TokenStream, item: TokenStream) -> TokenStream {
    let tbt = parse_macro_input!(attr as ToBytesType);
    let input = parse_macro_input!(item as DeriveInput);
    let input2 = input.clone();

    let name = input.ident;

    let generics = match tbt {
        ToBytesType::None => add_trait_bounds(input.generics),
        _ => input.generics,
    };
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let where_clause = match tbt {
        ToBytesType::AsIsCopy => where_clause
            .cloned()
            .map(|v| {
                let mut nv = v.clone();
                nv.predicates.push(parse_quote!(Self: Default + Copy));
                v
            })
            .or_else(|| {
                Some(WhereClause {
                    where_token: parse_quote!(where),
                    predicates: {
                        let mut p = Punctuated::new();
                        p.push(parse_quote!(Self: Default + Copy));
                        p
                    },
                })
            }),
        ToBytesType::AsIs => where_clause
            .cloned()
            .map(|v| {
                let mut nv = v.clone();
                nv.predicates.push(parse_quote!(Self: Default));
                v
            })
            .or_else(|| {
                Some(WhereClause {
                    where_token: parse_quote!(where),
                    predicates: {
                        let mut p = Punctuated::new();
                        p.push(parse_quote!(Self: Default));
                        p
                    },
                })
            }),
        _ => where_clause.cloned(),
    };

    let impls = match tbt {
        ToBytesType::None => {
            let check = check_fields(&input.data, &name);
            quote! {
                #check
            }
        }
        ToBytesType::AsIs => {
            quote! {
                unsafe {
                    let raw = ::to_bytes::ToRawBytes::to_raw_bytes(self);
                    ::to_bytes::Bytes::from_raw_with_drops(raw, Some(vec![(0, ::to_bytes::the_drop::<Self>)]))
                }
            }
        }
        ToBytesType::AsIsCopy => {
            quote! {
                unsafe {
                    let raw = ::to_bytes::ToRawBytes::to_raw_bytes(self);
                    ::to_bytes::Bytes::from_raw_bytes(raw)
                }
            }
        },
    };

    let backs = match tbt {
        ToBytesType::None => {
            let back = back_fields(&input.data, &name);
            quote! {
                impl #impl_generics ::to_bytes::TransmuteBack for #name #ty_generics #where_clause {
                    #[allow(unused_assignments)]
                    #[inline]
                    unsafe fn transmute_back(ptr: *const u8) -> Self {
                        #[allow(unused_mut)]
                        let mut offset = 0usize;
                        #back
                    }
                }
            }
        }
        ToBytesType::AsIs => {
            let mut generics = generics.clone();
            generics.params.push(GenericParam::Lifetime(parse_quote!('bytes)));
            let (impl_generics, ..) = generics.split_for_impl();
            quote! {
                impl #impl_generics ::to_bytes::ReadBack<'bytes> for #name #ty_generics #where_clause {
                    #[inline]
                    unsafe fn read_back(ptr: *const u8) -> &'bytes Self {
                        &*(ptr as *const Self)
                    }
                }
                impl #impl_generics ::to_bytes::ReadBackMut<'bytes> for #name #ty_generics #where_clause {
                    #[inline]
                    unsafe fn read_back_mut(ptr: *mut u8) -> &'bytes mut Self {
                        &mut *(ptr as *mut Self)
                    }
                }
            }
        }
        ToBytesType::AsIsCopy => {
            let mut generics = generics.clone();
            generics.params.push(GenericParam::Lifetime(parse_quote!('bytes)));
            let (impl_generics, ..) = generics.split_for_impl();
            quote! {
                impl #impl_generics ::to_bytes::ReadBack<'bytes> for #name #ty_generics #where_clause {
                    #[inline]
                    unsafe fn read_back(ptr: *const u8) -> &'bytes Self {
                        &*(ptr as *const Self)
                    }
                }
                impl #impl_generics ::to_bytes::ReadBackMut<'bytes> for #name #ty_generics #where_clause {
                    #[inline]
                    unsafe fn read_back_mut(ptr: *mut u8) -> &'bytes mut Self {
                        &mut *(ptr as *mut Self)
                    }
                }
                impl #impl_generics ::to_bytes::TransmuteBack for #name #ty_generics #where_clause {
                    #[inline]
                    unsafe fn transmute_back(ptr: *const u8) -> Self {
                        *(ptr as *const Self)
                    }
                }
            }
        },
    };

    let expanded = quote! {
        #input2
        impl #impl_generics ::to_bytes::ToBytes for #name #ty_generics #where_clause {
            #[inline]
            fn to_bytes(self) -> ::to_bytes::Bytes {
                #impls
            }
        }
        #backs
    };

    //return syn::Error::new(Span::call_site(), expanded.to_string()).to_compile_error().into();
    expanded.into()
}

enum ToBytesType {
    None,
    AsIs,
    AsIsCopy,
}
impl Parse for ToBytesType {
    fn parse(input: ParseStream) -> Result<Self> {
        if let Some(asis) = try_parser!(input; Ident) {
            if asis.to_string().to_lowercase() != "asis" {
                return Err(Error::new(asis.span(), format!("\"{}\" s not a legal tag. Can only be \"asis\" or \"asis copy\"", asis)));
            }
            if let Some(copy) = try_parser!(input; Ident) {
                if copy.to_string().to_lowercase() != "copy" {
                    return Err(Error::new(asis.span(), format!("\"{} {}\" s not a legal tag. Can only be \"asis\" or \"asis copy\"", asis, copy)));
                }
                Ok(Self::AsIsCopy)
            } else {
                Ok(Self::AsIs)
            }
        } else {
            Ok(Self::None)
        }
    }
}

fn add_trait_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(to_bytes::ToBytes));
        }
    }
    generics
}

fn check_fields(data: &Data, name: &Ident) -> TokenStream2 {
    match data {
        Data::Struct(data) => match data.fields {
            Fields::Named(ref fields) => {
                let recurse = fields.named.iter().map(|f| {
                    let name = &f.ident;
                    quote_spanned! {f.span() => self.#name }
                });
                quote! {
                    ::to_bytes::bytes![#(#recurse)*]
                }
            }
            Fields::Unnamed(ref fields) => {
                let recurse = fields.unnamed.iter().enumerate().map(|(i, f)| {
                    let index = Index::from(i);
                    quote_spanned! {f.span() => self.#index }
                });
                quote! {
                    ::to_bytes::bytes![#(#recurse)*]
                }
            }
            Fields::Unit => quote!(),
        },
        Data::Enum(data) => {
            let mut matches = vec![];
            for (i, variant) in data.variants.iter().enumerate() {
                let i = i as u8;
                let subtype = &variant.ident;
                let subtype = quote!(#name::#subtype);
                match variant.fields {
                    Fields::Unnamed(ref fields) => {
                        let len = fields.unnamed.len();
                        let vars = numbered_vars(len, "");
                        let matc = quote! {
                            #subtype(#(#vars),*) => {
                                ::to_bytes::bytes![#i #(#vars)*]
                            }
                        };
                        matches.push(matc);
                    }
                    Fields::Named(ref fields) => {
                        let fies = fields.named.iter().collect::<Vec<_>>();
                        let names = field_idents(&fies);
                        let matc = quote! {
                            #subtype { #(#names),* } => {
                                ::to_bytes::bytes![#i #(#names)*]
                            }
                        };
                        matches.push(matc);
                    }
                    Fields::Unit => {}
                };
            }
            quote! {
                match self {
                    #(#matches),*
                }
            }
        }
        Data::Union(_) => Error::new(name.span(), "ToBytes does not support union").to_compile_error(),
    }
}

fn back_fields(data: &Data, name: &Ident) -> TokenStream2 {
    match data {
        Data::Struct(data) => match data.fields {
            Fields::Named(ref fields) => {
                let recurse = fields.named.iter().map(|f| {
                    let name = &f.ident;
                    quote_spanned! {f.span() =>
                        #name : {
                            let v = ::to_bytes::TransmuteBack::transmute_back(ptr.add(offset));
                            offset += ::core::mem::size_of_val(&v);
                            v
                        },
                    }
                });
                quote! {
                    Self { #(#recurse)* }
                }
            }
            Fields::Unnamed(ref fields) => {
                let recurse = fields.unnamed.iter().enumerate().map(|(_, f)| {
                    //let index = Index::from(i);
                    quote_spanned! {f.span() =>
                        {
                            let v = ::to_bytes::TransmuteBack::transmute_back(ptr.add(offset));
                            offset += ::core::mem::size_of_val(&v);
                            v
                        },
                    }
                });
                quote! {
                    Self ( #(#recurse)* )
                }
            }
            Fields::Unit => quote!(),
        },
        Data::Enum(data) => {
            let mut matches = vec![];
            for (i, variant) in data.variants.iter().enumerate() {
                let i = i as u8;
                let subtype = &variant.ident;
                let subtype = quote!(#name::#subtype);
                match variant.fields {
                    Fields::Unnamed(ref fields) => {
                        let len = fields.unnamed.len();
                        let back = (0..len).map(|_| {
                            quote! {
                                {
                                    let v = ::to_bytes::TransmuteBack::transmute_back(ptr.add(offset));
                                    offset += ::core::mem::size_of_val(&v);
                                    v
                                },
                            }
                        });
                        let matc = quote! {
                            #i => {
                                #subtype (#(#back)*)
                            }
                        };
                        matches.push(matc);
                    }
                    Fields::Named(ref fields) => {
                        let back = fields.named.iter().map(|f| {
                            let id = f.ident.as_ref().expect("Tried to get field names of a tuple struct");
                            quote! {
                                #id: {
                                    let v = ::to_bytes::TransmuteBack::transmute_back(ptr.add(offset));
                                    offset += ::core::mem::size_of_val(&v);
                                    v
                                },
                            }
                        });
                        let matc = quote! {
                            #i => {
                                #subtype { #(#back)* }
                            }
                        };
                        matches.push(matc);
                    }
                    Fields::Unit => {}
                };
            }
            quote! {
                let tag: u8 = ::to_bytes::TransmuteBack::transmute_back(ptr);
                offset += ::core::mem::size_of::<u8>();
                match tag {
                    #(#matches),*
                    _ => panic!("never")
                }
            }
        }
        Data::Union(_) => Error::new(name.span(), "ToBytes does not support union").to_compile_error(),
    }
}

fn numbered_vars(count: usize, prefix: &str) -> Vec<Ident> {
    (0..count).map(|i| format_ident!("__{}{}", prefix, i)).collect()
}

fn field_idents<'a>(fields: &'a [&'a Field]) -> Vec<&'a Ident> {
    fields.iter().map(|f| f.ident.as_ref().expect("Tried to get field names of a tuple struct")).collect()
}
