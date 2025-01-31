// comp: mapping for_if_clause+
// mapping: expression
// for_if_clause:
//    | 'for' pattern 'in' expression ('if' expression)*
// pattern: name (, name)*

// #[derive(Debug)] for debug extra-trait but off when actually using after building proc macro

use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Expr, Pat, Token,
};

struct Comprehensions {
    mapping: Mapping,
    for_if_clauses: ForIfClause,
    additional_for_ifs: Vec<ForIfClause>,
}

impl Parse for Comprehensions {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            mapping: input.parse()?,
            for_if_clauses: input.parse()?,
            additional_for_ifs: parse_zero_or_more(input),
        })
    }
}

impl ToTokens for Comprehensions {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let mut inv_for_ifs = std::iter::once(&self.for_if_clauses)
            .chain(&self.additional_for_ifs)
            .rev();

        let init_output = {
            let Mapping(mapping) = &self.mapping;
            let inner_clause = inv_for_ifs
                .next()
                .expect("At lease one ForIfClause should be present");

            let ForIfClause {
                pattern,
                sequence,
                conditions,
            } = inner_clause;

            quote! {
                core::iter::IntoIterator::into_iter(#sequence).flat_map(|#pattern|{
                    (true #(&& (#conditions))*).then(|| #mapping)
                })
            }
        };

        tokens.extend(inv_for_ifs.fold(init_output, |curr_output, next_fic| {
            let ForIfClause {
                pattern,
                sequence,
                conditions,
            } = next_fic;

            quote! {
                core::iter::IntoIterator::into_iter(#sequence).filter_map(|#pattern|{
                    (true #(&& (#conditions))*).then(|| #curr_output)
                }).flatten()
            }
        }))
    }
}

struct Comprehension {
    mapping: Mapping,
    for_if_clause: ForIfClause,
}

impl Parse for Comprehension {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            mapping: input.parse()?,
            for_if_clause: input.parse()?,
        })
    }
}

impl ToTokens for Comprehension {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        // core::iter::IntoIterator::into_iter(sequence|in expression|).flat_map(move|pattern|{
        //  (true && ...).then(|| mapping)
        //})
        let Mapping(mapping) = &self.mapping;
        let ForIfClause {
            pattern,
            sequence,
            conditions,
        } = &self.for_if_clause;

        // let conditions = conditions.iter().map(|c| c.0.to_token_stream());

        tokens.extend(quote! {
            core::iter::IntoIterator::into_iter(#sequence).flat_map(|#pattern|{
                (true #(&& (#conditions))*).then(|| #mapping)
            })
        })
    }
}

struct Mapping(syn::Expr);

impl Parse for Mapping {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse().map(Self)
    }
}

impl ToTokens for Mapping {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.0.to_tokens(tokens)
    }
}

struct ForIfClause {
    pattern: Pat,
    sequence: syn::Expr,
    conditions: Vec<Condition>,
}

impl Parse for ForIfClause {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        _ = input.parse::<Token![for]>()?;
        let pattern = Pat::parse_single(input)?;
        _ = input.parse::<Token![in]>()?;
        let sequence = input.parse()?;
        let conditions = parse_zero_or_more(input);
        Ok(Self {
            pattern,
            sequence,
            conditions,
        })
    }
}

fn parse_zero_or_more<T: Parse>(input: ParseStream) -> Vec<T> {
    let mut result = Vec::new();
    while let Ok(item) = input.parse() {
        result.push(item);
    }
    result
}

struct Condition(Expr);

impl Parse for Condition {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // consume 'if'
        _ = input.parse::<Token![if]>()?;
        input.parse().map(Self)
    }
}

impl ToTokens for Condition {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.0.to_tokens(tokens);
    }
}

#[proc_macro]
pub fn comp(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // parse input
    let c = parse_macro_input!(input as Comprehension);
    // render output
    quote! { #c }.into()
}

#[proc_macro]
pub fn comps(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // parse input
    let c = parse_macro_input!(input as Comprehensions);
    // render output
    quote! { #c }.into()
}
