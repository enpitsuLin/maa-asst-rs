use crate::utils::{get_doc_attrs, get_inner_type, is_option_type, is_string_type};
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Error, Fields};

pub fn generate_task(input: DeriveInput) -> Result<proc_macro2::TokenStream, Error> {
    let name = &input.ident;
    let builder_name = format_ident!("{}Builder", name);

    // 获取 task 属性
    let task_attr = input
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("task"))
        .expect("必须提供 #[task(name = \"...\", task_type = \"...\")] 属性");

    let task_meta = task_attr.meta.require_list().expect("task 属性必须是一个列表");

    let mut task_name = None;
    let mut task_type = None;

    for nested in task_meta
        .parse_args_with(syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated)
        .expect("task 属性格式错误")
    {
        match nested {
            syn::Meta::NameValue(nv) => {
                if nv.path.is_ident("name") {
                    if let syn::Expr::Lit(syn::ExprLit {
                        lit: syn::Lit::Str(lit),
                        ..
                    }) = &nv.value
                    {
                        task_name = Some(lit.value());
                    }
                } else if nv.path.is_ident("task_type") {
                    if let syn::Expr::Lit(syn::ExprLit {
                        lit: syn::Lit::Str(lit),
                        ..
                    }) = &nv.value
                    {
                        task_type = Some(lit.value());
                    }
                }
            },
            _ => {
                return Err(Error::new_spanned(nested, "task 属性只支持 name 和 type 参数"));
            },
        }
    }

    let task_name = task_name.expect("必须提供 task name");
    let task_type = task_type.expect("必须提供 task type");

    // 获取结构体的字段
    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => return Err(Error::new_spanned(&input, "MAATask 只支持命名字段的结构体")),
        },
        _ => return Err(Error::new_spanned(&input, "MAATask 只支持结构体")),
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
        let doc = get_doc_attrs(&f.attrs);

        if is_option_type(ty) {
            if is_string_type(&inner_ty) {
                quote! {
                    #(#doc)*
                    pub fn #name(mut self, #name: impl Into<String>) -> Self {
                        self.#name = Some(#name.into());
                        self
                    }
                }
            } else {
                quote! {
                    #(#doc)*
                    pub fn #name(mut self, #name: #inner_ty) -> Self {
                        self.#name = Some(#name);
                        self
                    }
                }
            }
        } else {
            if is_string_type(ty) {
                quote! {
                    #(#doc)*
                    pub fn #name(mut self, #name: impl Into<String>) -> Self {
                        self.#name = Some(#name.into());
                        self
                    }
                }
            } else {
                quote! {
                    #(#doc)*
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

    Ok(quote! {
        impl Task for #name {
            fn task_type(&self) -> &'static str {
                #task_type
            }

            fn task_name(&self) -> &'static str {
                #task_name
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
    })
}
