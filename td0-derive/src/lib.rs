// SPDX-FileCopyrightText: © 2026 Nathan Pugh <natepugh@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

mod common;
mod repeat;
mod td0_struct;

#[proc_macro_derive(TD0ChunkItemDerive, attributes(td0_field))]
pub fn td0_struct_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    td0_struct::td0_struct_derive(input)
}

#[proc_macro_attribute]
pub fn repeat_fields(
    _args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    repeat::repeat_fields(_args, input)
}
