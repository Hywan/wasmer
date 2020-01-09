extern crate proc_macro;

use proc_macro::{Literal, TokenStream, TokenTree};
use quote::quote;
use syn::{
    parse::{Parse, ParseStream, Result},
    parse_macro_input, Error, Ident, LitStr, Token,
};

struct DeprecatedArguments {
    since: LitStr,
    note: LitStr,
}

impl Parse for DeprecatedArguments {
    fn parse(input: ParseStream) -> Result<Self> {
        let key = input.parse::<Ident>()?;

        if key != "since" {
            return Err(Error::new(key.span(), "expected `since`"));
        }

        input.parse::<Token![=]>()?;
        let since = input.parse()?;

        input.parse::<Token![,]>()?;

        let key = input.parse::<Ident>()?;

        if key != "note" {
            return Err(Error::new(key.span(), "expected `note`"));
        }

        input.parse::<Token![=]>()?;
        let note = input.parse()?;

        Ok(DeprecatedArguments { since, note })
    }
}

#[proc_macro_attribute]
pub fn cbindgen_deprecated(arguments: TokenStream, input: TokenStream) -> TokenStream {
    let arguments = parse_macro_input!(arguments as DeprecatedArguments);
    let since = arguments.since;
    let note = arguments.note;
    let cbindgen_note = "cbindgen:prefix=CBINDGEN_DEPRECATED(\"".to_owned()
        + &note.value().replace("\"", "\\\"")
        + "\")";

    let mut output = TokenStream::new();
    let doc = quote!(
        #[doc(#cbindgen_note)]
        #[deprecated(since = #since, note = #note)]
    );

    output.extend(TokenStream::from(doc));
    output.extend(input);

    output
}

const C_CPP_MACROS: &str = r###"
#if !defined(CBINDGEN_DEPRECATED)

// Compatibility with non-Clang compilers.
#if !defined(__has_attribute)
#  define __has_attribute(x) 0
#endif

// Compatibility with non-Clang compilers.
#if !defined(__has_declspec_attribute)
#  define __has_declspec_attribute(x) 0
#endif

// Define the `DEPRECATED` macro.
#if defined(GCC) || defined(__GNUC__) || __has_attribute(deprecated)
#  define CBINDGEN_DEPRECATED(message) __attribute__((deprecated(message)))
#elif defined(MSVC) || __has_declspec_attribute(deprecated)
#  define CBINDGEN_DEPRECATED(message) __declspec(deprecated(message))
#endif

#endif
"###;

#[proc_macro]
pub fn c_cpp_macros(_: TokenStream) -> TokenStream {
    TokenStream::from(TokenTree::Literal(Literal::string(C_CPP_MACROS)))
}
