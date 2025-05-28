use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput, Fields, Type};

/// 为结构体自动实现 Task trait 和 Builder 模式
#[proc_macro_derive(Task, attributes(task_type))]
pub fn derive_maa_task(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let builder_name = format_ident!("{}Builder", name);

    // 获取结构体的字段
    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => panic!("MAATask 只支持命名字段的结构体"),
        },
        _ => panic!("MAATask 只支持结构体"),
    };

    // 生成 builder 字段（与结构体字段相同，但必选字段使用 Option 包装）
    let builder_fields = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        if is_option_type(ty) {
            quote! {
                #name: #ty
            }
        } else {
            quote! {
                #name: Option<#ty>
            }
        }
    });

    // 生成 builder 方法
    let builder_methods = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        let inner_ty = get_inner_type(ty);

        if is_option_type(ty) {
            if is_string_type(&inner_ty) {
                quote! {
                    pub fn #name(mut self, #name: impl Into<String>) -> Self {
                        self.#name = Some(#name.into());
                        self
                    }
                }
            } else {
                quote! {
                    pub fn #name(mut self, #name: #inner_ty) -> Self {
                        self.#name = Some(#name);
                        self
                    }
                }
            }
        } else {
            if is_string_type(ty) {
                quote! {
                    pub fn #name(mut self, #name: impl Into<String>) -> Self {
                        self.#name = Some(#name.into());
                        self
                    }
                }
            } else {
                quote! {
                    pub fn #name(mut self, #name: #ty) -> Self {
                        self.#name = Some(#name);
                        self
                    }
                }
            }
        }
    });

    // 生成 build 方法中的字段初始化
    let build_fields = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;

        if is_option_type(ty) {
            quote! {
                #name: self.#name.or_else(Default::default)
            }
        } else {
            quote! {
                #name: self.#name.expect(concat!("必选字段 ", stringify!(#name), " 未设置"))
            }
        }
    });

    // 获取任务类型
    let task_type = name.to_string().replace("Task", "");

    let expanded = quote! {
        impl Task for #name {
            fn task_type(&self) -> &'static str {
                #task_type
            }

            fn to_json(&self) -> String {
                serde_json::to_string(self).unwrap()
            }

            fn from_json(json: &str) -> Result<Self, serde_json::Error> {
                serde_json::from_str(json)
            }
        }

        #[derive(Default)]
        pub struct #builder_name {
            #(#builder_fields,)*
        }

        impl #name {
            pub fn new() -> Self {
                Self::builder().build()
            }

            pub fn builder() -> #builder_name {
                #builder_name::default()
            }
        }

        impl #builder_name {
            #(#builder_methods)*

            pub fn build(self) -> #name {
                #name {
                    #(#build_fields,)*
                }
            }
        }
    };

    TokenStream::from(expanded)
}

/// 检查类型是否为 Option<T>
fn is_option_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == "Option";
        }
    }
    false
}

/// 获取 Option<T> 中的 T 类型，如果不是 Option 则返回原类型
fn get_inner_type(ty: &Type) -> Type {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            if segment.ident == "Option" {
                if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let Some(syn::GenericArgument::Type(inner_ty)) = args.args.first() {
                        return inner_ty.clone();
                    }
                }
            }
        }
    }
    ty.clone()
}

fn is_string_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == "String";
        }
    }
    false
}
