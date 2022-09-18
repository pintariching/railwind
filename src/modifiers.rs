#[derive(Debug)]
pub enum PseudoClass {
    Hover,
    Focus,
    FocusWithin,
    FocusVisible,
    Active,
    Visited,
    Target,
    First,
    Last,
    Only,
    Odd,
    Even,
    FirstOfType,
    LastOfType,
    OnlyOfType,
    Empty,
    Disabled,
    Enabled,
    Checked,
    Indeterminate,
    Default,
    Required,
    Valid,
    Invalid,
    InRange,
    OutOfRange,
    PlaceholderShown,
    Autofill,
    ReadOnly,
    Open,
}

impl PseudoClass {
    fn as_str(&self) -> &'static str {
        match self {
            PseudoClass::Hover => "hover",
            PseudoClass::Focus => "focus",
            PseudoClass::FocusWithin => "focus-within",
            PseudoClass::FocusVisible => "focus-visible",
            PseudoClass::Active => "active",
            PseudoClass::Visited => "visited",
            PseudoClass::Target => "target",
            PseudoClass::First => "first-child",
            PseudoClass::Last => "last-child",
            PseudoClass::Only => "only-child",
            PseudoClass::Odd => "nth-child(odd)",
            PseudoClass::Even => "nth-child(even)",
            PseudoClass::FirstOfType => "first-of-type",
            PseudoClass::LastOfType => "last-of-type",
            PseudoClass::OnlyOfType => "only-of-type",
            PseudoClass::Empty => "empty",
            PseudoClass::Disabled => "disabled",
            PseudoClass::Enabled => "enabled",
            PseudoClass::Checked => "checked",
            PseudoClass::Indeterminate => "indeterminate",
            PseudoClass::Default => "default",
            PseudoClass::Required => "required",
            PseudoClass::Valid => "valid",
            PseudoClass::Invalid => "invalid",
            PseudoClass::InRange => "in-range",
            PseudoClass::OutOfRange => "out-of-range",
            PseudoClass::PlaceholderShown => "placeholder-shown",
            PseudoClass::Autofill => "autofill",
            PseudoClass::ReadOnly => "readonly",
            PseudoClass::Open => "open",
        }
    }

    fn parse_from_str(str: &str) -> Option<PseudoClass> {
        match str {
            "hover" => Some(PseudoClass::Hover),
            "focus" => Some(PseudoClass::Focus),
            "focus-within" => Some(PseudoClass::FocusWithin),
            "focus-visible" => Some(PseudoClass::FocusVisible),
            "active" => Some(PseudoClass::Active),
            "visited" => Some(PseudoClass::Visited),
            "target" => Some(PseudoClass::Target),
            "first-child" => Some(PseudoClass::First),
            "last-child" => Some(PseudoClass::Last),
            "only-child" => Some(PseudoClass::Only),
            "nth-child(odd)" => Some(PseudoClass::Odd),
            "nth-child(even)" => Some(PseudoClass::Even),
            "first-of-type" => Some(PseudoClass::FirstOfType),
            "last-of-type" => Some(PseudoClass::LastOfType),
            "only-of-type" => Some(PseudoClass::OnlyOfType),
            "empty" => Some(PseudoClass::Empty),
            "disabled" => Some(PseudoClass::Disabled),
            "enabled" => Some(PseudoClass::Enabled),
            "checked" => Some(PseudoClass::Checked),
            "indeterminate" => Some(PseudoClass::Indeterminate),
            "default" => Some(PseudoClass::Default),
            "required" => Some(PseudoClass::Required),
            "valid" => Some(PseudoClass::Valid),
            "invalid" => Some(PseudoClass::Invalid),
            "in-range" => Some(PseudoClass::InRange),
            "out-of-range" => Some(PseudoClass::OutOfRange),
            "placeholder-shown" => Some(PseudoClass::PlaceholderShown),
            "autofill" => Some(PseudoClass::Autofill),
            "readonly" => Some(PseudoClass::ReadOnly),
            "open" => Some(PseudoClass::Open),
            _ => None,
        }
    }
}
#[derive(Debug)]
pub struct Group(PseudoClass);

impl Group {
    fn to_string(&self) -> String {
        format!("group-{}", self.0.as_str())
    }
}

#[derive(Debug)]
pub struct Peer(PseudoClass);

