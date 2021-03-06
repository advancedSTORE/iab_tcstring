use std::convert::TryFrom;

pub mod error;
pub mod model;
pub mod tcf_1;
pub mod tcf_2;
pub(crate) mod util;

use crate::decode::{
    error::TcsError,
    model::{TcModel, TcModelV1, TcModelV2},
};

impl TryFrom<&str> for TcModel {
    type Error = TcsError;

    fn try_from(val: &str) -> Result<TcModel, Self::Error> {
        Ok(match val.chars().next() {
            Some('B') => TcModel::V1(Box::new(TcModelV1::try_from(val)?)),
            Some('C') => TcModel::V2(Box::new(TcModelV2::try_from(val)?)),
            _ => return Err(TcsError::UnsupportedVersion),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::decode::model::{PublisherRestriction, PublisherRestrictionType, VendorSet};

    #[test]
    fn iab_tcf_v1_tc_model_sample() {
        assert_eq!(
            TcModel::try_from("BOEFEAyOEFEAyAHABDENAI4AAAB9vABAASA"),
            Ok(TcModel::V1(Box::new(TcModelV1 {
                created_at: 15100821554,
                updated_at: 15100821554,
                cmp_id: 7,
                cmp_version: 1,
                consent_screen: 3,
                consent_lang: String::from("EN"),
                vendor_list_version: 8,
                purposes_consent: vec![1, 2, 3],
                vendors: VendorSet {
                    is_blocklist: true,
                    list: vec![9],
                },
            })))
        );
    }

    #[test]
    fn iab_tcf_v2_tc_model_sample() {
        assert_eq!(
            TcModel::try_from("COw4XqLOw4XqLAAAAAENAXCf-v-gAAAfwIAAACngAI8AEFABgACAA4A.IAPPwAPrwA.QAPPwAPrwA.cAEAPAAAC7gAHw4AAA"),
            Ok(TcModel::V2(Box::new(TcModelV2{
                created_at: 1585246887500,
                updated_at: 1585246887500,
                cmp_id: 0,
                cmp_version: 0,
                consent_screen: 0,
                consent_language: String::from("EN"),
                vendor_list_version: 23,
                tcf_policy_version: 2,
                is_service_specific: false,
                use_non_standard_stacks: true,
                special_feature_opt_ins: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 11],
                purposes_consent: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 11],
                purposes_li_transparency: vec![12, 13, 14, 15, 16, 17, 18],
                purpose_one_treatment: true,
                publisher_country_code: String::from("AA"),
                vendors_consent: vec![2, 3, 4, 5],
                vendors_li_consent: vec![1, 2, 3, 4],
                publisher_restrictions: vec![PublisherRestriction {
                    purpose_id: 1,
                    restriction_type: PublisherRestrictionType::RequireConsent,
                    vendor_list: vec![1, 2, 3, 4, 5, 6, 7]
                }],
                disclosed_vendors: vec![1, 2, 3, 4, 5, 6, 19, 20, 21, 22, 23, 25, 27, 28, 29, 30],
                allowed_vendors: vec![1, 2, 3, 4, 5, 6, 19, 20, 21, 22, 23, 25, 27, 28, 29, 30],
                publisher_purposes_consent: vec![1, 13, 24],
                publisher_purposes_li_transparency: vec![1, 2, 3],
                custom_purposes_consent: vec![2, 3, 4, 19, 20, 21, 22, 23],
                custom_purposes_li_transparency: vec![5, 6, 7],
            })))
        );
    }
}
