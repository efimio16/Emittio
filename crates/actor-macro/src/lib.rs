use std::collections::HashMap;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Attribute, Expr, Fields, Ident, ImplItem, Item, ItemEnum, ItemFn, ItemImpl, ItemMod, ItemStruct, Token, Type, Variant, parse::ParseStream, parse_macro_input, parse_quote};

// const ACTOR_ATTR: &'static str = "actor";
const COMMANDS_ATTR: &'static str = "commands";
const HANDLE_ATTR: &'static str = "handle";
const SERVICE_ATTR: &'static str = "service";
const COMMAND_ATTR: &'static str = "command";
const LISTEN_ATTR: &'static str = "listen";
// const EVERY_ATTR: &'static str = "every";
const CALLBACK_ATTR: &'static str = "callback";

#[derive(Clone)]
struct Command {
    name: Ident,
    field_names: Vec<Ident>,
    field_types: Vec<Type>,
    reply: (Type, Type),
}

impl Command {
    fn from_and_modifying(item: &mut Variant) -> Self {
        let Fields::Named(fields) = &mut item.fields else {
            panic!("Command enum should contain only variants with named fields")
        };

        let (ok, _, err): (Type, Token![,], Type) = find_and_remove_attr(&mut item.attrs, CALLBACK_ATTR)
            .expect("Each command should have a reply type")
            .parse_args_with(|input: ParseStream| {
                Ok((
                    input.parse::<Type>()?,
                    input.parse::<Token![,]>()?,
                    input.parse::<Type>()?,
                ))
            })
            .expect("Failed to parse reply type");

        let mut field_names = vec![];
        let mut field_types = vec![];

        for field in fields.named.iter() {
            field_names.push(field.ident.clone().expect("Field should have a name"));
            field_types.push(field.ty.clone());
        }
        
        fields.named.push(parse_quote! { reply_tx: ::actor::Callback<#ok, #err> });

        Self {
            name: item.ident.clone(),
            field_names,
            field_types,
            reply: (ok, err),
        }
    }
}

#[derive(Clone)]
struct CommandList {
    name: Ident,
    list: Vec<Command>,
}

impl CommandList {
    fn from_and_modifying(item: &mut ItemEnum) -> Self {
        Self {
            name: item.ident.clone(),
            list: item.variants.iter_mut().map(|v| Command::from_and_modifying(v)).collect(),
        }
    }
}

struct Handle {
    name: Ident,
    commands: CommandList,
}

impl Handle {
    fn from_and_modifying(item: &mut ItemStruct, commands: CommandList) -> Self {
        let Fields::Unit = &mut item.fields else {
            panic!("Handler struct should be a unit")
        };

        let cmd_name = &commands.name;
        item.fields = Fields::Named(parse_quote! { { tx: ::actor::Channel<#cmd_name> } });

        Self {
            name: item.ident.clone(),
            commands,
        }
    }

    fn generate_impl(&self) -> Item {
        let handler_name = &self.name;
        let cmd_name = &self.commands.name;

        let methods = self.commands.list.iter().map(|c| -> ItemFn {
            let Command { name, field_names, field_types, reply: (ok, err) } = c;

            let method_name = format_ident!("{}", name.to_string().to_lowercase());

            parse_quote! {
                pub async fn #method_name(&self, #(#field_names: #field_types),*) -> Result<Result<#ok, #err>, ::actor::ChannelError> {
                    let (tx, rx) = ::actor::Callback::new();

                    self.tx.send(
                        #cmd_name::#name {
                            #( #field_names, )*
                            reply_tx: tx,
                        }
                    ).await?;

                    Ok(rx.await?)
                }
            }
        });

        parse_quote! {
            impl #handler_name {
                #(#methods)*
            }
        }
    }
}

struct ListenMethod {
    name: Ident,
    source: Expr,
}

struct Service {
    name: Ident,
    commands: CommandList,
    command_methods: HashMap<Ident, Ident>,
    // timer_methods: Vec<Ident>,
    listen_methods: Vec<ListenMethod>,
}

impl Service {
    fn from_and_modifying(item: &mut ItemStruct, item_impls: Vec<&mut ItemImpl>, commands: CommandList) -> Self {
        let Fields::Named(fields) = &mut item.fields else {
            panic!("Service struct should have named fields")
        };

        let cmd_name = &commands.name;
        fields.named.push(parse_quote! { rx: ::actor::mpsc::Receiver<#cmd_name> });

        let mut command_methods = HashMap::new();
        // let mut timer_methods = Vec::new();
        let mut listen_methods = Vec::new();

        for item_impl in item_impls {
            for impl_item in item_impl.items.iter_mut() {
                match impl_item {
                    ImplItem::Fn(item) => {
                        if let Some(attr) = find_and_remove_attr(&mut item.attrs, COMMAND_ATTR) {
                            let cmd = attr.parse_args().expect("Expected ident");

                            command_methods.insert(cmd, item.sig.ident.clone());
                        } else if let Some(attr) = find_and_remove_attr(&mut item.attrs, LISTEN_ATTR) {
                            let source = attr.parse_args().expect("Expected expression");

                            listen_methods.push(ListenMethod { name: item.sig.ident.clone(), source });
                        }
                    }
                    _ => {}
                }
            }
        }

        Self {
            name: item.ident.clone(),
            commands,
            command_methods,
            // timer_methods: Vec::new(),
            listen_methods,
        }
    }

    fn generate_impl(&self) -> Item {
        let handler_name = &self.name;

        let cmd_match_arms = self.commands.list.iter().filter_map(|c| {
            if let Some(method_name) = self.command_methods.get(&c.name) {
                let cmd_name = &self.commands.name;
                let Command { name, field_names, .. } = c;
                Some(quote! { #cmd_name::#name { #(#field_names,)* reply_tx } => self.#method_name(#(#field_names,)* reply_tx).await })
            } else {
                None
            }
        });

        let listeners = self.listen_methods.iter().map(|l| {
            let ListenMethod { name, source } = l;

            quote!(Some(data) = #source => self.#name(data))
        });

        parse_quote! {
            impl ::actor::Service for #handler_name {
                async fn run(mut self, token: ::actor::CancellationToken) -> Result<(), ::actor::ChannelError> {
                    loop {
                        ::actor::select! {
                            _ = token.cancelled() => { return Ok(()); },
                            Some(cmd) = self.rx.recv() => {
                                match cmd {
                                    #(#cmd_match_arms,)*
                                    _ => {},
                                }
                            }
                            #(#listeners,)*
                        }
                    }
                }
            }
        }
    }
}

#[proc_macro_attribute]
pub fn actor(_args: TokenStream, item: TokenStream) -> TokenStream {
    let mut module = parse_macro_input!(item as ItemMod);

    let Some((_, items)) = &mut module.content else {
        panic!("module should not be empty")
    };

    let mut commands_enum: Option<&mut ItemEnum> = None;
    let mut handle_struct: Option<&mut ItemStruct> = None;
    let mut service_struct: Option<&mut ItemStruct> = None;

    let mut service_impls: Vec<&mut ItemImpl> = vec![];

    for item in items.iter_mut() {
        match item {
            Item::Enum(item) => {
                if find_and_remove_attr(&mut item.attrs, COMMANDS_ATTR).is_some() {
                    commands_enum = Some(item);
                }
            }
            Item::Struct(item) => {
                if find_and_remove_attr(&mut item.attrs, HANDLE_ATTR).is_some() {
                    handle_struct = Some(item);
                } else if find_and_remove_attr(&mut item.attrs, SERVICE_ATTR).is_some() {
                    service_struct = Some(item);
                }
            }
            Item::Impl(item) => {
                if find_and_remove_attr(&mut item.attrs, SERVICE_ATTR).is_some() {
                    service_impls.push(item);
                }
            }
            _ => {}
        }
    }

    let commands_enum = commands_enum.expect("Commands enum not found");

    let handle_struct = handle_struct.expect("Handle struct not found");
    let service_struct = service_struct.expect("Service struct not found");

    let commands = CommandList::from_and_modifying(commands_enum);
    let handle = Handle::from_and_modifying(handle_struct, commands.clone());
    let service = Service::from_and_modifying(service_struct, service_impls, commands.clone());

    items.push(handle.generate_impl());
    items.push(service.generate_impl());

    TokenStream::from(quote! { #(#items)* })
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
