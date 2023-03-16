extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_derive(Responder)]
pub fn responder(input: TokenStream) -> TokenStream {
    let ast = syn::parse::<syn::DeriveInput>(input).unwrap();
    derive_responder_impl(&ast)
}

fn derive_responder_impl(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let generator = quote::quote! {
        impl actix_web::Responder for #name {
            type Body = actix_web::body::BoxBody;
            fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
                actix_web::HttpResponse::Ok()
                    .content_type(actix_web::http::header::ContentType::json())
                    .json(self)
            }
        }
    };
    generator.into()
}
