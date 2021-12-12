use proc_macro2::{Ident, Span};
use syn::{
    parse::Parse, parse_macro_input, punctuated::Punctuated, token::Paren, AttributeArgs, Expr,
    ExprAssign, ExprPath, ItemFn, ItemType, Lit, Meta, MetaNameValue, Path, Token, Type, TypeTuple,
};

use quote::quote;

fn parse_expr_num(expr: &Expr, name: &str) -> u32 {
    match expr {
        Expr::Lit(l) => parse_lit_num(&l.lit, name),
        _ => panic!("Expected {} to be a literal", name),
    }
}

fn parse_lit_num(l: &Lit, name: &str) -> u32 {
    match &l {
        syn::Lit::Str(s) => s
            .value()
            .parse::<u32>()
            .expect(&format!("Expected {} to be a number", name)),
        syn::Lit::Int(i) => i.base10_parse().expect(&format!("Cannot parse {}", name)),
        _ => panic!("Expected {} to be a number", name),
    }
}

#[proc_macro]
pub fn xaoc(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    struct XaocInput {
        params: Vec<ExprAssign>,
    }
    impl Parse for XaocInput {
        fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
            Punctuated::<ExprAssign, Token![;]>::parse_terminated(input).map(|it| Self {
                params: it.into_iter().collect(),
            })
        }
    }

    impl XaocInput {
        fn lookup_param(&self, name: &str) -> Option<&Expr> {
            self.params
                .iter()
                .find(|it| match &*it.left {
                    Expr::Path(ExprPath {
                        path: Path { segments: s, .. },
                        ..
                    }) => s.len() == 1 && s.first().unwrap().ident == name,
                    _ => false,
                })
                .map(|it| &*it.right)
        }
    }

    let input = parse_macro_input!(input as XaocInput);

    let year = input.lookup_param("year").expect("Expected year argument");
    let day = input.lookup_param("day").expect("Expected day argument");

    let year = parse_expr_num(year, "year");
    let day = parse_expr_num(day, "day");

    quote!{
        use ::xaoc::linkme;

        pub(crate) const __XAOC_SOLUTION_DAY_DATA: ::xaoc::DayData = ::xaoc::DayData::new(#year, #day);

        #[::xaoc::ds]
        pub(crate) static __XAOC_EXAMPLES: [::xaoc::ExampleData<__XAOC_OUTPUT_TYPE1, __XAOC_OUTPUT_TYPE2>] = [..];
    }.into()
}

#[proc_macro]
pub fn xaoc_types(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    struct XaocTypesInput {
        types: Vec<ItemType>,
    }
    impl Parse for XaocTypesInput {
        fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
            let mut types = vec![];
            while !input.is_empty() {
                types.push(input.parse()?);
            }
            Ok(Self { types })
        }
    }

    impl XaocTypesInput {
        fn lookup_type(&self, name: &str) -> Option<&Type> {
            self.types
                .iter()
                .find(|it| it.ident == name)
                .map(|it| &*it.ty)
        }
    }

    let input = parse_macro_input!(input as XaocTypesInput);
    let input_ty = input.lookup_type("Input").expect("Expected input");
    let output_ty = input.lookup_type("Output");

    fn tyunit() -> Type {
        Type::Tuple(TypeTuple {
            paren_token: Paren {
                span: Span::call_site(),
            },
            elems: Punctuated::new(),
        })
    }

    let unit = tyunit();

    let output_ty = output_ty.unwrap_or(&unit);
    let output_ty1 = input.lookup_type("Output1").unwrap_or(output_ty);
    let output_ty2 = input.lookup_type("Output2").unwrap_or(output_ty);

    quote! {
        type Input = #input_ty;
        type Output = #output_ty;
        type Output1 = #output_ty1;
        type Output2 = #output_ty2;

        type __XAOC_INPUT_TYPE = #input_ty;
        type __XAOC_OUTPUT_TYPE1 = #output_ty1;
        type __XAOC_OUTPUT_TYPE2 = #output_ty2;
    }
    .into()
}

#[proc_macro]
pub fn xaoc_main(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    struct XaocMainInput {
        params: Vec<Ident>,
    }
    impl Parse for XaocMainInput {
        fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
            Punctuated::<Ident, Token![;]>::parse_terminated(input).map(|it| Self {
                params: it.into_iter().collect(),
            })
        }
    }

    let input = parse_macro_input!(input as XaocMainInput);

    let params = &input.params;

    quote! {
        #(mod #params;)*

        fn main() {
            "--year" / "-y" -> default current year;
            "--day" / "-d" -> default current day;
            "--part" / "-p" -> default last part there is;

            let y = 0;
            let d = 0;

            [#(& #params .__XAOC_SOLUTION_DAY_DATA,)*].find(|it| it.is(y, dd))
        }
    }
    .into()
}

#[proc_macro_attribute]
pub fn xaoc_input(
    _attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let fncode = parse_macro_input!(input as ItemFn);
    let fn_name = fncode.sig.ident.clone();

    quote! {
        #fncode
    
        use #fn_name as __XAOC_INPUT;
    }.into()
}

#[proc_macro_attribute]
pub fn xaoc_solver(
    attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let attr = parse_macro_input!(attr as AttributeArgs);
    let fncode = parse_macro_input!(input as ItemFn);
    let part = attr.iter().find_map(|it| match it {
        syn::NestedMeta::Meta(Meta::NameValue(MetaNameValue { path, lit, .. }))
            if path.is_ident("part") =>
        {
            Some(lit)
        }
        _ => None,
    });

    let part = parse_lit_num(
        part.expect("Missing path=1 or part=2 on xaoc_solver"),
        "part",
    );

    if part != 1 && part != 2 {
        panic!("Invalid part number: {}", part);
    }

    let name = quote::format_ident!("__XAOC_SOLVER{}", part);

    let fn_name = fncode.sig.ident.clone();

    quote! {
        #fncode

        use #fn_name as #name;
    }
    .into()
}
