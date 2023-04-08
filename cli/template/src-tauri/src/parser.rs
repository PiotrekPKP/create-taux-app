// This is the parser that generates the TypeScript file from the Rust file.
// You probably don't want to modify it.

use convert_case::{Case, Casing};
use spinners::{Spinner, Spinners};
use std::{fs::read_to_string, process::exit};
use syn;

use crate::{COMMANDS_FILE_PATH, TS_FILE_PATH, TYPES_FILE_PATH};

pub fn generate_typescript_command_types() {
    let mut sp = Spinner::new(
        Spinners::Dots9,
        "Generating TypeScript command types...".into(),
    );

    let types_file_path = if cfg!(debug_assertions) {
        format!("../{TYPES_FILE_PATH}")
    } else {
        TYPES_FILE_PATH.to_string()
    };

    let commands_file_path = if cfg!(debug_assertions) {
        format!("../{COMMANDS_FILE_PATH}")
    } else {
        COMMANDS_FILE_PATH.to_string()
    };

    let types_file_text = read_to_string(types_file_path).unwrap_or("".into());
    let types_file_syntax = syn::parse_file(&types_file_text)
        .expect("Could not parse the `types.rs` file. Make sure it is valid Rust syntax.");
    let commands_file_text = read_to_string(commands_file_path)
        .expect("Could not read the `commands.rs` file. Make sure it exists.");
    let commands_file_syntax = syn::parse_file(&commands_file_text)
        .expect("Could not parse the `commands.rs` file. Make sure it is valid Rust syntax.");

    let mut types_output_text = String::new();
    let mut command_output_text = String::new();

    types_output_text.push_str(&create_initial_types());
    command_output_text.push_str("export type RustFunction = {");

    for item in types_file_syntax.items.iter() {
        println!("{:?}", item);
        match item {
            syn::Item::Type(item_type) => {
                let no_parse = item_type.attrs.iter().any(|attr| {
                    if let syn::Meta::Path(meta_path) = &attr.meta {
                        meta_path
                            .segments
                            .iter()
                            .find(|segment| segment.ident.to_string() == "no_parse")
                            .is_some()
                    } else {
                        false
                    }
                });

                if no_parse {
                    continue;
                }

                let type_text = parse_item_type(item_type);
                types_output_text.push_str(&type_text);
            }
            syn::Item::Enum(item_enum) => {
                let no_parse = item_enum.attrs.iter().any(|attr| {
                    if let syn::Meta::Path(meta_path) = &attr.meta {
                        meta_path
                            .segments
                            .iter()
                            .find(|segment| segment.ident.to_string() == "no_parse")
                            .is_some()
                    } else {
                        false
                    }
                });

                if no_parse {
                    continue;
                }

                let enum_text = parse_item_enum(item_enum);
                types_output_text.push_str(&enum_text);
            }
            syn::Item::Struct(item_struct) => {
                let no_parse = item_struct.attrs.iter().any(|attr| {
                    if let syn::Meta::Path(meta_path) = &attr.meta {
                        meta_path
                            .segments
                            .iter()
                            .find(|segment| segment.ident.to_string() == "no_parse")
                            .is_some()
                    } else {
                        false
                    }
                });

                if no_parse {
                    continue;
                }

                let struct_text = parse_item_struct(item_struct);
                types_output_text.push_str(&struct_text);
            }
            _ => {}
        }
    }

    for item in commands_file_syntax.items.iter() {
        match item {
            syn::Item::Type(item_type) => {
                let no_parse = item_type.attrs.iter().any(|attr| {
                    if let syn::Meta::Path(meta_path) = &attr.meta {
                        meta_path
                            .segments
                            .iter()
                            .find(|segment| segment.ident.to_string() == "no_parse")
                            .is_some()
                    } else {
                        false
                    }
                });

                if no_parse {
                    continue;
                }

                let type_text = parse_item_type(item_type);
                types_output_text.push_str(&type_text);
            }
            syn::Item::Enum(item_enum) => {
                let no_parse = item_enum.attrs.iter().any(|attr| {
                    if let syn::Meta::Path(meta_path) = &attr.meta {
                        meta_path
                            .segments
                            .iter()
                            .find(|segment| segment.ident.to_string() == "no_parse")
                            .is_some()
                    } else {
                        false
                    }
                });

                if no_parse {
                    continue;
                }

                let enum_text = parse_item_enum(item_enum);
                types_output_text.push_str(&enum_text);
            }
            syn::Item::Struct(item_struct) => {
                let no_parse = item_struct.attrs.iter().any(|attr| {
                    if let syn::Meta::Path(meta_path) = &attr.meta {
                        meta_path
                            .segments
                            .iter()
                            .find(|segment| segment.ident.to_string() == "no_parse")
                            .is_some()
                    } else {
                        false
                    }
                });

                if no_parse {
                    continue;
                }

                let struct_text = parse_item_struct(item_struct);
                types_output_text.push_str(&struct_text);
            }
            syn::Item::Fn(fun) => {
                let tauri_command = fun.attrs.iter().any(|attr| {
                    if let syn::Meta::Path(meta_path) = &attr.meta {
                        meta_path
                            .segments
                            .iter()
                            .filter(|segment| {
                                segment.ident.to_string() == "command"
                                    || segment.ident.to_string() == "tauri"
                            })
                            .count()
                            == 2
                    } else {
                        false
                    }
                });

                if !tauri_command {
                    continue;
                }

                let fn_name = fun.sig.ident.to_string();

                let arguments_string = if fun.sig.inputs.len() > 0 {
                    let mut args_string = String::from("{");

                    fun.sig.inputs.iter().for_each(|fn_arg| match fn_arg {
                        syn::FnArg::Typed(arg) => {
                            let (arg_name, arg_type) = parse_fn_argument(&arg);

                            if !arg_type.contains("AppState") && !arg_type.contains("AppHandler") {
                                args_string.push_str(&format!("{}: {}, ", arg_name, arg_type));
                            }
                        }
                        _ => {}
                    });

                    if args_string != "{" {
                        args_string.push_str("}");
                    } else {
                        args_string = "never".to_string();
                    }

                    args_string
                } else {
                    "never".into()
                };

                let return_string = match &fun.sig.output {
                    syn::ReturnType::Default => "void".into(),
                    syn::ReturnType::Type(_, return_type) => parse_type(&return_type),
                };

                if return_string.starts_with("Result<") {
                    let result_type = return_string.replace("Result<", "");
                    let result_type = result_type[..result_type.len() - 1].to_string();
                    let (success_type, error_type) = result_type
                        .split_once(",")
                        .unwrap_or(("void".into(), "never".into()));

                    command_output_text.push_str(&format!(
                        "{fn_name}: RustFunctionCreator<{arguments_string}, {success_type}, {error_type}>, "
                    ));
                } else {
                    command_output_text.push_str(&format!(
                        "{fn_name}: RustFunctionCreator<{arguments_string}, {return_string}, never>, "
                    ));
                }
            }
            _ => {}
        }
    }

    command_output_text.push_str("}");

    let merged_types = format!(
        "// This file was auto generated by the Rust app. Do not modify it.\n\n\n{}\n\n{}",
        types_output_text, command_output_text
    );

    let ts_file_path = if cfg!(debug_assertions) {
        format!("../{TS_FILE_PATH}")
    } else {
        TS_FILE_PATH.to_string()
    };

    std::fs::write(ts_file_path, merged_types)
        .expect("Could not write to the `types.ts` file. Make sure the path exists.");

    sp.stop_with_message("Generated TypeScript command types!".into());
}

