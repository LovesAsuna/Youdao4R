pub struct LanguageType(pub &'static str);

impl LanguageType {
    pub const AUTO: LanguageType = LanguageType("AUTO");
    pub const CHINESE: LanguageType = LanguageType("zh-CNS");
    pub const ENGLISH: LanguageType = LanguageType("en");
    pub const KOREAN: LanguageType = LanguageType("ko");
    pub const JAPANESE: LanguageType = LanguageType("ja");
    pub const FRENCH: LanguageType = LanguageType("fr");
    pub const RUSSIA: LanguageType = LanguageType("ru");
    pub const SPANISH: LanguageType = LanguageType("es");
    pub const PORTUGUESE: LanguageType = LanguageType("pt");
    pub const HINDI: LanguageType = LanguageType("hi");
    pub const ARAB: LanguageType = LanguageType("ar");
    pub const DANISH: LanguageType = LanguageType("da");
    pub const GERMAN: LanguageType = LanguageType("de");
    pub const GREECE: LanguageType = LanguageType("el");
    pub const FINLAND: LanguageType = LanguageType("fi");
    pub const ITALY: LanguageType = LanguageType("it");
    pub const MALAY: LanguageType = LanguageType("ms");
    pub const VIETNAM: LanguageType = LanguageType("vi");
    pub const INDONESIA: LanguageType = LanguageType("id");
    pub const NETHERLAND: LanguageType = LanguageType("nl");
    pub const THAI: LanguageType = LanguageType("th");
}