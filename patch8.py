with open("core/src/lib.rs", "r") as f:
    content = f.read()

content = content.replace('let mut chain = quote! { gpui::#tag_ident() };', 'let mut chain = quote! { gpui::#tag_ident() };')

with open("core/src/lib.rs", "w") as f:
    f.write(content)
