use std::fmt;
#[macro_export]
macro_rules! impl_token {
    ($struct_name:ident) => {
        impl AsRef<TokenContent> for $struct_name {
            fn as_ref(&self) -> &TokenContent {
                &self.content
            }
        }
        impl AsMut<TokenContent> for $struct_name {
            fn as_mut(&mut self) -> &mut TokenContent {
                &mut self.content
            }
        }
        impl fmt::Display for $struct_name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.content.fmt(f)
            }
        }
        impl TryFrom<char> for $struct_name {
            type Error = ParseError;
            fn try_from(c: char) -> Result<Self> {
                TokenContent::from(c).try_into()
            }
        }
        impl TryFrom<TokenContent> for $struct_name {
            type Error = ParseError;
            fn try_from(t: TokenContent) -> Result<Self> {
                Ok(Self{
                    content: t,
                })
            }
        }
        impl TryFrom<&str> for $struct_name {
            type Error = ParseError;
            fn try_from(s: &str) -> Result<Self> {
                TokenContent::from(s).try_into()
            }
        }
        impl TryFrom<String> for $struct_name {
            type Error = ParseError;
            fn try_from(s: String) -> Result<Self> {
                TokenContent::from(s).try_into()
            }
        }
        impl TryFrom<(usize, usize, &str)> for $struct_name {
            type Error = ParseError;
            fn try_from(t: (usize, usize, &str)) -> Result<Self> {
                TokenContent::from(t).try_into()
            }
        }
    };
    ($enum_name:ident {$($child_name:ident,$child_type:ty,)+}) => {
        impl AsRef<TokenContent> for $enum_name {
            fn as_ref(&self) -> &TokenContent {
                match self {
                    $(Self::$child_name(x) => x.as_ref(),)+
                }
            }
        }
        impl AsMut<TokenContent> for $enum_name {
            fn as_mut(&mut self) -> &mut TokenContent {
                match self {
                    $(Self::$child_name(x) => x.as_mut(),)+
                }
            }
        }
        impl fmt::Display for $enum_name{
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    $(Self::$child_name(x) => x.fmt(f),)+
                }
            }
        }
        $(
            impl From<$child_type> for $enum_name {
                fn from(t: $child_type) -> Self {
                    Self::$child_name(t)
                }
            }
        )+
    };
}
macro_rules! impl_enum_is {
    ($enum_name:ident {$($child_name:ident,$child_type:ty,)+}) => {
        impl $enum_name {
            $(
                impl From<$child_type> for $enum_name {
                    fn from(t: $child_type) -> Self {
                        Self::$child_name(t)
                    }
                }
            )+

        }
    };
}


macro_rules! impl_list_item {
    ($struct_name:ident) => {
        
        impl fmt::Display for $struct_name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.content.fmt(f)
            }
        }
    };
    ($enum_name:ident {$($child_name:ident,$child_type:ty,)+}) => {
        impl AsRef<TokenContent> for $enum_name {
            fn as_ref(&self) -> &TokenContent {
                match self {
                    $(Self::$child_name(x) => x.as_ref(),)+
                }
            }
        }
        impl AsMut<TokenContent> for $enum_name {
            fn as_mut(&mut self) -> &mut TokenContent {
                match self {
                    $(Self::$child_name(x) => x.as_mut(),)+
                }
            }
        }
        impl fmt::Display for $enum_name{
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    $(Self::$child_name(x) => x.fmt(f),)+
                }
            }
        }
        $(
            impl From<$child_type> for $enum_name {
                fn from(t: $child_type) -> Self {
                    Self::$child_name(t)
                }
            }
        )+
    };
}

