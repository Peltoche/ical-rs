use crate::ical_property;
use parser::vcard::component::VcardContact;
use property::Property;

pub struct IcalVcardBuilder {
    vcard: VcardContact,
}
pub struct Name(IcalVcardBuilder);
pub struct FormattedName {
    builder: IcalVcardBuilder,
    names: Vec<String>,
}
pub struct Finalizer(IcalVcardBuilder);

/// Builds a new [RFC 2426 VCard-Entry](https://tools.ietf.org/html/rfc2426#section-2.4.2).
///
/// ```
/// # use ical::generator::*;
/// # use ical::ical_property;
/// #
/// let vcard = IcalVcardBuilder::version("4.0")
///     .names(
///         Some("Marx"),
///         Some("Adolph"),
///         Some("Arthur"),
///         Some("Mr."),
///         None,
///     )
///     .generate_fn()
///     .set(ical_property!("NICKNAME", "Harpo Marx"))
///     .build();
/// ```
impl IcalVcardBuilder {
    pub fn version<S: Into<String>>(version: S) -> Name {
        let mut me = Self {
            vcard: VcardContact::new(),
        };
        me.vcard.properties.push(ical_property!("VERSION", version));
        Name(me)
    }
}

#[inline]
fn clean<S: Into<String>>(t: Option<S>) -> String {
    match t {
        None => String::new(),
        Some(s) => {
            let s = s.into();
            s.trim().into()
        }
    }
}

impl Name {
    //! To specify the components of the name of the object the vCard represents.
    //!
    //! Format:
    //!
    //!   `familiy-name;given-name;additional-name;honor-prefix;honor-suffix`
    //!
    //! Type example:
    //!
    //!   `Public;John;Quinlan;Mr.;Esq.`
    //!
    //!   `Stevenson;John;Philip,Paul;Dr.;Jr.,M.D.,A.C.P.`
    //!
    pub fn name(mut self, n: impl Into<String>) -> FormattedName {
        let na = n.into();
        self.0
            .vcard
            .properties
            .push(ical_property!("N", na.clone()));

        FormattedName {
            builder: self.0,
            names: na.split(';').map(String::from).collect(),
        }
    }

    /// To specify the components of the name of the object the vCard represents.
    ///
    /// Format:
    ///
    ///    `familiy-name, given-name, additional-name, honor-prefix, honor-suffix`
    ///
    pub fn names<S: Into<String>>(
        mut self,
        family_name: Option<S>,
        given_name: Option<S>,
        additional_name: Option<S>,
        honorific_prefixes: Option<S>,
        honorific_suffixes: Option<S>,
    ) -> FormattedName {
        let names: Vec<String> = vec![
            clean(family_name),
            clean(given_name),
            clean(additional_name),
            clean(honorific_prefixes),
            clean(honorific_suffixes),
        ];

        self.0
            .vcard
            .properties
            .push(ical_property!("N", names.join(";")));
        FormattedName {
            builder: self.0,
            names,
        }
    }
}

impl FormattedName {
    /// To specify the formatted text corresponding to the name
    /// of the object the vCard represents.
    ///
    /// Type example
    ///
    ///   `Mr. John Q. Public, Esq.`
    pub fn formatted_name(mut self, f_n: impl Into<String>) -> Finalizer {
        self.builder
            .vcard
            .properties
            .push(ical_property!("FN", f_n.into()));
        Finalizer(self.builder)
    }

    /// Generates the formatted text based on the parts defined by `Name`.
    pub fn generate_fn(self) -> Finalizer {
        #[inline]
        fn add_sep(os: Option<&String>, sep: &str) -> String {
            match os {
                None => String::new(),
                Some(s) => {
                    if s.is_empty() {
                        String::new()
                    } else {
                        s.clone() + sep
                    }
                }
            }
        }
        let formatted_name = add_sep(self.names.get(3), " ")
            + &add_sep(self.names.get(1), " ")
            + &add_sep(self.names.get(2), " ")
            + &add_sep(self.names.get(0), " ")
            + &add_sep(self.names.get(4), " ");

        Self::formatted_name(self, formatted_name.trim())
    }
}

impl Finalizer {
    /// creates a valid `VcardContact`.
    pub fn build(self) -> VcardContact {
        self.0.vcard
    }

