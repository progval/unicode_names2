//! A macro that maps unicode names to chars and strings.

#![crate_type = "dylib"]

extern crate unicode_names2;

#[macro_use]
extern crate syn;

struct CharByName(syn::LitChar);

impl syn::parse::Parse for CharByName {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let string: syn::LitStr = input.parse()?;
        let name = string.value();
        match unicode_names2::character(&name) {
            None => Err(syn::Error::new(
                string.span(),
                format!("`{}` does not name a character", name),
            )),
            Some(c) => Ok(CharByName(syn::LitChar::new(c, string.span()))),
        }
    }
}

#[proc_macro]
pub fn named_char(stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let CharByName(c) = parse_macro_input!(stream as CharByName);
    proc_macro::TokenTree::Literal(proc_macro::Literal::character(c.value())).into()
}

struct StringWithCharNames(syn::LitStr);

impl syn::parse::Parse for StringWithCharNames {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let string: syn::LitStr = input.parse()?;
        // make sure unclosed braces don't escape.
        let names_re = regex::Regex::new(r"\\N\{(.*?)(?:\}|$)").unwrap();

        let mut errors = Vec::new();
        let new = names_re.replace_all(&string.value(), |c: &regex::Captures| {
            let full = c.at(0).unwrap();
            if !full.ends_with('}') {
                errors.push(format!("unclosed escape in `named!`: {}", full));
            } else {
                let name = c.at(1).unwrap();
                match unicode_names2::character(name) {
                    Some(c) => return c.to_string(),
                    None => {
                        errors.push(format!("`{}` does not name a character", name));
                    }
                }
            }
            // failed :(
            String::new()
        });
        if !errors.is_empty() {
            // TODO: show all errors at once?
            Err(syn::Error::new(string.span(), errors.get(0).unwrap()))
        } else {
            Ok(StringWithCharNames(syn::LitStr::new(&new, string.span())))
        }
    }
}

#[proc_macro]
pub fn named(stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let StringWithCharNames(s) = parse_macro_input!(stream as StringWithCharNames);
    proc_macro::TokenTree::Literal(proc_macro::Literal::string(&s.value())).into()
}
