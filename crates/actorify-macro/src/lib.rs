use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::{Attribute, FnArg, ImplItem, ItemImpl, Pat, parse_macro_input, parse_quote};

#[proc_macro_attribute]
pub fn actor(_args: TokenStream, item: TokenStream) -> TokenStream {
    let mut item = parse_macro_input!(item as ItemImpl);

    let actor_ty = item.self_ty.as_ref();
    let actor_name = actor_ty.to_token_stream().to_string();
    let cmd_ident = format_ident!("{}Cmd", &actor_name);
    let handle_ident = format_ident!("{}Handle", &actor_name);

    let mut cmd_variants = Vec::new();
    let mut handle_methods = Vec::new();
    let mut actor_cmd_arms = Vec::new();

    let mut actor_listener_arms = Vec::new();
    let mut actor_timer_inits = Vec::new();
    let mut actor_timer_arms = Vec::new();

    for impl_item in item.items.iter_mut() {
        match impl_item {
            ImplItem::Fn(impl_item_fn) => {
                if let Some(_) = find_and_remove_attr(&mut impl_item_fn.attrs, "command") {
                    let fn_name = &impl_item_fn.sig.ident;
                    let cmd_var_ident = format_ident!("{}", fn_name.to_string().to_case(Case::Pascal));

                    let mut cmd_arg_idents = Vec::new();
                    let mut cmd_arg_types = Vec::new();

                    let mut handle_args = Vec::new();

                    let mut handle_cb_name = None;
                    let mut reply_ty = None;

                    let mut i = 0u16;

                    for arg in impl_item_fn.sig.inputs.iter_mut() {
                        if let FnArg::Typed(arg) = arg {
                            let ident = if let Pat::Ident(ident) = arg.pat.as_ref() {
                                ident.ident.clone()
                            } else {
                                format_ident!("arg{}", i)
                            };

                            if let Some(_) = find_and_remove_attr(&mut arg.attrs, "callback") {
                                handle_cb_name = Some(ident.clone());
                                reply_ty = Some(arg.ty.as_ref().clone());
                                *arg.ty.as_mut() = parse_quote! { ::actorify::Callback<#reply_ty> };
                            } else {
                                let ty = arg.ty.as_ref();
                                handle_args.push(quote! { #ident: #ty });
                            }

                            cmd_arg_idents.push(ident.clone());
                            cmd_arg_types.push(arg.ty.as_ref().clone());

                            i += 1;
                        }
                    }

                    let handle_cb_name = handle_cb_name.expect("Callback not found");
                    let reply_ty = reply_ty.expect("Callback not found");

                    cmd_variants.push(quote! { #cmd_var_ident(#(#cmd_arg_types),*) });
                    handle_methods.push(quote! {
                        pub async fn #fn_name(&self, #(#handle_args),*) -> Result<#reply_ty, ::actorify::ChannelError> {
                            let (#handle_cb_name, rx) = ::actorify::Callback::new();

                            self.0.send(#cmd_ident::#cmd_var_ident(#(#cmd_arg_idents),*)).await?;

                            Ok(rx.await?)
                        }
                    });
                    actor_cmd_arms.push(quote! {
                        #cmd_ident::#cmd_var_ident(#(#cmd_arg_idents),*) => {
                            self.#fn_name(#(#cmd_arg_idents),*).await;
                        }
                    });
                } else if let Some(attr) = find_and_remove_attr(&mut impl_item_fn.attrs, "listen") {
                    let fn_name = &impl_item_fn.sig.ident;
                    let source = attr.to_token_stream();

                    actor_listener_arms.push(quote! {
                        Some(value) = #source => {
                            self.#fn_name(value).await;
                        }
                    });
                } else if let Some(attr) = find_and_remove_attr(&mut impl_item_fn.attrs, "every") {
                    let fn_name = &impl_item_fn.sig.ident;
                    let timer_ident = format_ident!("{}_timer", fn_name);
                    let frequency = attr.to_token_stream();

                    actor_timer_inits.push(quote! {
                        let mut #timer_ident = ::actorify::tokio::time::interval(#frequency);
                    });
                    actor_timer_arms.push(quote! {
                        _ = #timer_ident.tick() => {
                            self.#fn_name().await;
                        }
                    });
                }
            }
            _ => {}
        }
    }
    
    TokenStream::from(quote! {
        enum #cmd_ident {
            #(#cmd_variants,)*
        }

        #[derive(Clone)]
        pub struct #handle_ident(::actorify::Channel<#cmd_ident>);

        impl #handle_ident {
            #(#handle_methods)*
        }

        // impl ::actorify::ActorHandle<#cmd_ident> for #handle_ident {
        //     fn new(tx: ::actorify::Channel<#cmd_ident>) -> Self {
        //         Self(tx)
        //     }
        // }

        #item

        impl ::actorify::Actor for #actor_ty {
            type Handle = #handle_ident;
            fn run(mut self, token: ::actorify::tokio_util::sync::CancellationToken) -> (Self::Handle, impl Future<Output = ()> + Send) {
                let (tx, mut rx) = ::actorify::Channel::new(1024);
                
                (
                    #handle_ident(tx),
                    async move {
                        #(#actor_timer_inits)*
                        loop {
                            ::actorify::tokio::select! {
                                _ = token.cancelled() => {
                                    break;
                                }
                                #(#actor_timer_arms)*
                                #(#actor_listener_arms)*
                                Some(cmd) = rx.recv() => {
                                    match cmd {
                                        #(#actor_cmd_arms)*
                                    }
                                }
                            }
                        }
                    }
                )
            }
        }
    })
}

fn find_and_remove_attr(
    attrs: &mut Vec<Attribute>,
    name: &str,
) -> Option<Attribute> {
    let index = attrs
        .iter()
        .position(|a| a.path().is_ident(name))?;

    Some(attrs.remove(index))
}
