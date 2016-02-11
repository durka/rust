// Copyright 2012-2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use deriving::generic::*;
use deriving::generic::ty::*;

use syntax::ast::{Expr, ItemKind, Generics, MetaItem, VariantData};
use syntax::attr::{self, AttrMetaMethods};
use syntax::codemap::Span;
use syntax::ext::base::{ExtCtxt, Annotatable};
use syntax::ext::build::AstBuilder;
use syntax::parse::token::InternedString;
use syntax::ptr::P;

pub fn expand_deriving_clone(cx: &mut ExtCtxt,
                             span: Span,
                             mitem: &MetaItem,
                             item: &Annotatable,
                             push: &mut FnMut(Annotatable))
{
    // check if we can use a short form
    //
    // the short form is `fn clone(&self) -> Self { *self }`
    //
    // we can use the short form if:
    // - the item is Copy (unfortunately, all we can check is whether it's also deriving Copy)
    // - there are no generic parameters (after specialization this limitation can be removed)
    //      if we used the short form with generics, we'd have to bound the generics with
    //      Clone + Copy, and then there'd be no Clone impl at all if the user fills in something
    //      that is Clone but not Copy. and until specialization we can't write both impls.
    let bounds;
    let substructure;
    let nested_match;
    match *item {
        Annotatable::Item(ref item) => {
            match item.node {
                ItemKind::Struct(_, Generics { ref ty_params, .. }) |
                ItemKind::Enum(_, Generics { ref ty_params, .. })
                    if ty_params.is_empty() && attr::contains_name(&item.attrs, "derive_Copy") => {

                    bounds = vec![Literal(path_std!(cx, core::marker::Copy))];
                    substructure = combine_substructure(Box::new(|c, s, _| {
                        cs_shallow_clone(c, s)
                    }));
                    nested_match = false;
                }

                _ => {
                    bounds = vec![];
                    substructure = combine_substructure(Box::new(|c, s, sub| {
                        cs_deep_clone("Clone", c, s, sub)
                    }));
                    nested_match = true;
                }
            }
        }

        _ => cx.span_bug(span, "#[derive(Clone)] on trait item or impl item")
    }

    let inline = cx.meta_word(span, InternedString::new("inline"));
    let attrs = vec!(cx.attribute(span, inline));
    let trait_def = TraitDef {
        span: span,
        attributes: Vec::new(),
        path: path_std!(cx, core::clone::Clone),
        additional_bounds: bounds,
        generics: LifetimeBounds::empty(),
        is_unsafe: false,
        methods: vec!(
            MethodDef {
                name: "clone",
                generics: LifetimeBounds::empty(),
                explicit_self: borrowed_explicit_self(),
                nested_match: nested_match,
                args: Vec::new(),
                ret_ty: Self_,
                attributes: attrs,
                is_unsafe: false,
                combine_substructure: substructure,
            }
        ),
        associated_types: Vec::new(),
    };

    trait_def.expand(cx, mitem, item, push)
}

fn cs_shallow_clone(cx: &mut ExtCtxt, trait_span: Span) -> P<Expr> {

    cx.expr_deref(trait_span, cx.expr_self(trait_span))
}

fn cs_deep_clone(
    name: &str,
    cx: &mut ExtCtxt, trait_span: Span,
    substr: &Substructure) -> P<Expr> {
    let ctor_path;
    let all_fields;
    let fn_path = cx.std_path(&["clone", "Clone", "clone"]);
    let subcall = |field: &FieldInfo| {
        let args = vec![cx.expr_addr_of(field.span, field.self_.clone())];

        cx.expr_call_global(field.span, fn_path.clone(), args)
    };

    let vdata;
    match *substr.fields {
        Struct(vdata_, ref af) => {
            ctor_path = cx.path(trait_span, vec![substr.type_ident]);
            all_fields = af;
            vdata = vdata_;
        }
        EnumMatching(_, variant, ref af) => {
            ctor_path = cx.path(trait_span, vec![substr.type_ident, variant.node.name]);
            all_fields = af;
            vdata = &variant.node.data;
        },
        EnumNonMatchingCollapsed (..) => {
            cx.span_bug(trait_span,
                        &format!("non-matching enum variants in \
                                 `derive({})`", name))
        }
        StaticEnum(..) | StaticStruct(..) => {
            cx.span_bug(trait_span,
                        &format!("static method in `derive({})`", name))
        }
    }

    match *vdata {
        VariantData::Struct(..) => {
            let fields = all_fields.iter().map(|field| {
                let ident = match field.name {
                    Some(i) => i,
                    None => {
                        cx.span_bug(trait_span,
                                    &format!("unnamed field in normal struct in \
                                             `derive({})`", name))
                    }
                };
                cx.field_imm(field.span, ident, subcall(field))
            }).collect::<Vec<_>>();

            cx.expr_struct(trait_span, ctor_path, fields)
        }
        VariantData::Tuple(..) => {
            let subcalls = all_fields.iter().map(subcall).collect();
            let path = cx.expr_path(ctor_path);
            cx.expr_call(trait_span, path, subcalls)
        }
        VariantData::Unit(..) => {
            cx.expr_path(ctor_path)
        }
    }
}
