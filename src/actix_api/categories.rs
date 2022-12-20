use super::{fmt, get, get_value_mutex_safe, Error, FromStr, HttpResponse, Serialize};
use std::slice::Iter;

#[derive(Serialize)]
struct SideBarCategory {
    category_id: String,
    category_display_name: String,
    icon: String,
    subcategory: Vec<SideBarSubCategory>,
}

impl SideBarCategory {
    pub fn new() -> Vec<Self> {
        Grades::iterator()
            .map(|each| {
                let category_id = each.to_string();
                let category_display_name = Grades::get_kh(each.to_owned());
                let subcategory = SideBarSubCategory::new(each.to_owned());
                let icon = Grades::get_icons(each.to_owned());
                SideBarCategory {
                    category_id,
                    icon,
                    category_display_name,
                    subcategory,
                }
            })
            .collect()
    }
}

#[derive(Serialize)]
struct SideBarSubCategory {
    subcategory_id: String,
    subcategory_display_name: String,
}

impl SideBarSubCategory {
    pub fn new(grade: Grades) -> Vec<Self> {
        if grade == Grades::FolkLore {
            Subjects::get_lang_iter().map(|each| {
                let subcategory_id = each.to_string();
                let subcategory_display_name = Subjects::get_kh(each.to_owned());
                SideBarSubCategory {
                    subcategory_id,
                    subcategory_display_name,
                }
            }).collect()
        } else if grade == Grades::Help {
            Subjects::get_help_iter().map(|each| {
                let subcategory_id = each.to_string();
                let subcategory_display_name = Subjects::get_kh(each.to_owned());
                SideBarSubCategory {
                    subcategory_id,
                    subcategory_display_name,
                }
            }).collect()
        } else if grade == Grades::Grade1 || grade == Grades::Grade2 || grade == Grades::Grade3 {
            Subjects::get_basic_iter().map(|each| {
                let subcategory_id = each.to_string();
                let subcategory_display_name = Subjects::get_kh(each.to_owned());
                SideBarSubCategory {
                    subcategory_id,
                    subcategory_display_name,
                }
            }).collect()
        } else {
            Subjects::iterator().map(|each| {
                let subcategory_id = each.to_string();
                let subcategory_display_name = Subjects::get_kh(each.to_owned());
                SideBarSubCategory {
                    subcategory_id,
                    subcategory_display_name,
                }
            }).collect()
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize)]
pub enum Subjects {
    MindMotion,
    PreMath,
    PreWriting,
    Science,
    Social,
    Art,
    PE,
    FrenchLang,
    EnglishLang,
    KreungLang,
    TompounLang,
    ProvLang,
    PnorngLang,
    KavetLang,
    KhmerLang,
    ICT,
    BasicPL,
    TeachingGuide,
    FlashCard,
    Help,
    None,
}

impl Subjects {
    pub fn get_basic_iter() -> Iter<'static, Subjects> {
        static SUBJECTS: [Subjects; 16] = [
            self::Subjects::FlashCard,
            self::Subjects::EnglishLang,
            self::Subjects::KhmerLang,
            self::Subjects::KavetLang,
            self::Subjects::FrenchLang,
            self::Subjects::PnorngLang,
            self::Subjects::TompounLang,
            self::Subjects::KreungLang,
            self::Subjects::KhmerLang,
            self::Subjects::ProvLang,
            self::Subjects::TeachingGuide,
            self::Subjects::MindMotion,
            self::Subjects::PreMath,
            self::Subjects::PreWriting,
            self::Subjects::Art,
            self::Subjects::PE,
        ];
        SUBJECTS.iter()
    }