fn create_initial_types() -> String {
    let mut output_text = String::new();

    output_text.push_str("type HashSet<T extends number | string> = Record<T, undefined>;");
    output_text.push_str("type HashMap<T extends number | string, U> = Record<T, U>;");
    output_text.push_str("type Vec<T> = Array<T>;");
    output_text.push_str("type Option<T> = T | undefined;");
    output_text.push_str("type Result<T, U> = T | U;");
    output_text
        .push_str("type RustFunctionCreator<Args, Return, Error> = { args: Args; return: Return; error: Error };");

    output_text
}

fn parse_fn_argument(fn_arg: &syn::PatType) -> (String, String) {
    let arg_name = if let syn::Pat::Ident(id) = &*fn_arg.pat {
        id.ident.to_string()
    } else {
        println!("Could not parse the ident of the argument.");
        exit(1);
    };

    let arg_type = parse_type(&fn_arg.ty);

    (arg_name.to_case(Case::Camel), arg_type)
}

fn parse_item_type(item_type: &syn::ItemType) -> String {
    let mut output_text = String::new();

    output_text.push_str(
        format!(
            "export type {type_name} = ",
            type_name = item_type.ident.to_string()
        )
        .as_str(),
    );

    let type_string = parse_type(&item_type.ty);
    output_text.push_str(&type_string);
    output_text.push_str(";");

    output_text
}

fn parse_item_enum(item_enum: &syn::ItemEnum) -> String {
    let mut output_text = String::new();

    output_text.push_str(
        format!(
            "export type {enum_name} = ",
            enum_name = item_enum.ident.to_string()
        )
        .as_str(),
    );

    let enum_string = parse_enum(&item_enum);
    output_text.push_str(&enum_string);
    output_text.push_str(";");

    output_text
}

