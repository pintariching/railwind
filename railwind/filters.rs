mod filters {
    use macro_derive::{ConfigurableParser, KeywordConfigurableParser};
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::combinator::{map, map_opt};
    use nom::sequence::preceded;
    use nom::IResult;
    use crate::class::Decl;
    use crate::class::utils::neg_keyword_value;
    use crate::class::IntoDeclaration;
    use crate::config::Config;
    const FILTER_STYLE: &str = "filter: var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)";
    const WEBKIT_BACKDROP_FILTER_STYLE: &str = "-webkit-backdrop-filter: var(--tw-backdrop-blur) var(--tw-backdrop-brightness) var(--tw-backdrop-contrast) var(--tw-backdrop-grayscale) var(--tw-backdrop-hue-rotate) var(--tw-backdrop-invert) var(--tw-backdrop-opacity) var(--tw-backdrop-saturate) var(--tw-backdrop-sepia)";
    const BACKDROP_FILTER_STYLE: &str = "        backdrop-filter: var(--tw-backdrop-blur) var(--tw-backdrop-brightness) var(--tw-backdrop-contrast) var(--tw-backdrop-grayscale) var(--tw-backdrop-hue-rotate) var(--tw-backdrop-invert) var(--tw-backdrop-opacity) var(--tw-backdrop-saturate) var(--tw-backdrop-sepia)";
    pub enum Filter<'a> {
        Blur(Blur<'a>),
        Brightness(Brightness<'a>),
        Contrast(Contrast<'a>),
        DropShadow(DropShadow<'a>),
        Grayscale(Grayscale<'a>),
        HueRotate(HueRotate),
        Invert(Invert<'a>),
        Saturate(Saturate<'a>),
        Sepia(Sepia<'a>),
        BackdropBlur(BackdropBlur<'a>),
        BackdropBrightness(BackdropBrightness<'a>),
        BackdropContrast(BackdropContrast<'a>),
        BackdropGrayscale(BackdropGrayscale<'a>),
        BackdropHueRotate(BackdropHueRotate),
        BackdropInvert(BackdropInvert<'a>),
        BackdropOpacity(BackdropOpacity<'a>),
        BackdropSaturate(BackdropSaturate<'a>),
        BackdropSepia(BackdropSepia<'a>),
    }
    #[automatically_derived]
    impl<'a> ::core::fmt::Debug for Filter<'a> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                Filter::Blur(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Blur",
                        &__self_0,
                    )
                }
                Filter::Brightness(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Brightness",
                        &__self_0,
                    )
                }
                Filter::Contrast(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Contrast",
                        &__self_0,
                    )
                }
                Filter::DropShadow(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "DropShadow",
                        &__self_0,
                    )
                }
                Filter::Grayscale(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Grayscale",
                        &__self_0,
                    )
                }
                Filter::HueRotate(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "HueRotate",
                        &__self_0,
                    )
                }
                Filter::Invert(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Invert",
                        &__self_0,
                    )
                }
                Filter::Saturate(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Saturate",
                        &__self_0,
                    )
                }
                Filter::Sepia(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Sepia",
                        &__self_0,
                    )
                }
                Filter::BackdropBlur(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "BackdropBlur",
                        &__self_0,
                    )
                }
                Filter::BackdropBrightness(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "BackdropBrightness",
                        &__self_0,
                    )
                }
                Filter::BackdropContrast(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "BackdropContrast",
                        &__self_0,
                    )
                }
                Filter::BackdropGrayscale(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "BackdropGrayscale",
                        &__self_0,
                    )
                }
                Filter::BackdropHueRotate(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "BackdropHueRotate",
                        &__self_0,
                    )
                }
                Filter::BackdropInvert(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "BackdropInvert",
                        &__self_0,
                    )
                }
                Filter::BackdropOpacity(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "BackdropOpacity",
                        &__self_0,
                    )
                }
                Filter::BackdropSaturate(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "BackdropSaturate",
                        &__self_0,
                    )
                }
                Filter::BackdropSepia(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "BackdropSepia",
                        &__self_0,
                    )
                }
            }
        }
    }
    #[automatically_derived]
    impl<'a> ::core::marker::StructuralPartialEq for Filter<'a> {}
    #[automatically_derived]
    impl<'a> ::core::cmp::PartialEq for Filter<'a> {
        #[inline]
        fn eq(&self, other: &Filter<'a>) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
                && match (self, other) {
                    (Filter::Blur(__self_0), Filter::Blur(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    (Filter::Brightness(__self_0), Filter::Brightness(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    (Filter::Contrast(__self_0), Filter::Contrast(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    (Filter::DropShadow(__self_0), Filter::DropShadow(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    (Filter::Grayscale(__self_0), Filter::Grayscale(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    (Filter::HueRotate(__self_0), Filter::HueRotate(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    (Filter::Invert(__self_0), Filter::Invert(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    (Filter::Saturate(__self_0), Filter::Saturate(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    (Filter::Sepia(__self_0), Filter::Sepia(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    (Filter::BackdropBlur(__self_0), Filter::BackdropBlur(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    (
                        Filter::BackdropBrightness(__self_0),
                        Filter::BackdropBrightness(__arg1_0),
                    ) => *__self_0 == *__arg1_0,
                    (
                        Filter::BackdropContrast(__self_0),
                        Filter::BackdropContrast(__arg1_0),
                    ) => *__self_0 == *__arg1_0,
                    (
                        Filter::BackdropGrayscale(__self_0),
                        Filter::BackdropGrayscale(__arg1_0),
                    ) => *__self_0 == *__arg1_0,
                    (
                        Filter::BackdropHueRotate(__self_0),
                        Filter::BackdropHueRotate(__arg1_0),
                    ) => *__self_0 == *__arg1_0,
                    (
                        Filter::BackdropInvert(__self_0),
                        Filter::BackdropInvert(__arg1_0),
                    ) => *__self_0 == *__arg1_0,
                    (
                        Filter::BackdropOpacity(__self_0),
                        Filter::BackdropOpacity(__arg1_0),
                    ) => *__self_0 == *__arg1_0,
                    (
                        Filter::BackdropSaturate(__self_0),
                        Filter::BackdropSaturate(__arg1_0),
                    ) => *__self_0 == *__arg1_0,
                    (
                        Filter::BackdropSepia(__self_0),
                        Filter::BackdropSepia(__arg1_0),
                    ) => *__self_0 == *__arg1_0,
                    _ => unsafe { ::core::intrinsics::unreachable() }
                }
        }
    }
    #[automatically_derived]
    impl<'a> ::core::hash::Hash for Filter<'a> {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            ::core::hash::Hash::hash(&__self_tag, state);
            match self {
                Filter::Blur(__self_0) => ::core::hash::Hash::hash(__self_0, state),
                Filter::Brightness(__self_0) => ::core::hash::Hash::hash(__self_0, state),
                Filter::Contrast(__self_0) => ::core::hash::Hash::hash(__self_0, state),
                Filter::DropShadow(__self_0) => ::core::hash::Hash::hash(__self_0, state),
                Filter::Grayscale(__self_0) => ::core::hash::Hash::hash(__self_0, state),
                Filter::HueRotate(__self_0) => ::core::hash::Hash::hash(__self_0, state),
                Filter::Invert(__self_0) => ::core::hash::Hash::hash(__self_0, state),
                Filter::Saturate(__self_0) => ::core::hash::Hash::hash(__self_0, state),
                Filter::Sepia(__self_0) => ::core::hash::Hash::hash(__self_0, state),
                Filter::BackdropBlur(__self_0) => {
                    ::core::hash::Hash::hash(__self_0, state)
                }
                Filter::BackdropBrightness(__self_0) => {
                    ::core::hash::Hash::hash(__self_0, state)
                }
                Filter::BackdropContrast(__self_0) => {
                    ::core::hash::Hash::hash(__self_0, state)
                }
                Filter::BackdropGrayscale(__self_0) => {
                    ::core::hash::Hash::hash(__self_0, state)
                }
                Filter::BackdropHueRotate(__self_0) => {
                    ::core::hash::Hash::hash(__self_0, state)
                }
                Filter::BackdropInvert(__self_0) => {
                    ::core::hash::Hash::hash(__self_0, state)
                }
                Filter::BackdropOpacity(__self_0) => {
                    ::core::hash::Hash::hash(__self_0, state)
                }
                Filter::BackdropSaturate(__self_0) => {
                    ::core::hash::Hash::hash(__self_0, state)
                }
                Filter::BackdropSepia(__self_0) => {
                    ::core::hash::Hash::hash(__self_0, state)
                }
            }
        }
    }
    pub fn filter<'a>(
        input: &'a str,
        config: &'a Config,
    ) -> IResult<&'a str, Filter<'a>> {
        alt((
            preceded(tag("blur-"), map(|i| blur(i, config), Filter::Blur)),
            map_opt(
                tag("blur"),
                |_| { config.filters.get_blur().get("").map(|v| Filter::Blur(Blur(v))) },
            ),
        ))(input)
    }
    impl<'a> IntoDeclaration for Filter<'a> {
        fn to_decl(self) -> Decl {
            match self {
                Filter::Blur(f) => f.to_decl(),
                Filter::Brightness(f) => f.to_decl(),
                Filter::Contrast(f) => f.to_decl(),
                Filter::DropShadow(f) => f.to_decl(),
                Filter::Grayscale(f) => f.to_decl(),
                Filter::HueRotate(f) => f.to_decl(),
                Filter::Invert(f) => f.to_decl(),
                Filter::Saturate(f) => f.to_decl(),
                Filter::Sepia(f) => f.to_decl(),
                Filter::BackdropBlur(f) => f.to_decl(),
                Filter::BackdropBrightness(f) => f.to_decl(),
                Filter::BackdropContrast(f) => f.to_decl(),
                Filter::BackdropGrayscale(f) => f.to_decl(),
                Filter::BackdropHueRotate(f) => f.to_decl(),
                Filter::BackdropInvert(f) => f.to_decl(),
                Filter::BackdropOpacity(f) => f.to_decl(),
                Filter::BackdropSaturate(f) => f.to_decl(),
                Filter::BackdropSepia(f) => f.to_decl(),
            }
        }
    }
    #[name(blur)]
    #[keyword("blur")]
    #[no_args(false)]
    #[config(filters.get_blur)]
    pub struct Blur<'a>(pub &'a str);
    #[automatically_derived]
    impl<'a> ::core::fmt::Debug for Blur<'a> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Blur", &&self.0)
        }
    }
    #[automatically_derived]
    impl<'a> ::core::marker::StructuralPartialEq for Blur<'a> {}
    #[automatically_derived]
    impl<'a> ::core::cmp::PartialEq for Blur<'a> {
        #[inline]
        fn eq(&self, other: &Blur<'a>) -> bool {
            self.0 == other.0
        }
    }
    #[automatically_derived]
    impl<'a> ::core::hash::Hash for Blur<'a> {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.0, state)
        }
    }
    fn blur<'a>(
        input: &'a str,
        config: &'a crate::config::Config,
    ) -> nom::IResult<&'a str, Blur<'a>> {
        nom::branch::alt((
            nom::sequence::preceded(
                nom::sequence::preceded(
                    nom::bytes::complete::tag("blur"),
                    nom::bytes::complete::tag("-"),
                ),
                nom::combinator::map(
                    nom::branch::alt((
                        nom::sequence::delimited(
                            nom::bytes::complete::tag("["),
                            nom::bytes::complete::is_not("]"),
                            nom::bytes::complete::tag("]"),
                        ),
                        nom::combinator::map_opt(
                            nom::bytes::complete::is_not(" "),
                            |v| config.filters.get_blur().get(v).copied(),
                        ),
                    )),
                    Blur,
                ),
            ),
            nom::combinator::map_opt(
                nom::bytes::complete::tag("blur"),
                |_| config.filters.get_blur().get("").map(|v| Blur),
            ),
        ))(input)
    }
    impl<'a> IntoDeclaration for Blur<'a> {
        fn to_decl(self) -> Decl {
            Decl::Vec(
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        {
                            let res = ::alloc::fmt::format(
                                format_args!("--tw-blur: blur({0})", self.0),
                            );
                            res
                        },
                        FILTER_STYLE.into(),
                    ]),
                ),
            )
        }
    }
    #[name(brightness)]
    #[config(filters.get_brightness)]
    pub struct Brightness<'a>(pub &'a str);
    #[automatically_derived]
    impl<'a> ::core::fmt::Debug for Brightness<'a> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Brightness", &&self.0)
        }
    }
    #[automatically_derived]
    impl<'a> ::core::marker::StructuralPartialEq for Brightness<'a> {}
    #[automatically_derived]
    impl<'a> ::core::cmp::PartialEq for Brightness<'a> {
        #[inline]
        fn eq(&self, other: &Brightness<'a>) -> bool {
            self.0 == other.0
        }
    }
    #[automatically_derived]
    impl<'a> ::core::hash::Hash for Brightness<'a> {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.0, state)
        }
    }
    fn brightness<'a>(
        input: &'a str,
        config: &'a crate::config::Config,
    ) -> nom::IResult<&'a str, Brightness<'a>> {
        nom::combinator::map(
            nom::branch::alt((
                nom::sequence::delimited(
                    nom::bytes::complete::tag("["),
                    nom::bytes::complete::is_not("]"),
                    nom::bytes::complete::tag("]"),
                ),
                nom::combinator::map_opt(
                    nom::bytes::complete::is_not(" "),
                    |v| config.filters.get_brightness().get(v).copied(),
                ),
            )),
            Brightness,
        )(input)
    }
    impl<'a> IntoDeclaration for Brightness<'a> {
        fn to_decl(self) -> Decl {
            Decl::Vec(
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        {
                            let res = ::alloc::fmt::format(
                                format_args!("--tw-brightness: brightness({0})", self.0),
                            );
                            res
                        },
                        FILTER_STYLE.into(),
                    ]),
                ),
            )
        }
    }
    #[name(contrast)]
    #[config(filters.get_contrast)]
    pub struct Contrast<'a>(pub &'a str);
    #[automatically_derived]
    impl<'a> ::core::fmt::Debug for Contrast<'a> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Contrast", &&self.0)
        }
    }
    #[automatically_derived]
    impl<'a> ::core::marker::StructuralPartialEq for Contrast<'a> {}
    #[automatically_derived]
    impl<'a> ::core::cmp::PartialEq for Contrast<'a> {
        #[inline]
        fn eq(&self, other: &Contrast<'a>) -> bool {
            self.0 == other.0
        }
    }
    #[automatically_derived]
    impl<'a> ::core::hash::Hash for Contrast<'a> {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.0, state)
        }
    }
    fn contrast<'a>(
        input: &'a str,
        config: &'a crate::config::Config,
    ) -> nom::IResult<&'a str, Contrast<'a>> {
        nom::combinator::map(
            nom::branch::alt((
                nom::sequence::delimited(
                    nom::bytes::complete::tag("["),
                    nom::bytes::complete::is_not("]"),
                    nom::bytes::complete::tag("]"),
                ),
                nom::combinator::map_opt(
                    nom::bytes::complete::is_not(" "),
                    |v| config.filters.get_contrast().get(v).copied(),
                ),
            )),
            Contrast,
        )(input)
    }
    impl<'a> IntoDeclaration for Contrast<'a> {
        fn to_decl(self) -> Decl {
            Decl::Vec(
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        {
                            let res = ::alloc::fmt::format(
                                format_args!("--tw-contrast: contrast({0})", self.0),
                            );
                            res
                        },
                        FILTER_STYLE.into(),
                    ]),
                ),
            )
        }
    }
    #[name(drop_shadow)]
    #[config(filters.get_drop_shadow)]
    pub struct DropShadow<'a>(pub &'a str);
    #[automatically_derived]
    impl<'a> ::core::fmt::Debug for DropShadow<'a> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(f, "DropShadow", &&self.0)
        }
    }
    #[automatically_derived]
    impl<'a> ::core::marker::StructuralPartialEq for DropShadow<'a> {}
    #[automatically_derived]
    impl<'a> ::core::cmp::PartialEq for DropShadow<'a> {
        #[inline]
        fn eq(&self, other: &DropShadow<'a>) -> bool {
            self.0 == other.0
        }
    }
    #[automatically_derived]
    impl<'a> ::core::hash::Hash for DropShadow<'a> {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.0, state)
        }
    }
    fn drop_shadow<'a>(
        input: &'a str,
        config: &'a crate::config::Config,
    ) -> nom::IResult<&'a str, DropShadow<'a>> {
        nom::combinator::map(
            nom::branch::alt((
                nom::sequence::delimited(
                    nom::bytes::complete::tag("["),
                    nom::bytes::complete::is_not("]"),
                    nom::bytes::complete::tag("]"),
                ),
                nom::combinator::map_opt(
                    nom::bytes::complete::is_not(" "),
                    |v| config.filters.get_drop_shadow().get(v).copied(),
                ),
            )),
            DropShadow,
        )(input)
    }
    impl<'a> IntoDeclaration for DropShadow<'a> {
        fn to_decl(self) -> Decl {
            Decl::Vec(
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        {
                            let res = ::alloc::fmt::format(
                                format_args!("--tw-drop-shadow: drop-shadow({0})", self.0),
                            );
                            res
                        },
                        FILTER_STYLE.into(),
                    ]),
                ),
            )
        }
    }
    #[name(grayscale)]
    #[config(filters.get_grayscale)]
    pub struct Grayscale<'a>(pub &'a str);
    #[automatically_derived]
    impl<'a> ::core::fmt::Debug for Grayscale<'a> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Grayscale", &&self.0)
        }
    }
    #[automatically_derived]
    impl<'a> ::core::marker::StructuralPartialEq for Grayscale<'a> {}
    #[automatically_derived]
    impl<'a> ::core::cmp::PartialEq for Grayscale<'a> {
        #[inline]
        fn eq(&self, other: &Grayscale<'a>) -> bool {
            self.0 == other.0
        }
    }
    #[automatically_derived]
    impl<'a> ::core::hash::Hash for Grayscale<'a> {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.0, state)
        }
    }
    fn grayscale<'a>(
        input: &'a str,
        config: &'a crate::config::Config,
    ) -> nom::IResult<&'a str, Grayscale<'a>> {
        nom::combinator::map(
            nom::branch::alt((
                nom::sequence::delimited(
                    nom::bytes::complete::tag("["),
                    nom::bytes::complete::is_not("]"),
                    nom::bytes::complete::tag("]"),
                ),
                nom::combinator::map_opt(
                    nom::bytes::complete::is_not(" "),
                    |v| config.filters.get_grayscale().get(v).copied(),
                ),
            )),
            Grayscale,
        )(input)
    }
    impl<'a> IntoDeclaration for Grayscale<'a> {
        fn to_decl(self) -> Decl {
            Decl::Vec(
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        {
                            let res = ::alloc::fmt::format(
                                format_args!("--tw-grayscale: grayscale({0})", self.0),
                            );
                            res
                        },
                        FILTER_STYLE.into(),
                    ]),
                ),
            )
        }
    }
    pub struct HueRotate(pub String);
    #[automatically_derived]
    impl ::core::fmt::Debug for HueRotate {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(f, "HueRotate", &&self.0)
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for HueRotate {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for HueRotate {
        #[inline]
        fn eq(&self, other: &HueRotate) -> bool {
            self.0 == other.0
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for HueRotate {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.0, state)
        }
    }
    fn hue_rotate<'a>(
        input: &'a str,
        config: &'a Config,
    ) -> IResult<&'a str, HueRotate> {
        map(
            neg_keyword_value("hue-rotate", config.filters.get_hue_rotate()),
            HueRotate,
        )(input)
    }
    impl IntoDeclaration for HueRotate {
        fn to_decl(self) -> Decl {
            Decl::Vec(
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        {
                            let res = ::alloc::fmt::format(
                                format_args!("--tw-hue-rotate: hue-rotate({0})", self.0),
                            );
                            res
                        },
                        FILTER_STYLE.into(),
                    ]),
                ),
            )
        }
    }
    #[name(invert)]
    #[config(filters.get_invert)]
    pub struct Invert<'a>(pub &'a str);
    #[automatically_derived]
    impl<'a> ::core::fmt::Debug for Invert<'a> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Invert", &&self.0)
        }
    }
    #[automatically_derived]
    impl<'a> ::core::marker::StructuralPartialEq for Invert<'a> {}
    #[automatically_derived]
    impl<'a> ::core::cmp::PartialEq for Invert<'a> {
        #[inline]
        fn eq(&self, other: &Invert<'a>) -> bool {
            self.0 == other.0
        }
    }
    #[automatically_derived]
    impl<'a> ::core::hash::Hash for Invert<'a> {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.0, state)
        }
    }
    fn invert<'a>(
        input: &'a str,
        config: &'a crate::config::Config,
    ) -> nom::IResult<&'a str, Invert<'a>> {
        nom::combinator::map(
            nom::branch::alt((
                nom::sequence::delimited(
                    nom::bytes::complete::tag("["),
                    nom::bytes::complete::is_not("]"),
                    nom::bytes::complete::tag("]"),
                ),
                nom::combinator::map_opt(
                    nom::bytes::complete::is_not(" "),
                    |v| config.filters.get_invert().get(v).copied(),
                ),
            )),
            Invert,
        )(input)
    }
    impl<'a> IntoDeclaration for Invert<'a> {
        fn to_decl(self) -> Decl {
            Decl::Vec(
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        {
                            let res = ::alloc::fmt::format(
                                format_args!("--tw-invert: invert({0})", self.0),
                            );
                            res
                        },
                        FILTER_STYLE.into(),
                    ]),
                ),
            )
        }
    }
    #[name(saturate)]
    #[config(filters.get_saturate)]
    pub struct Saturate<'a>(pub &'a str);
    #[automatically_derived]
    impl<'a> ::core::fmt::Debug for Saturate<'a> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Saturate", &&self.0)
        }
    }
    #[automatically_derived]
    impl<'a> ::core::marker::StructuralPartialEq for Saturate<'a> {}
    #[automatically_derived]
    impl<'a> ::core::cmp::PartialEq for Saturate<'a> {
        #[inline]
        fn eq(&self, other: &Saturate<'a>) -> bool {
            self.0 == other.0
        }
    }
    #[automatically_derived]
    impl<'a> ::core::hash::Hash for Saturate<'a> {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.0, state)
        }
    }
    fn saturate<'a>(
        input: &'a str,
        config: &'a crate::config::Config,
    ) -> nom::IResult<&'a str, Saturate<'a>> {
        nom::combinator::map(
            nom::branch::alt((
                nom::sequence::delimited(
                    nom::bytes::complete::tag("["),
                    nom::bytes::complete::is_not("]"),
                    nom::bytes::complete::tag("]"),
                ),
                nom::combinator::map_opt(
                    nom::bytes::complete::is_not(" "),
                    |v| config.filters.get_saturate().get(v).copied(),
                ),
            )),
            Saturate,
        )(input)
    }
    impl<'a> IntoDeclaration for Saturate<'a> {
        fn to_decl(self) -> Decl {
            Decl::Vec(
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        {
                            let res = ::alloc::fmt::format(
                                format_args!("--tw-saturate: saturate({0})", self.0),
                            );
                            res
                        },
                        FILTER_STYLE.into(),
                    ]),
                ),
            )
        }
    }
    #[name(sepia)]
    #[config(filters.get_sepia)]
    pub struct Sepia<'a>(pub &'a str);
    #[automatically_derived]
    impl<'a> ::core::fmt::Debug for Sepia<'a> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Sepia", &&self.0)
        }
    }
    #[automatically_derived]
    impl<'a> ::core::marker::StructuralPartialEq for Sepia<'a> {}
    #[automatically_derived]
    impl<'a> ::core::cmp::PartialEq for Sepia<'a> {
        #[inline]
        fn eq(&self, other: &Sepia<'a>) -> bool {
            self.0 == other.0
        }
    }
    #[automatically_derived]
    impl<'a> ::core::hash::Hash for Sepia<'a> {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.0, state)
        }
    }
    fn sepia<'a>(
        input: &'a str,
        config: &'a crate::config::Config,
    ) -> nom::IResult<&'a str, Sepia<'a>> {
        nom::combinator::map(
            nom::branch::alt((
                nom::sequence::delimited(
                    nom::bytes::complete::tag("["),
                    nom::bytes::complete::is_not("]"),
                    nom::bytes::complete::tag("]"),
                ),
                nom::combinator::map_opt(
                    nom::bytes::complete::is_not(" "),
                    |v| config.filters.get_sepia().get(v).copied(),
                ),
            )),
            Sepia,
        )(input)
    }
    impl<'a> IntoDeclaration for Sepia<'a> {
        fn to_decl(self) -> Decl {
            Decl::Vec(
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        {
                            let res = ::alloc::fmt::format(
                                format_args!("--tw-sepia: sepia({0})", self.0),
                            );
                            res
                        },
                        FILTER_STYLE.into(),
                    ]),
                ),
            )
        }
    }
    #[name(backdrop_blur)]
    #[config(filters.get_blur)]
    pub struct BackdropBlur<'a>(pub &'a str);
    #[automatically_derived]
    impl<'a> ::core::fmt::Debug for BackdropBlur<'a> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(
                f,
                "BackdropBlur",
                &&self.0,
            )
        }
    }
    #[automatically_derived]
    impl<'a> ::core::marker::StructuralPartialEq for BackdropBlur<'a> {}
    #[automatically_derived]
    impl<'a> ::core::cmp::PartialEq for BackdropBlur<'a> {
        #[inline]
        fn eq(&self, other: &BackdropBlur<'a>) -> bool {
            self.0 == other.0
        }
    }
    #[automatically_derived]
    impl<'a> ::core::hash::Hash for BackdropBlur<'a> {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.0, state)
        }
    }
    fn backdrop_blur<'a>(
        input: &'a str,
        config: &'a crate::config::Config,
    ) -> nom::IResult<&'a str, BackdropBlur<'a>> {
        nom::combinator::map(
            nom::branch::alt((
                nom::sequence::delimited(
                    nom::bytes::complete::tag("["),
                    nom::bytes::complete::is_not("]"),
                    nom::bytes::complete::tag("]"),
                ),
                nom::combinator::map_opt(
                    nom::bytes::complete::is_not(" "),
                    |v| config.filters.get_blur().get(v).copied(),
                ),
            )),
            BackdropBlur,
        )(input)
    }
    impl<'a> IntoDeclaration for BackdropBlur<'a> {
        fn to_decl(self) -> Decl {
            Decl::Vec(
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        {
                            let res = ::alloc::fmt::format(
                                format_args!("--tw-backdrop-blur: blur({0})", self.0),
                            );
                            res
                        },
                        WEBKIT_BACKDROP_FILTER_STYLE.into(),
                        BACKDROP_FILTER_STYLE.into(),
                    ]),
                ),
            )
        }
    }
    #[name(backdrop_brightness)]
    #[config(filters.get_brightness)]
    pub struct BackdropBrightness<'a>(pub &'a str);
    #[automatically_derived]
    impl<'a> ::core::fmt::Debug for BackdropBrightness<'a> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(
                f,
                "BackdropBrightness",
                &&self.0,
            )
        }
    }
    #[automatically_derived]
    impl<'a> ::core::marker::StructuralPartialEq for BackdropBrightness<'a> {}
    #[automatically_derived]
    impl<'a> ::core::cmp::PartialEq for BackdropBrightness<'a> {
        #[inline]
        fn eq(&self, other: &BackdropBrightness<'a>) -> bool {
            self.0 == other.0
        }
    }
    #[automatically_derived]
    impl<'a> ::core::hash::Hash for BackdropBrightness<'a> {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.0, state)
        }
    }
    fn backdrop_brightness<'a>(
        input: &'a str,
        config: &'a crate::config::Config,
    ) -> nom::IResult<&'a str, BackdropBrightness<'a>> {
        nom::combinator::map(
            nom::branch::alt((
                nom::sequence::delimited(
                    nom::bytes::complete::tag("["),
                    nom::bytes::complete::is_not("]"),
                    nom::bytes::complete::tag("]"),
                ),
                nom::combinator::map_opt(
                    nom::bytes::complete::is_not(" "),
                    |v| config.filters.get_brightness().get(v).copied(),
                ),
            )),
            BackdropBrightness,
        )(input)
    }
    impl<'a> IntoDeclaration for BackdropBrightness<'a> {
        fn to_decl(self) -> Decl {
            Decl::Vec(
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        {
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "--tw-backdrop-brightness: brightness({0})", self.0
                                ),
                            );
                            res
                        },
                        WEBKIT_BACKDROP_FILTER_STYLE.into(),
                        BACKDROP_FILTER_STYLE.into(),
                    ]),
                ),
            )
        }
    }
    #[name(backdrop_contrast)]
    #[config(filters.get_contrast)]
    pub struct BackdropContrast<'a>(pub &'a str);
    #[automatically_derived]
    impl<'a> ::core::fmt::Debug for BackdropContrast<'a> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(
                f,
                "BackdropContrast",
                &&self.0,
            )
        }
    }
    #[automatically_derived]
    impl<'a> ::core::marker::StructuralPartialEq for BackdropContrast<'a> {}
    #[automatically_derived]
    impl<'a> ::core::cmp::PartialEq for BackdropContrast<'a> {
        #[inline]
        fn eq(&self, other: &BackdropContrast<'a>) -> bool {
            self.0 == other.0
        }
    }
    #[automatically_derived]
    impl<'a> ::core::hash::Hash for BackdropContrast<'a> {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.0, state)
        }
    }
    fn backdrop_contrast<'a>(
        input: &'a str,
        config: &'a crate::config::Config,
    ) -> nom::IResult<&'a str, BackdropContrast<'a>> {
        nom::combinator::map(
            nom::branch::alt((
                nom::sequence::delimited(
                    nom::bytes::complete::tag("["),
                    nom::bytes::complete::is_not("]"),
                    nom::bytes::complete::tag("]"),
                ),
                nom::combinator::map_opt(
                    nom::bytes::complete::is_not(" "),
                    |v| config.filters.get_contrast().get(v).copied(),
                ),
            )),
            BackdropContrast,
        )(input)
    }
    impl<'a> IntoDeclaration for BackdropContrast<'a> {
        fn to_decl(self) -> Decl {
            Decl::Vec(
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        {
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "--tw-backdrop-contrast: contrast({0})", self.0
                                ),
                            );
                            res
                        },
                        WEBKIT_BACKDROP_FILTER_STYLE.into(),
                        BACKDROP_FILTER_STYLE.into(),
                    ]),
                ),
            )
        }
    }
    #[name(backdrop_grayscale)]
    #[config(filters.get_grayscale)]
    pub struct BackdropGrayscale<'a>(pub &'a str);
    #[automatically_derived]
    impl<'a> ::core::fmt::Debug for BackdropGrayscale<'a> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(
                f,
                "BackdropGrayscale",
                &&self.0,
            )
        }
    }
    #[automatically_derived]
    impl<'a> ::core::marker::StructuralPartialEq for BackdropGrayscale<'a> {}
    #[automatically_derived]
    impl<'a> ::core::cmp::PartialEq for BackdropGrayscale<'a> {
        #[inline]
        fn eq(&self, other: &BackdropGrayscale<'a>) -> bool {
            self.0 == other.0
        }
    }
    #[automatically_derived]
    impl<'a> ::core::hash::Hash for BackdropGrayscale<'a> {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.0, state)
        }
    }
    fn backdrop_grayscale<'a>(
        input: &'a str,
        config: &'a crate::config::Config,
    ) -> nom::IResult<&'a str, BackdropGrayscale<'a>> {
        nom::combinator::map(
            nom::branch::alt((
                nom::sequence::delimited(
                    nom::bytes::complete::tag("["),
                    nom::bytes::complete::is_not("]"),
                    nom::bytes::complete::tag("]"),
                ),
                nom::combinator::map_opt(
                    nom::bytes::complete::is_not(" "),
                    |v| config.filters.get_grayscale().get(v).copied(),
                ),
            )),
            BackdropGrayscale,
        )(input)
    }
    impl<'a> IntoDeclaration for BackdropGrayscale<'a> {
        fn to_decl(self) -> Decl {
            Decl::Vec(
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        {
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "--tw-backdrop-grayscale: grayscale({0})", self.0
                                ),
                            );
                            res
                        },
                        WEBKIT_BACKDROP_FILTER_STYLE.into(),
                        BACKDROP_FILTER_STYLE.into(),
                    ]),
                ),
            )
        }
    }
    pub struct BackdropHueRotate(pub String);
    #[automatically_derived]
    impl ::core::fmt::Debug for BackdropHueRotate {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(
                f,
                "BackdropHueRotate",
                &&self.0,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for BackdropHueRotate {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for BackdropHueRotate {
        #[inline]
        fn eq(&self, other: &BackdropHueRotate) -> bool {
            self.0 == other.0
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for BackdropHueRotate {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.0, state)
        }
    }
    fn backdrop_hue_rotate<'a>(
        input: &'a str,
        config: &'a Config,
    ) -> IResult<&'a str, BackdropHueRotate> {
        map(
            neg_keyword_value("backdrop_hue-rotate", config.filters.get_hue_rotate()),
            BackdropHueRotate,
        )(input)
    }
    impl<'a> IntoDeclaration for BackdropHueRotate {
        fn to_decl(self) -> Decl {
            Decl::Vec(
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        {
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "--tw-backdrop-hue-rotate: hue-rotate({0})", self.0
                                ),
                            );
                            res
                        },
                        WEBKIT_BACKDROP_FILTER_STYLE.into(),
                        BACKDROP_FILTER_STYLE.into(),
                    ]),
                ),
            )
        }
    }
    #[name(backdrop_invert)]
    #[config(filters.get_invert)]
    pub struct BackdropInvert<'a>(pub &'a str);
    #[automatically_derived]
    impl<'a> ::core::fmt::Debug for BackdropInvert<'a> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(
                f,
                "BackdropInvert",
                &&self.0,
            )
        }
    }
    #[automatically_derived]
    impl<'a> ::core::marker::StructuralPartialEq for BackdropInvert<'a> {}
    #[automatically_derived]
    impl<'a> ::core::cmp::PartialEq for BackdropInvert<'a> {
        #[inline]
        fn eq(&self, other: &BackdropInvert<'a>) -> bool {
            self.0 == other.0
        }
    }
    #[automatically_derived]
    impl<'a> ::core::hash::Hash for BackdropInvert<'a> {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.0, state)
        }
    }
    fn backdrop_invert<'a>(
        input: &'a str,
        config: &'a crate::config::Config,
    ) -> nom::IResult<&'a str, BackdropInvert<'a>> {
        nom::combinator::map(
            nom::branch::alt((
                nom::sequence::delimited(
                    nom::bytes::complete::tag("["),
                    nom::bytes::complete::is_not("]"),
                    nom::bytes::complete::tag("]"),
                ),
                nom::combinator::map_opt(
                    nom::bytes::complete::is_not(" "),
                    |v| config.filters.get_invert().get(v).copied(),
                ),
            )),
            BackdropInvert,
        )(input)
    }
    impl<'a> IntoDeclaration for BackdropInvert<'a> {
        fn to_decl(self) -> Decl {
            Decl::Vec(
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        {
                            let res = ::alloc::fmt::format(
                                format_args!("--tw-backdrop-invert: invert({0})", self.0),
                            );
                            res
                        },
                        WEBKIT_BACKDROP_FILTER_STYLE.into(),
                        BACKDROP_FILTER_STYLE.into(),
                    ]),
                ),
            )
        }
    }
    #[name(backdrop_opacity)]
    #[config(filters.get_opacity)]
    pub struct BackdropOpacity<'a>(pub &'a str);
    #[automatically_derived]
    impl<'a> ::core::fmt::Debug for BackdropOpacity<'a> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(
                f,
                "BackdropOpacity",
                &&self.0,
            )
        }
    }
    #[automatically_derived]
    impl<'a> ::core::marker::StructuralPartialEq for BackdropOpacity<'a> {}
    #[automatically_derived]
    impl<'a> ::core::cmp::PartialEq for BackdropOpacity<'a> {
        #[inline]
        fn eq(&self, other: &BackdropOpacity<'a>) -> bool {
            self.0 == other.0
        }
    }
    #[automatically_derived]
    impl<'a> ::core::hash::Hash for BackdropOpacity<'a> {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.0, state)
        }
    }
    fn backdrop_opacity<'a>(
        input: &'a str,
        config: &'a crate::config::Config,
    ) -> nom::IResult<&'a str, BackdropOpacity<'a>> {
        nom::combinator::map(
            nom::branch::alt((
                nom::sequence::delimited(
                    nom::bytes::complete::tag("["),
                    nom::bytes::complete::is_not("]"),
                    nom::bytes::complete::tag("]"),
                ),
                nom::combinator::map_opt(
                    nom::bytes::complete::is_not(" "),
                    |v| config.filters.get_opacity().get(v).copied(),
                ),
            )),
            BackdropOpacity,
        )(input)
    }
    impl<'a> IntoDeclaration for BackdropOpacity<'a> {
        fn to_decl(self) -> Decl {
            Decl::Vec(
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        {
                            let res = ::alloc::fmt::format(
                                format_args!("--tw-backdrop-opacity: opacity({0})", self.0),
                            );
                            res
                        },
                        WEBKIT_BACKDROP_FILTER_STYLE.into(),
                        BACKDROP_FILTER_STYLE.into(),
                    ]),
                ),
            )
        }
    }
    #[name(backdrop_saturate)]
    #[config(filters.get_saturate)]
    pub struct BackdropSaturate<'a>(pub &'a str);
    #[automatically_derived]
    impl<'a> ::core::fmt::Debug for BackdropSaturate<'a> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(
                f,
                "BackdropSaturate",
                &&self.0,
            )
        }
    }
    #[automatically_derived]
    impl<'a> ::core::marker::StructuralPartialEq for BackdropSaturate<'a> {}
    #[automatically_derived]
    impl<'a> ::core::cmp::PartialEq for BackdropSaturate<'a> {
        #[inline]
        fn eq(&self, other: &BackdropSaturate<'a>) -> bool {
            self.0 == other.0
        }
    }
    #[automatically_derived]
    impl<'a> ::core::hash::Hash for BackdropSaturate<'a> {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.0, state)
        }
    }
    fn backdrop_saturate<'a>(
        input: &'a str,
        config: &'a crate::config::Config,
    ) -> nom::IResult<&'a str, BackdropSaturate<'a>> {
        nom::combinator::map(
            nom::branch::alt((
                nom::sequence::delimited(
                    nom::bytes::complete::tag("["),
                    nom::bytes::complete::is_not("]"),
                    nom::bytes::complete::tag("]"),
                ),
                nom::combinator::map_opt(
                    nom::bytes::complete::is_not(" "),
                    |v| config.filters.get_saturate().get(v).copied(),
                ),
            )),
            BackdropSaturate,
        )(input)
    }
    impl<'a> IntoDeclaration for BackdropSaturate<'a> {
        fn to_decl(self) -> Decl {
            Decl::Vec(
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        {
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "--tw-backdrop-saturate: saturate({0})", self.0
                                ),
                            );
                            res
                        },
                        WEBKIT_BACKDROP_FILTER_STYLE.into(),
                        BACKDROP_FILTER_STYLE.into(),
                    ]),
                ),
            )
        }
    }
    #[name(backdrop_sepia)]
    #[config(filters.get_sepia)]
    pub struct BackdropSepia<'a>(pub &'a str);
    #[automatically_derived]
    impl<'a> ::core::fmt::Debug for BackdropSepia<'a> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(
                f,
                "BackdropSepia",
                &&self.0,
            )
        }
    }
    #[automatically_derived]
    impl<'a> ::core::marker::StructuralPartialEq for BackdropSepia<'a> {}
    #[automatically_derived]
    impl<'a> ::core::cmp::PartialEq for BackdropSepia<'a> {
        #[inline]
        fn eq(&self, other: &BackdropSepia<'a>) -> bool {
            self.0 == other.0
        }
    }
    #[automatically_derived]
    impl<'a> ::core::hash::Hash for BackdropSepia<'a> {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.0, state)
        }
    }
    fn backdrop_sepia<'a>(
        input: &'a str,
        config: &'a crate::config::Config,
    ) -> nom::IResult<&'a str, BackdropSepia<'a>> {
        nom::combinator::map(
            nom::branch::alt((
                nom::sequence::delimited(
                    nom::bytes::complete::tag("["),
                    nom::bytes::complete::is_not("]"),
                    nom::bytes::complete::tag("]"),
                ),
                nom::combinator::map_opt(
                    nom::bytes::complete::is_not(" "),
                    |v| config.filters.get_sepia().get(v).copied(),
                ),
            )),
            BackdropSepia,
        )(input)
    }
    impl<'a> IntoDeclaration for BackdropSepia<'a> {
        fn to_decl(self) -> Decl {
            Decl::Vec(
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        {
                            let res = ::alloc::fmt::format(
                                format_args!("--tw-backdrop-sepia: sepia({0})", self.0),
                            );
                            res
                        },
                        WEBKIT_BACKDROP_FILTER_STYLE.into(),
                        BACKDROP_FILTER_STYLE.into(),
                    ]),
                ),
            )
        }
    }
}