    /// adds optional properties to the `VcardContact`.
    pub fn set(mut self, property: Property) -> Self {
        self.0.vcard.properties.push(property);
        self
    }
}

#[allow(unused)]
mod should {
    use crate::*;
    use generator::vcard_builder::IcalVcardBuilder;
    use property::Property;

    // Example from https://en.wikipedia.org/wiki/VCard
    #[test]
    fn build_vcards_wikipedia_example() {
        use generator::Emitter;
        let expect = "BEGIN:VCARD\r\n\
        VERSION:4.0\r\n\
        N:Gump;Forrest;;Mr.;\r\n\
        FN:Forrest Gump\r\n\
        ORG:Bubba Gump Shrimp Co.\r\n\
        TITLE:Shrimp Man\r\n\
        PHOTO;MEDIATYPE=image/gif:http://www.example.com/dir_photos/my_photo.gif\r\n\
        TEL;TYPE=work,voice;VALUE=uri:tel:+1-111-555-1212\r\n\
        TEL;TYPE=home,voice;VALUE=uri:tel:+1-404-555-1212\r\n\
        ADR;TYPE=WORK;PREF=1;LABEL=\"100 Waters Edge\\nBaytown\\, LA 30314\\nUnited Sta\r\n \
         tes of America\":;;100 Waters Edge;Baytown;LA;30314;United States of Americ\r\n \
         a\r\n\
        ADR;TYPE=HOME;LABEL=\"42 Plantation St.\\nBaytown\\, LA 30314\\nUnited States o\r\n f \
         America\":;;42 Plantation St.;Baytown;LA;30314;United States of America\r\n\
        EMAIL:forrestgump@example.com\r\n\
        REV:20080424T195243Z\r\n\
        x-qq:21588891\r\n\
        END:VCARD\r\n\
        ";

        let vcard = IcalVcardBuilder::version("4.0")
            .names(Some("Gump"), Some("Forrest"), None, Some("Mr. "), None)
            .formatted_name("Forrest Gump")
            .set(ical_property!("ORG", "Bubba Gump Shrimp Co."))
            .set(ical_property!("TITLE", "Shrimp Man"))
            .set(ical_property!(
                "PHOTO",
                "http://www.example.com/dir_photos/my_photo.gif",
                ical_param!("MEDIATYPE", "image/gif")
            ))
            .set(ical_property!(
                "TEL",
                "tel:+1-111-555-1212",
                ical_param!("TYPE", "work", "voice"),
                ical_param!("VALUE", "uri")
            ))
            .set(ical_property!(
                "TEL",
                "tel:+1-404-555-1212",
                ical_param!("TYPE", "home", "voice"),
                ical_param!("VALUE", "uri")
            ))
            .set(ical_property!(
                "ADR",
                ";;100 Waters Edge;Baytown;LA;30314;United States of America",
                ical_param!("TYPE", "WORK"),
                ical_param!("PREF", "1"),
                ical_param!(
                    "LABEL",
                    "\"100 Waters Edge\nBaytown\\, LA 30314\nUnited States of America\""
                )
            ))
            .set(ical_property!(
                "ADR",
                ";;42 Plantation St.;Baytown;LA;30314;United States of America",
                ical_param!("TYPE", "HOME"),
                ical_param!(
                    "LABEL",
                    "\"42 Plantation St.\nBaytown\\, LA 30314\nUnited States of America\""
                )
            ))
            .set(ical_property!("EMAIL", "forrestgump@example.com"))
            .set(ical_property!("REV", "20080424T195243Z"))
            .set(ical_property!("x-qq", "21588891"))
            .build();

        assert_eq!(vcard.generate(), expect);
    }

    #[test]
    fn build_vcard_with_fn_generated() {
        use generator::Emitter;
        let expect = "BEGIN:VCARD\r\n\
        VERSION:4.0\r\n\
        N:Marx;Adolph;Arthur;Mr.;\r\n\
        FN:Mr. Adolph Arthur Marx\r\n\
        NICKNAME:Harpo Marx\r\n\
        END:VCARD\r\n\
        ";
        let vcard = IcalVcardBuilder::version("4.0")
            .names(
                Some("Marx"),
                Some("Adolph"),
                Some("Arthur"),
                Some("Mr."),
                None,
            )
            .generate_fn()
            .set(ical_property!("NICKNAME", "Harpo Marx"))
            .build();
        assert_eq!(vcard.generate(), expect);
    }
}
