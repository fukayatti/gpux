with open("demo_app/src/main.rs", "r") as f:
    content = f.read()

content = content.replace('use gpui::{', 'use gpui::{InteractiveElement, ParentElement, Styled, State, Global, ')

with open("demo_app/src/main.rs", "w") as f:
    f.write(content)
