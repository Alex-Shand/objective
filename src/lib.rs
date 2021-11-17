use proc::prelude::*;
use syn::{FnArg, ItemTrait, Pat, TraitItem};

#[proc::attribute]
fn objective(input: ItemTrait) -> TokenStream {
    let name = &input.ident;
    let items = match input
        .items
        .iter()
        .map(process_trait_item)
        .collect::<Result<Vec<_>, _>>()
    {
        Ok(items) => items,
        Err(err) => return err,
    };
    quote! {
        #input
        impl<T: #name + ?Sized> #name for Box<T> {
            #(#items)*
        }
    }
}

fn process_trait_item(item: &TraitItem) -> Result<TokenStream, TokenStream> {
    match item {
        TraitItem::Type(ty) => {
            let name = &ty.ident;
            Ok(quote! {
                type #name = T::#name;
            })
        }
        TraitItem::Method(m) => {
            let name = &m.sig.ident;
            let sig = &m.sig;

            let args = &m.sig.inputs.iter().collect::<Vec<_>>();
            if args.is_empty() {
                return Err(error_at!(sig, "Static trait methods aren't object safe"));
            }

            let receiver = match args[0] {
                FnArg::Receiver(r) => r,
                FnArg::Typed(_) => {
                    return Err(error_at!(sig, "Static trait methods aren't object safe"))
                }
            };

            let args = args[1..]
                .iter()
                .map(|arg| match arg {
                    FnArg::Receiver(_) => Err(error_at!(
                        arg,
                        "A self reference after the first argument should be impossible"
                    )),
                    FnArg::Typed(arg) => Ok(match &*arg.pat {
                        Pat::Ident(id) => &id.ident,
                        _ => panic!("Unexpected argument pattern"),
                    }),
                })
                .collect::<Result<Vec<_>, TokenStream>>();
            let args = match args {
                Ok(args) => args,
                Err(e) => return Err(e),
            };

            if receiver.reference.is_none() {
                return Err(error_at!(
                    receiver,
                    "All trait methods must accept &self or &mut self"
                ));
            }

            let get = if receiver.mutability.is_some() {
                quote!(as_mut)
            } else {
                quote!(as_ref)
            };

            Ok(quote! {
                #sig {
                    self.#get().#name(#(#args),*)
                }
            })
        }
        item => Err(error_at!(
            item,
            "Objective only supports types and functions in traits"
        )),
    }
}

proc::testutils::tests! {
    macro = objective;
    pass = "test/pass";
    fail = "test/fail";
}
