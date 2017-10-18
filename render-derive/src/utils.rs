use syn;
use quote;

pub fn convert_snake_to_camel_case(intput: &str) -> String {
    let mut is_snake = true;
    let mut result = String::new();
    result.reserve(intput.len());

    for c in intput.chars() {
        match c {
            '_' if !is_snake => {
                is_snake = true;
            }

            c => {
                if is_snake {
                    for u in c.to_uppercase() {
                        result.push(u);
                    }
                } else {
                    result.push(c);
                }
                is_snake = false;
            }
        }
    }
    result
}


pub fn impl_offset_of(container: &syn::Ident, field: &syn::Ident) -> quote::Tokens {
    quote! { unsafe {
        use std::mem;
        // Make sure the field actually exists. This line ensures that a
        // compile-time error is generated if $field is accessed through a
        // Deref impl.
        #[allow(unused_variables)]
        let #container { #field: _, .. };

        // Create an instance of the container and calculate the offset to its
        // field. Although we are creating references to uninitialized data this
        // is fine since we are not dereferencing them.
        let val: #container = mem::uninitialized();
        let result = &val.#field as *const _ as usize - &val as *const _ as usize;
        mem::forget(val);

        result as usize
    } }
}