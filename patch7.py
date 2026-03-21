with open("core/src/lib.rs", "r") as f:
    content = f.read()

content = content.replace('let mut chain = quote! { gpui::#tag_ident() };', 'let mut chain = quote! { gpui::#tag_ident() };\n    if tag_name == "div" {\n        chain = quote! { gpui::#tag_ident().into_any_element().into_any() };\n    }')

with open("core/src/lib.rs", "w") as f:
    f.write(content)