fn parse_item_struct(item_struct: &syn::ItemStruct) -> String {
    let mut output_text = String::new();

    output_text.push_str(
        format!(
            "export interface {struct_name}",
            struct_name = item_struct.ident.to_string()
        )
        .as_str(),
    );

    let struct_string = parse_struct(&item_struct.fields);
    output_text.push_str(&struct_string);
    output_text.push_str(";");

    output_text
}

fn parse_type(syn_type: &syn::Type) -> String {
    let mut output_text = String::new();

    match syn_type {
        syn::Type::Path(type_path) => {
            let segment = type_path.path.segments.last().unwrap();

            let field_type = segment.ident.to_string();

            let ts_field_type = parse_type_ident(&field_type).to_owned();
            output_text.push_str(&ts_field_type);

            match &segment.arguments {
                syn::PathArguments::None => {}
                syn::PathArguments::AngleBracketed(angle_bracket_args) => {
                    output_text.push_str("<");
                    for (i, arg) in angle_bracket_args.args.iter().enumerate() {
                        match arg {
                            syn::GenericArgument::Type(inner_type) => {
                                output_text.push_str(&parse_type(inner_type));

                                if i != angle_bracket_args.args.len() - 1 {
                                    output_text.push_str(",");
                                }
                            }
                            _ => {}
                        }
                    }
                    output_text.push_str(">");
                }
                _ => {}
            }
        }
        syn::Type::Tuple(type_tuple) => {
            output_text.push_str("[");
            for elem in type_tuple.elems.iter() {
                output_text.push_str(&parse_type(elem));
                output_text.push_str(",");
            }
            output_text.push_str("]");
        }
        syn::Type::Reference(type_reference) => {
            output_text.push_str(&parse_type(&type_reference.elem));
        }
        syn::Type::Slice(type_slice) => {
            output_text.push_str(&parse_type(&type_slice.elem));
            output_text.push_str("[]");
        }
        _ => {}
    };

    output_text
}

fn parse_enum(item_enum: &syn::ItemEnum) -> String {
    let mut output_text = String::new();

    for variant in item_enum.variants.iter() {
        output_text.push_str(" | {");
        output_text.push_str(" ");

        output_text.push_str("t: \"");
        let variant_name = variant.ident.to_string();
        output_text.push_str(&variant_name);
        output_text.push_str("\" , c: ");

        match &variant.fields {
            syn::Fields::Named(named_fields) => {
                output_text.push_str("{");
                for field in named_fields.named.iter() {
                    if let Some(ident) = &field.ident {
                        output_text.push_str(&ident.to_string());
                        output_text.push_str(":");

                        let field_type = parse_type(&field.ty);
                        output_text.push_str(&field_type);
                        output_text.push_str(";");
                    }
                }
                output_text.push_str("}");
            }
            syn::Fields::Unnamed(unnamed_fields) => {
                let unnamed_field = unnamed_fields.unnamed.first().unwrap();
                let field_type = parse_type(&unnamed_field.ty);
                output_text.push_str(&field_type);
            }
            syn::Fields::Unit => {
                output_text.push_str("undefined");
            }
        }

        output_text.push_str("}");
    }

    output_text
}

fn parse_struct(fields: &syn::Fields) -> String {
    let mut output_text = String::new();

    output_text.push_str("{");
    match fields {
        syn::Fields::Named(named_fields) => {
            for named_field in named_fields.named.iter() {
                match &named_field.ident {
                    Some(ident) => {
                        let field_name = ident.to_string();
                        output_text.push_str(&field_name);
                        output_text.push_str(":");
                    }
                    None => {}
                }
                let field_type = parse_type(&named_field.ty);
                output_text.push_str(&field_type);
                output_text.push_str(";");
            }
        }
        syn::Fields::Unnamed(fields) => {
            for (index, field) in fields.unnamed.iter().enumerate() {
                output_text.push_str(&index.to_string());
                output_text.push_str(":");
                output_text.push_str(&parse_type(&field.ty));
                output_text.push_str(";");
            }
        }
        syn::Fields::Unit => (),
    }
    output_text.push_str("}");

    output_text
}

fn parse_type_ident(ident: &str) -> &str {
    match ident {
        "i8" | "i16" | "i32" | "i64" | "i128" | "u8" | "u16" | "u32" | "u64" | "f32" | "f64"
        | "isize" | "usize" => "number",
        "str" | "String" | "char" => "string",
        "bool" => "boolean",
        _ => ident,
    }
}
