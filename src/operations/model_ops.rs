use crate::types::ActivityMetaData;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BroadCastActivity {
    pub user_id: i32,
    pub subject_user_id: i32,
    pub activity_type: String,
    pub content: String,
}

impl BroadCastActivity {
    pub async fn new(activity_data: ActivityMetaData) -> Option<Self> {
        let ActivityMetaData {
            user_id,
            activity_type,
            subject_user_id,
            content,
        } = activity_data;

        let validator = Self::validate_metadata(subject_user_id, &activity_type, &content);
        if !validator {
            return None;
        };
        let data = Self {
            user_id,
            activity_type,
            subject_user_id,
            content,
        };
        Some(data)
    }

    fn validate_metadata(subject_user_id: i32, activity_type: &String, content: &String) -> bool {
        if subject_user_id == 0 {
            let broadcast_pattern = Regex::new(r"^(?i:BROADCAST)$").unwrap();
            if broadcast_pattern.is_match(activity_type.as_str()) {
                let url_pattern = Regex::new(r"https?://[^\s/$.?#].[^\s]*").unwrap();
                if url_pattern.is_match(content.as_str()) {
                    true
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PostActivity {
    pub user_id: i32,
    pub subject_user_id: i32,
    pub activity_type: String,
    pub content: String,
}

impl PostActivity {
    pub async fn new(activity_data: ActivityMetaData) -> Option<Self> {
        let ActivityMetaData {
            user_id,
            activity_type,
            subject_user_id,
            content,
        } = activity_data;

        let validator = Self::validate_metadata(subject_user_id, &activity_type, &content);
        if !validator {
            return None;
        };
        println!("{:?}", validator);
        let data = Self {
            user_id,
            activity_type,
            subject_user_id,
            content,
        };
        Some(data)
    }

    fn validate_metadata(subject_user_id: i32, activity_type: &str, content: &str) -> bool {
        if subject_user_id == 0 {
            let post_pattern = Regex::new(r"^(?i:POST)$").unwrap();

            if post_pattern.is_match(activity_type)
                && !content.trim().is_empty()
                && content.chars().count() < 240
            {
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}