    pub fn get_lang_iter() -> Iter<'static, Subjects> {
        static SUBJECTS: [Subjects; 10] = [
            self::Subjects::EnglishLang,
            self::Subjects::KhmerLang,
            self::Subjects::KavetLang,
            self::Subjects::FrenchLang,
            self::Subjects::PnorngLang,
            self::Subjects::TompounLang,
            self::Subjects::KreungLang,
            self::Subjects::KhmerLang,
            self::Subjects::ProvLang,
            self::Subjects::TeachingGuide,
        ];
        SUBJECTS.iter()
    }

    pub fn get_help_iter() -> Iter<'static, Subjects> {
        static SUBJECTS: [Subjects; 1] = [self::Subjects::Help];
        SUBJECTS.iter()
    }

    pub fn iterator() -> Iter<'static, Subjects> {
        static SUBJECTS: [Subjects; 21] = [
            self::Subjects::TeachingGuide,
            self::Subjects::EnglishLang,
            self::Subjects::KhmerLang,
            self::Subjects::KavetLang,
            self::Subjects::FrenchLang,
            self::Subjects::PnorngLang,
            self::Subjects::TompounLang,
            self::Subjects::KreungLang,
            self::Subjects::KhmerLang,
            self::Subjects::ProvLang,
            self::Subjects::FlashCard,
            self::Subjects::MindMotion,
            self::Subjects::PreMath,
            self::Subjects::PreWriting,
            self::Subjects::Science,
            self::Subjects::Social,
            self::Subjects::Art,
            self::Subjects::PE,
            self::Subjects::ICT,
            self::Subjects::BasicPL,
            self::Subjects::Help,
        ];
        SUBJECTS.iter()
    }

    pub fn get_kh(subject: Self) -> String {
        match subject {
            Subjects::MindMotion => String::from("ចិត្តចលភាព"),
            Subjects::PreMath => String::from("បុរេគណិត"),
            Subjects::PreWriting => String::from("បុរេសំណេរ"),
            Subjects::Science => String::from("វិទ្យាសាស្រ្ត"),
            Subjects::Social => String::from("សង្គម"),
            Subjects::Art => String::from("អប់រំសិល្បៈ"),
            Subjects::PE => String::from("អប់រំកាយនិងកីឡា"),
            Subjects::FrenchLang => String::from("ភាសាបារាំង"),
            Subjects::EnglishLang => String::from("ភាសាអង់គ្លេស"),
            Subjects::KreungLang => String::from("ភាសាគ្រឹង"),
            Subjects::PnorngLang => String::from("ភាសាព្នង"),
            Subjects::KavetLang => String::from("ភាសាកាវែត"),
            Subjects::TompounLang => String::from("ភាសាទំពួន"),
            Subjects::ProvLang => String::from("ភាសាព្រៅ"),
            Subjects::KhmerLang => String::from("ភាសាខ្មែរ"),
            Subjects::ICT => String::from("ព័ត៌មានវិទ្យា"),
            Subjects::BasicPL => String::from("បំណិនជីវិតមូលដ្ឋាន"),
            Subjects::TeachingGuide => String::from("សៀវភៅមគ្គុទេសគ្រូថ្នាក់អប់រំពហុភាសា"),
            Subjects::FlashCard => String::from("កាតប្លាស់"),
            Subjects::Help => String::from("ជំនួយ"),
            Subjects::None => String::from(""),
        }
    }
}

impl FromStr for Subjects {
    type Err = String;