impl Peer {
    fn to_string(&self) -> String {
        format!("peer-{}", self.0.as_str())
    }
}

#[derive(Debug)]
pub enum PseudoElement {
    Before,
    After,
    Placeholder,
    File,
    Marker,
    Selection,
    FirstLine,
    FirstLetter,
    LastLine,
    Backdrop,
}

impl PseudoElement {
    fn as_str(&self) -> &'static str {
        match self {
            PseudoElement::Before => "before",
            PseudoElement::After => "after",
            PseudoElement::Placeholder => "placeholder",
            PseudoElement::File => "file",
            PseudoElement::Marker => "marker",
            PseudoElement::Selection => "selection",
            PseudoElement::FirstLine => "first-line",
            PseudoElement::FirstLetter => "first-letter",
            PseudoElement::LastLine => "last-line",
            PseudoElement::Backdrop => "backdrop",
        }
    }

    fn parse_from_str(str: &str) -> Option<PseudoElement> {
        match str {
            "before" => Some(PseudoElement::Before),
            "after" => Some(PseudoElement::After),
            "placeholder" => Some(PseudoElement::Placeholder),
            "file" => Some(PseudoElement::File),
            "marker" => Some(PseudoElement::Marker),
            "selection" => Some(PseudoElement::Selection),
            "first-line" => Some(PseudoElement::FirstLine),
            "first-letter" => Some(PseudoElement::FirstLetter),
            "last-line" => Some(PseudoElement::LastLine),
            "backdrop" => Some(PseudoElement::Backdrop),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum MediaQuery {
    Sm,
    Md,
    Lg,
    Xl,
    Xxl,
    Dark,
    MotionReduce,
    MotionSafe,
    ContrastMore,
    ContrastLess,
    Portrait,
    Landscape,
    Print,
    Ltr,
    Rtl,
}

impl MediaQuery {
    fn as_str(&self) -> &'static str {
        match self {
            MediaQuery::Sm => "sm",
            MediaQuery::Md => "md",
            MediaQuery::Lg => "lg",
            MediaQuery::Xl => "xl",
            MediaQuery::Xxl => "2xl",
            MediaQuery::Dark => "dark",
            MediaQuery::MotionReduce => "motion-reduce",
            MediaQuery::MotionSafe => "motion-safe",
            MediaQuery::ContrastMore => "contrast-more",
            MediaQuery::ContrastLess => "contrast-less",
            MediaQuery::Portrait => "portrait",
            MediaQuery::Landscape => "landscape",
            MediaQuery::Print => "print",
            MediaQuery::Ltr => "ltr",
            MediaQuery::Rtl => "rtl",
        }
    }

    fn parse_from_str(str: &str) -> Option<MediaQuery> {
        match str {
            "sm" => Some(MediaQuery::Sm),
            "md" => Some(MediaQuery::Md),
            "lg" => Some(MediaQuery::Lg),
            "xl" => Some(MediaQuery::Xl),
            "2xl" => Some(MediaQuery::Xxl),
            "dark" => Some(MediaQuery::Dark),
            "motion-reduce" => Some(MediaQuery::MotionReduce),
            "motion-safe" => Some(MediaQuery::MotionSafe),
            "contrast-more" => Some(MediaQuery::ContrastMore),
            "contrast-less" => Some(MediaQuery::ContrastLess),
            "portrait" => Some(MediaQuery::Portrait),
            "landscape" => Some(MediaQuery::Landscape),
            "print" => Some(MediaQuery::Print),
            "ltr" => Some(MediaQuery::Ltr),
            "rtl" => Some(MediaQuery::Rtl),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum Modifier {
    PseudoClass(PseudoClass),
    PseudoElement(PseudoElement),
    MediaQuery(MediaQuery),
}

impl Modifier {
    pub fn parse_from_str(str: &str) -> Option<Modifier> {
        if let Some(modifier) = PseudoClass::parse_from_str(str) {
            return Some(Modifier::PseudoClass(modifier));
        }

        if let Some(modifier) = PseudoElement::parse_from_str(str) {
            return Some(Modifier::PseudoElement(modifier));
        }

        if let Some(modifier) = MediaQuery::parse_from_str(str) {
            return Some(Modifier::MediaQuery(modifier));
        }

        None
    }
}
