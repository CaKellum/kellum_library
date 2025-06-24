use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum MPAARating {
    GeneralAudiences,
    ParentalGuidance,
    ParentsStronglyCautioned,
    Restricted,
    AdultsOnly,
}

impl MPAARating {
    pub fn from_string(rating_string: &str) -> Option<Self> {
        match rating_string {
            "GeneralAudiences" => Some(Self::GeneralAudiences),
            "ParentalGuidance" => Some(Self::ParentalGuidance),
            "ParentsStronglyCautioned" => Some(Self::ParentsStronglyCautioned),
            "Restricted" => Some(Self::Restricted),
            "AdultsOnly" => Some(Self::AdultsOnly),
            _ => None,
        }
    }

    pub fn string(&self) -> String {
        match self {
            Self::GeneralAudiences => "GeneralAudiences".to_string(),
            Self::ParentalGuidance => "ParentalGuidance".to_string(),
            Self::ParentsStronglyCautioned => "ParentsStronglyCautioned".to_string(),
            Self::Restricted => "Restricted".to_string(),
            Self::AdultsOnly => "AdultsOnly".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum MotionPictureFormat {
    BluRay,
    UltraHD,
    DVD,
    VHS,
}

impl MotionPictureFormat {
    pub fn from_string(rating_string: &str) -> Option<Self> {
        match rating_string {
            "BluRay" => Some(Self::BluRay),
            "UltraHD" => Some(Self::UltraHD),
            "DVD" => Some(Self::DVD),
            "VHS" => Some(Self::VHS),
            _ => None,
        }
    }

    pub fn string(&self) -> String {
        match self {
            Self::BluRay => "BluRay".to_string(),
            Self::DVD => "DVD".to_string(),
            Self::UltraHD => "UltraHD".to_string(),
            Self::VHS => "VHS".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Movie {
    pub id: String,
    pub title: String,
    pub format: MotionPictureFormat,
    pub rating: MPAARating,
}

impl Movie {
    pub fn new(title: &str, format: &str, rating: &str) -> Option<Self> {
        let format = MotionPictureFormat::from_string(format)?;
        let rating = MPAARating::from_string(rating)?;
        return Some(Movie {
            id: Uuid::new_v4().to_string(),
            title: title.to_string(),
            format,
            rating,
        });
    }
}
