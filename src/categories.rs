use super::{fmt, FromStr, Serialize, get_value_mutex_safe};
use std::slice::Iter;
use actix_web::{Error, HttpResponse, get};

#[derive(Serialize)]
pub struct SideBarCategory {
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
            .collect::<Vec<Self>>()
    }
}

#[derive(Serialize)]
pub struct SideBarSubCategory {
    subcategory_id: String,
    subcategory_display_name: String,
}

impl SideBarSubCategory {
    pub fn new(grade: Grades) -> Vec<Self> {
        let mut vec_cate: Vec<Self> = Vec::new();
        if grade == Grades::FolkLore {
            Subjects::get_lang_iter().for_each(|each| {
                let subcategory_id = each.to_string();
                let subcategory_display_name = Subjects::get_kh(each.to_owned());
                vec_cate.push(SideBarSubCategory {
                    subcategory_id,
                    subcategory_display_name,
                })
            })
        }
        if grade == Grades::Help {
            Subjects::get_help_iter().for_each(|each| {
                let subcategory_id = each.to_string();
                let subcategory_display_name = Subjects::get_kh(each.to_owned());
                vec_cate.push(SideBarSubCategory {
                    subcategory_id,
                    subcategory_display_name,
                })
            })
        }
        else if grade == Grades::Grade1 || grade == Grades::Grade2 || grade == Grades::Grade3 {
            Subjects::get_basic_iter().for_each(|each| {
                let subcategory_id = each.to_string();
                let subcategory_display_name = Subjects::get_kh(each.to_owned());
                vec_cate.push(SideBarSubCategory {
                    subcategory_id,
                    subcategory_display_name,
                })
            })
        } else {
            Subjects::iterator().for_each(|each| {
                let subcategory_id = each.to_string();
                let subcategory_display_name = Subjects::get_kh(each.to_owned());
                vec_cate.push(SideBarSubCategory {
                    subcategory_id,
                    subcategory_display_name,
                })
            })
        }
        vec_cate
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
            self::Subjects::PE
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
        static SUBJECTS: [Subjects; 1] = [
            self::Subjects::Help,
        ];
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
            self::Subjects::Help
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
            Subjects::TompounLang => String::from("ភាសាទំពួន"),
            Subjects::PnorngLang => String::from("ភាសាព្នង"),
            Subjects::KavetLang => String::from("ភាសាកាវែត"),
            Subjects::KhmerLang => String::from("ភាសាខ្មែរ"),
            Subjects::ICT => String::from("ព័ត៌មានវិទ្យា"),
            Subjects::BasicPL => String::from("បំណិនជីវិតមូលដ្ឋាន"),
            Subjects::TeachingGuide => String::from("សៀវភៅមគ្គុទេសគ្រូថ្នាក់អប់រំពហុភាសា"),
            Subjects::ProvLang => String::from("ភាសាព្រៅ"),
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
            "MindMotion" => Ok(Subjects::MindMotion),
            "mindmotion" => Ok(Subjects::MindMotion),
            "MINDMOTION" => Ok(Subjects::MindMotion),
            "PreMath" => Ok(Subjects::PreMath),
            "PREMATH" => Ok(Subjects::PreMath),
            "premath" => Ok(Subjects::PreMath),
            "PreWriting" => Ok(Subjects::PreWriting),
            "prewriting" => Ok(Subjects::PreWriting),
            "PREWRITING" => Ok(Subjects::PreWriting),
            "Pre-Math" => Ok(Subjects::PreMath),
            "PRE-MATH" => Ok(Subjects::PreMath),
            "pre-math" => Ok(Subjects::PreMath),
            "Pre-Writing" => Ok(Subjects::PreWriting),
            "pre-writing" => Ok(Subjects::PreWriting),
            "PRE-WRITING" => Ok(Subjects::PreWriting),
            "Art" => Ok(Subjects::Art),
            "ART" => Ok(Subjects::Art),
            "art" => Ok(Subjects::Art),
            "PE" => Ok(Subjects::PE),
            "pe" => Ok(Subjects::PE),
            "PhysicalEducation" => Ok(Subjects::PE),
            "physicaleducation" => Ok(Subjects::PE),
            "PHYSICALEDUCATION" => Ok(Subjects::PE),
            "PhysicalEd" => Ok(Subjects::PE),
            "physicaled" => Ok(Subjects::PE),
            "PHYSICALED" => Ok(Subjects::PE),
            "Science" => Ok(Subjects::Science),
            "SCIENCE" => Ok(Subjects::Science),
            "science" => Ok(Subjects::Science),
            "SOCIAL" => Ok(Subjects::Social),
            "social" => Ok(Subjects::Social),
            "Social" => Ok(Subjects::Social),
            "FrenchLang" => Ok(Subjects::FrenchLang),
            "frenchlang" => Ok(Subjects::FrenchLang),
            "FRENCHLANG" => Ok(Subjects::FrenchLang),
            "EnglishLang" => Ok(Subjects::EnglishLang),
            "ENGLISHLANG" => Ok(Subjects::EnglishLang),
            "englishlang" => Ok(Subjects::EnglishLang),
            "KreungLang" => Ok(Subjects::KreungLang),
            "KREUNGLANG" => Ok(Subjects::KreungLang),
            "kreunglang" => Ok(Subjects::KreungLang),
            "PnorngLang" => Ok(Subjects::PnorngLang),
            "PNORGLANG" => Ok(Subjects::PnorngLang),
            "pnorglang" => Ok(Subjects::PnorngLang),
            "KhmerLang" => Ok(Subjects::KhmerLang),
            "KHMERLANG" => Ok(Subjects::KhmerLang),
            "khmerlang" => Ok(Subjects::KhmerLang),
            "KavetLang" => Ok(Subjects::KavetLang),
            "KAVETLANG" => Ok(Subjects::KavetLang),
            "kavetlang" => Ok(Subjects::KavetLang),
            "TompounLang" => Ok(Subjects::TompounLang),
            "TOMPOUNLANG" => Ok(Subjects::TompounLang),
            "tompounlang" => Ok(Subjects::TompounLang),
            "ProvLang" => Ok(Subjects::ProvLang),
            "PROVLANG" => Ok(Subjects::ProvLang),
            "provlang" => Ok(Subjects::ProvLang), 
            "ICT" => Ok(Subjects::ICT),
            "ict" => Ok(Subjects::ICT),
            "informationcommunicationtechnology" => Ok(Subjects::ICT),
            "InformationCommunicationTechnology" => Ok(Subjects::ICT),
            "information_communication_technology" => Ok(Subjects::ICT),
            "Information_Communication_Technology" => Ok(Subjects::ICT),
            "BasicPL" => Ok(Subjects::BasicPL),
            "BASICPL" => Ok(Subjects::BasicPL),
            "basicpl" => Ok(Subjects::BasicPL),
            "basicPL" => Ok(Subjects::BasicPL),
            "basicProfessionalLife" => Ok(Subjects::BasicPL),
            "basicprofessionallife" => Ok(Subjects::BasicPL),
            "BasicProfessionalLife" => Ok(Subjects::BasicPL),
            "Basic_Professional_Life" => Ok(Subjects::BasicPL),
            "TeachingGuide" => Ok(Subjects::TeachingGuide),
            "TEACHINGGUIDE" => Ok(Subjects::TeachingGuide),
            "teachingguide" => Ok(Subjects::TeachingGuide),
            "FlashCard" => Ok(Subjects::FlashCard),
            "FLASHCARD" => Ok(Subjects::FlashCard),
            "flashcard" => Ok(Subjects::FlashCard),
            "Help" => Ok(Subjects::Help),
            "HELP" => Ok(Subjects::Help),
            "help" => Ok(Subjects::Help),
            "None" => Ok(Subjects::None),
            "NONE" => Ok(Subjects::None),
            "none" => Ok(Subjects::None),
            _ => Err(String::from(
"Mismatch type: MindMotion, PreMath, PreWriting, Science, \
Social, Art, PE, FrenchLang, PnorngLang, KreungLang, KavetLang, \
KhmerLang, EnglishLang, ICT, BasicPL,"
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
            "Grade1" => Ok(Grades::Grade1),
            "grade1" => Ok(Grades::Grade1),
            "grade_1" => Ok(Grades::Grade1),
            "Grade_1" => Ok(Grades::Grade1),
            "GRADE1" => Ok(Grades::Grade1),
            "GRADE_1" => Ok(Grades::Grade1),
            "1" => Ok(Grades::Grade1),
            "Grade2" => Ok(Grades::Grade2),
            "grade2" => Ok(Grades::Grade2),
            "grade_2" => Ok(Grades::Grade2),
            "Grade_2" => Ok(Grades::Grade2),
            "GRADE2" => Ok(Grades::Grade2),
            "GRADE_2" => Ok(Grades::Grade2),
            "2" => Ok(Grades::Grade2),
            "Grade3" => Ok(Grades::Grade3),
            "grade3" => Ok(Grades::Grade3),
            "grade_3" => Ok(Grades::Grade3),
            "Grade_3" => Ok(Grades::Grade3),
            "GRADE3" => Ok(Grades::Grade3),
            "GRADE_3" => Ok(Grades::Grade3),
            "3" => Ok(Grades::Grade3),
            "Grade4" => Ok(Grades::Grade4),
            "grade4" => Ok(Grades::Grade4),
            "grade_4" => Ok(Grades::Grade4),
            "Grade_4" => Ok(Grades::Grade4),
            "GRADE4" => Ok(Grades::Grade4),
            "GRADE_4" => Ok(Grades::Grade4),
            "4" => Ok(Grades::Grade4),
            "Grade5" => Ok(Grades::Grade5),
            "grade5" => Ok(Grades::Grade5),
            "grade_5" => Ok(Grades::Grade5),
            "Grade_5" => Ok(Grades::Grade5),
            "GRADE5" => Ok(Grades::Grade5),
            "GRADE_5" => Ok(Grades::Grade5),
            "5" => Ok(Grades::Grade5),
            "Grade6" => Ok(Grades::Grade6),
            "grade6" => Ok(Grades::Grade6),
            "grade_6" => Ok(Grades::Grade6),
            "Grade_6" => Ok(Grades::Grade6),
            "GRADE6" => Ok(Grades::Grade6),
            "GRADE_6" => Ok(Grades::Grade6),
            "6" => Ok(Grades::Grade6),
            "FOLKLORE" => Ok(Grades::FolkLore),
            "FolkLore" => Ok(Grades::FolkLore),
            "folklore" => Ok(Grades::FolkLore),
            "Help" => Ok(Grades::Help),
            "HELP" => Ok(Grades::Help),
            "help" => Ok(Grades::Help),
            "None" => Ok(Grades::None),
            "NONE" => Ok(Grades::None),
            "none" => Ok(Grades::None),
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