use syn::{Attribute, Generics, Ident, MetaItem};

use {FromMetaItem, Result};
use codegen::FromFieldImpl;
use options::{Container, ForwardAttrs, ParseAttribute};
use util::IdentList;

pub struct FromFieldOptions {
    pub ident: Option<Ident>,
    pub vis: Option<Ident>,
    pub ty: Option<Ident>,
    pub attrs: Option<Ident>,
    pub attr_names: IdentList,
    pub container: Container,
    pub forward_attrs: Option<ForwardAttrs>,
}

impl FromFieldOptions {
    /// Create a new instance for the target name and generics.
    pub fn new(target_name: Ident, generics: Generics, attrs: &[Attribute]) -> Result<Self> {
        (FromFieldOptions {
            container: {
                let mut c = Container::from(target_name);
                c.generics = generics;
                c
            },
            ident: Default::default(),
            vis: Default::default(),
            ty: Default::default(),
            attrs: Default::default(),
            attr_names: Default::default(),
            forward_attrs: Default::default(),
        }).parse_attributes(attrs)
    }
}

impl ParseAttribute for FromFieldOptions {
    fn parse_nested(&mut self, mi: &MetaItem) -> Result<()> {
        match mi.name() {
            "attributes" => { self.attr_names = FromMetaItem::from_meta_item(mi)?; Ok(()) }
            "forward_attrs" => { self.forward_attrs = FromMetaItem::from_meta_item(mi)?; Ok(()) },
            _ => self.container.parse_nested(mi)
        }
    }
}

impl<'a> From<&'a FromFieldOptions> for FromFieldImpl<'a> {
    fn from(v: &'a FromFieldOptions) -> Self {
        FromFieldImpl {
            ident: v.ident.as_ref(),
            vis: v.vis.as_ref(),
            ty: v.ty.as_ref(),
            attrs: v.attrs.as_ref(),
            body: (&v.container).into(),
            attr_names: v.attr_names.as_strs(),
            forward_attrs: v.forward_attrs.as_ref(),
        }
    }
}