    fn from_str(input: &str) -> Result<Subjects, Self::Err> {
        match input {
            "MindMotion" | "mindmotion" | "MINDMOTION" | "ចិត្តចលភាព" => {
                Ok(Subjects::MindMotion)
            }
            "PreMath" | "PREMATH" | "premath" | "បុរេគណិត" => Ok(Subjects::PreMath),
            "PreWriting" | "prewriting" | "PREWRITING" | "បុរេសំណេរ" => {
                Ok(Subjects::PreWriting)
            }
            "Science" | "SCIENCE" | "science" | "វិទ្យាសាស្រ្ត" => {
                Ok(Subjects::Science)
            }
            "SOCIAL" | "social" | "Social" | "សង្គម" => Ok(Subjects::Social),
            "Art" | "ART" | "art" | "អប់រំសិល្បៈ" => Ok(Subjects::Art),
            "PE"
            | "pe"
            | "PhysicalEducation"
            | "physicaleducation"
            | "PHYSICALEDUCATION"
            | "PhysicalEd"
            | "physicaled"
            | "PHYSICALED"
            | "អប់រំកាយនិងកីឡា" => Ok(Subjects::PE),
            "FrenchLang" | "frenchlang" | "FRENCHLANG" | "ភាសាបារាំង" | "បារាំង" => {
                Ok(Subjects::FrenchLang)
            }
            "EnglishLang" | "ENGLISHLANG" | "englishlang" | "ភាសាអង់គ្លេស" | "អង់គ្លេស" => {
                Ok(Subjects::EnglishLang)
            }
            "KreungLang" | "KREUNGLANG" | "kreunglang" | "ភាសាគ្រឹង" | "គ្រឹង" => {
                Ok(Subjects::KreungLang)
            }
            "PnorngLang" | "PNORGLANG" | "pnorglang" | "ភាសាព្នង" | "ព្នង" => {
                Ok(Subjects::PnorngLang)
            }
            "KavetLang" | "KAVETLANG" | "kavetlang" | "ភាសាកាវែត" | "កាវែត" => {
                Ok(Subjects::KavetLang)
            }
            "TompounLang" | "TOMPOUNLANG" | "tompounlang" | "ភាសាទំពួន" | "ទំពួន" => {
                Ok(Subjects::TompounLang)
            }
            "ProvLang" | "PROVLANG" | "provlang" | "ភាសាព្រៅ" | "ព្រៅ" => {
                Ok(Subjects::ProvLang)
            }
            "KhmerLang" | "KHMERLANG" | "khmerlang" | "ភាសាខ្មែរ" | "ខ្មែរ" => {
                Ok(Subjects::KhmerLang)
            }
            "ICT"
            | "ict"
            | "ព័ត៌មានវិទ្យា"
            | "ពត៌មានវិទ្យា"
            | "informationcommunicationtechnology"
            | "InformationCommunicationTechnology"
            | "information_communication_technology"
            | "Information_Communication_Technology" => Ok(Subjects::ICT),
            "BasicPL"
            | "បំណិនជីវិតមូលដ្ឋាន"
            | "BASICPL"
            | "basicpl"
            | "basicPL"
            | "basicProfessionalLife"
            | "basicprofessionallife"
            | "BasicProfessionalLife"
            | "Basic_Professional_Life" => Ok(Subjects::BasicPL),
            "TeachingGuide"
            | "TEACHINGGUIDE"
            | "teachingguide"
            | "សៀវភៅមគ្គុទេសគ្រូថ្នាក់អប់រំពហុភាសា"
            | "គ្រូ" => Ok(Subjects::TeachingGuide),
            "FlashCard" | "FLASHCARD" | "flashcard" | "កាតប្លាស់" => {
                Ok(Subjects::FlashCard)
            }
            "Help" | "HELP" | "help" | "ជំនួយ" => Ok(Subjects::Help),
            "None" | "NONE" | "none" => Ok(Subjects::None),
            _ => Err(String::from(
                "Mismatch type: MindMotion, PreMath, PreWriting, Science, \
Social, Art, PE, FrenchLang, PnorngLang, KreungLang, KavetLang, \
KhmerLang, EnglishLang, ICT, BasicPL,",
            )),
        }
    }
}

impl fmt::Display for Subjects {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Subjects::MindMotion => write!(f, "MindMotion"),
            Subjects::PreMath => write!(f, "PreMath"),
            Subjects::Science => write!(f, "Science"),
            Subjects::Social => write!(f, "Social"),
            Subjects::PreWriting => write!(f, "PreWriting"),
            Subjects::Art => write!(f, "Art"),
            Subjects::PE => write!(f, "PE"),
            Subjects::EnglishLang => write!(f, "EnglishLang"),
            Subjects::ICT => write!(f, "ICT"),
            Subjects::FrenchLang => write!(f, "FrenchLang"),
            Subjects::BasicPL => write!(f, "BasicPL"),
            Subjects::None => write!(f, "None"),
            Subjects::KreungLang => write!(f, "KreungLang"),
            Subjects::TompounLang => write!(f, "TompounLang"),
            Subjects::PnorngLang => write!(f, "PnorngLang"),
            Subjects::KavetLang => write!(f, "KavetLang"),
            Subjects::KhmerLang => write!(f, "KhmerLang"),
            Subjects::TeachingGuide => write!(f, "TeachingGuide"),
            Subjects::ProvLang => write!(f, "ProvLang"),
            Subjects::FlashCard => write!(f, "FlashCard"),
            Subjects::Help => write!(f, "Help"),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq)]
pub enum Grades {
    Grade1,
    Grade2,
    Grade3,
    Grade4,
    Grade5,
    Grade6,
    FolkLore,
    Help,
    None,
}

impl Grades {
    pub fn get_kh(grade: Self) -> String {
        match grade {
            Grades::Grade1 => String::from("ថ្នាក់ទី១"),
            Grades::Grade2 => String::from("ថ្នាក់ទី២"),
            Grades::Grade3 => String::from("ថ្នាក់ទី៣"),
            Grades::Grade4 => String::from("ថ្នាក់ទី៤"),
            Grades::Grade5 => String::from("ថ្នាក់ទី៥"),
            Grades::Grade6 => String::from("ថ្នាក់ទី៦"),
            Grades::None => String::from("ថ្នាក់ទី០"),
            Grades::FolkLore => String::from("សៀវភៅរឿងនិទាន"),
            Grades::Help => String::from("ជំនួយ"),
        }
    }

    pub fn get_icons(grade: Self) -> String {
        match grade {
            Grades::Grade1 => get_value_mutex_safe("GRADE_1_THUMBNAIL"),
            Grades::Grade2 => get_value_mutex_safe("GRADE_2_THUMBNAIL"),
            Grades::Grade3 => get_value_mutex_safe("GRADE_3_THUMBNAIL"),
            Grades::Grade4 => get_value_mutex_safe("GRADE_4_THUMBNAIL"),
            Grades::Grade5 => get_value_mutex_safe("GRADE_5_THUMBNAIL"),
            Grades::Grade6 => get_value_mutex_safe("GRADE_6_THUMBNAIL"),
            Grades::None => get_value_mutex_safe("GRADE_1_THUMBNAIL"),
            Grades::FolkLore => get_value_mutex_safe("FOLKLORE_THUMBNAIL"),
            Grades::Help => get_value_mutex_safe("HELP_THUMBNAIL"),
        }
    }

    pub fn iterator() -> Iter<'static, Grades> {
        static GRADES: [Grades; 8] = [
            self::Grades::Grade1,
            self::Grades::Grade2,
            self::Grades::Grade3,
            self::Grades::Grade4,
            self::Grades::Grade5,
            self::Grades::Grade6,
            self::Grades::FolkLore,
            self::Grades::Help,
        ];
        GRADES.iter()
    }
}

impl FromStr for Grades {
    type Err = String;

    fn from_str(input: &str) -> Result<Grades, Self::Err> {
        match input {
            "Grade1" | "grade1" | "grade_1" | "Grade_1" | "GRADE1" | "GRADE_1" | "ថ្នាក់ទី១" | "1" => {
                Ok(Grades::Grade1)
            }
            "Grade2" | "grade2" | "grade_2" | "Grade_2" | "GRADE2" | "GRADE_2" | "ថ្នាក់ទី២" | "2" => {
                Ok(Grades::Grade2)
            }
            "Grade3" | "grade3" | "grade_3" | "Grade_3" | "GRADE3" | "GRADE_3" | "ថ្នាក់ទី៣" | "3" => {
                Ok(Grades::Grade3)
            }
            "Grade4" | "grade4" | "grade_4" | "Grade_4" | "GRADE4" | "GRADE_4" | "ថ្នាក់ទី៤" | "4" => {
                Ok(Grades::Grade4)
            }
            "Grade5" | "grade5" | "grade_5" | "Grade_5" | "GRADE5" | "GRADE_5" | "ថ្នាក់ទី៥" | "5" => {
                Ok(Grades::Grade5)
            }
            "Grade6" | "grade6" | "grade_6" | "Grade_6" | "GRADE6" | "GRADE_6" | "ថ្នាក់ទី៦" | "6" => {
                Ok(Grades::Grade6)
            }
            "FOLKLORE" | "FolkLore" | "សៀវភៅរឿងនិទាន" | "folklore" => {
                Ok(Grades::FolkLore)
            }
            "Help" | "HELP" | "ជំនួយ" | "help" => Ok(Grades::Help),
            "None" | "NONE" | "none" => Ok(Grades::None),
            _ => Err(String::from("Mismatch type: 1, 2, 3, 4, 5, 6")),
        }
    }
}

impl fmt::Display for Grades {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Grades::Grade1 => write!(f, "Grade1"),
            Grades::Grade2 => write!(f, "Grade2"),
            Grades::Grade3 => write!(f, "Grade3"),
            Grades::Grade4 => write!(f, "Grade4"),
            Grades::Grade5 => write!(f, "Grade5"),
            Grades::Grade6 => write!(f, "Grade6"),
            Grades::FolkLore => write!(f, "FolkLore"),
            Grades::Help => write!(f, "Help"),
            Grades::None => write!(f, "None"),
        }
    }
}

#[get("/public/api/sidebar")]
pub async fn get_sidebar() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(SideBarCategory::new()))
}
