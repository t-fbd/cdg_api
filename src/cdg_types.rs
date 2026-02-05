//! # CDG Types
//!
//! This module defines various types used by the CDG API client, particularly enums for API
//! endpoint parameters.

use serde::{Deserialize, Serialize};

// =========================================
// Enums for API Endpoint Parameters Values
// =========================================

/// Enum representing the response formats available for API endpoints.
///
/// This enum is used to specify the desired format of the API response,
/// allowing consumers to choose between JSON and XML formats.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Default)]
#[serde(rename_all = "lowercase")]
pub enum FormatType {
    /// JSON format.
    #[default]
    Json,

    /// XML format.
    Xml,
}

impl FormatType {
    /// Converts the [`FormatType`] variant to its corresponding query parameter string.
    ///
    /// # Returns
    ///
    /// A [`String`] representing the query parameter (e.g., `"format=json"`).
    pub fn to_query_param(&self) -> String {
        match self {
            FormatType::Json => "format=json".to_string(),
            FormatType::Xml => "format=xml".to_string(),
        }
    }

    /// Converts the [`FormatType`] variant to its lowercase string representation.
    ///
    /// # Returns
    ///
    /// A [`String`] representing the format type (e.g., `"json"` or `"xml"`).
    pub fn to_string(&self) -> String {
        match self {
            FormatType::Json => "json".to_string(),
            FormatType::Xml => "xml".to_string(),
        }
    }
}

/// Enum representing the sorting options available for API endpoints.
///
/// This enum allows consumers to sort responses based on update dates
/// in ascending or descending order.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Default)]
#[serde(rename_all = "lowercase")]
pub enum SortType {
    /// SortType by update date in ascending order.
    #[default]
    UpdateDateAsc,

    /// SortType by update date in descending order.
    UpdateDateDesc,
}

impl SortType {
    /// Converts the [`SortType`] variant to its corresponding query parameter string.
    ///
    /// # Returns
    ///
    /// A [`String`] representing the sort query parameter
    /// (e.g., `"sort=updateDate+asc"`).
    pub fn to_query_param(&self) -> String {
        match self {
            SortType::UpdateDateAsc => "sort=updateDate+asc".to_string(),
            SortType::UpdateDateDesc => "sort=updateDate+desc".to_string(),
        }
    }

    /// Converts the [`SortType`] variant to its lowercase string representation.
    ///
    /// # Returns
    ///
    /// A [`String`] representing the sort order (`"asc"` or `"desc"`).
    pub fn to_string(&self) -> String {
        match self {
            SortType::UpdateDateAsc => "asc".to_string(),
            SortType::UpdateDateDesc => "desc".to_string(),
        }
    }
}

/// Enum representing various bill types in legislative processes.
///
/// This enum categorizes bills based on their origin and nature within
/// the legislative system.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Default)]
pub enum BillType {
    /// House Resolution ([`hr`]).
    #[default]
    Hr, // House Resolution

    /// Senate Bill ([`s`]).
    S, // Senate

    /// House Joint Resolution ([`hjres`]).
    Hjres, // House Joint Resolution

    /// Senate Joint Resolution ([`sjres`]).
    Sjres, // Senate Joint Resolution

    /// House Concurrent Resolution ([`hconres`]).
    Hconres, // House Concurrent Resolution

    /// Senate Concurrent Resolution ([`sconres`]).
    Sconres, // Senate Concurrent Resolution

    /// House Simple Resolution ([`hres`]).
    Hres, // House Simple Resolution

    /// Senate Simple Resolution ([`sres`]).
    Sres, // Senate Simple Resolution
}

impl BillType {
    /// Converts the [`BillType`] variant to its lowercase string representation.
    ///
    /// # Returns
    ///
    /// A [`String`] representing the bill type (e.g., `"hr"`, `"s"`, etc.).
    pub fn to_string(&self) -> String {
        match self {
            BillType::Hr => "hr".to_string(),
            BillType::S => "s".to_string(),
            BillType::Hjres => "hjres".to_string(),
            BillType::Sjres => "sjres".to_string(),
            BillType::Hconres => "hconres".to_string(),
            BillType::Sconres => "sconres".to_string(),
            BillType::Hres => "hres".to_string(),
            BillType::Sres => "sres".to_string(),
        }
    }

    /// Converts a `&str` to the corresponding [`BillType`] variant.
    pub fn from_str(s: &str) -> Option<BillType> {
        match s.to_lowercase().as_str() {
            "hr" => Some(BillType::Hr),
            "s" => Some(BillType::S),
            "hjres" => Some(BillType::Hjres),
            "sjres" => Some(BillType::Sjres),
            "hconres" => Some(BillType::Hconres),
            "sconres" => Some(BillType::Sconres),
            "hres" => Some(BillType::Hres),
            "sres" => Some(BillType::Sres),
            _ => None,
        }
    }
}

/// Enum representing the types of amendments in legislative processes.
///
/// This enum categorizes amendments based on their origin within
/// the legislative chambers.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Default)]
pub enum AmendmentType {
    /// House Amendment ([`hamdt`]).
    Hamdt, // House Amendment

    /// Senate Amendment ([`samdt`]).
    #[default]
    Samdt, // Senate Amendment

    /// Senate Unnumbered Amendment ([`suamdt`]).
    Suamdt, // Senate Unnumbered Amendment
}

impl AmendmentType {
    /// Converts the [`AmendmentType`] variant to its lowercase string representation.
    ///
    /// # Returns
    ///
    /// A [`String`] representing the amendment type (e.g., `"hamdt"`, `"samdt"`, etc.).
    pub fn to_string(&self) -> String {
        match self {
            AmendmentType::Hamdt => "hamdt".to_string(),
            AmendmentType::Samdt => "samdt".to_string(),
            AmendmentType::Suamdt => "suamdt".to_string(),
        }
    }
}

/// Enum representing the chambers of Congress.
///
/// This enum distinguishes between the House, Senate, and Joint committees
/// within the legislative body.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Default)]
pub enum ChamberType {
    /// House chamber.
    House,

    /// Senate chamber.
    #[default]
    Senate,

    /// Joint chamber (both House and Senate).
    Joint,

    /// No chamber specified.
    NoChamber,
}

impl ChamberType {
    /// Converts the [`ChamberType`] variant to its lowercase string representation.
    ///
    /// # Returns
    ///
    /// A [`String`] representing the chamber type (e.g., `"house"`, `"senate"`, `"joint"`).
    pub fn to_string(&self) -> String {
        match self {
            ChamberType::House => "house".to_string(),
            ChamberType::Senate => "senate".to_string(),
            ChamberType::Joint => "joint".to_string(),
            ChamberType::NoChamber => "nochamber".to_string(),
        }
    }
}

/// Enum representing different types of communications handled by committees.
///
/// This enum categorizes the various communication types such as executive
/// communications, presidential messages, petitions, etc.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Default)]
pub enum CommunicationType {
    /// Executive Communication ([`ec`]).
    #[default]
    Ec, // Executive Communication

    /// Message from the President ([`ml`]).
    Ml, // Message from the President

    /// Presidential Message ([`pm`]).
    Pm, // Presidential Message

    /// Petition ([`pt`]).
    Pt, // Petition
}

impl CommunicationType {
    /// Converts the [`CommunicationType`] variant to its lowercase string representation.
    ///
    /// # Returns
    ///
    /// A [`String`] representing the communication type (e.g., `"ec"`, `"ml"`, etc.).
    pub fn to_string(&self) -> String {
        match self {
            CommunicationType::Ec => "ec".to_string(),
            CommunicationType::Ml => "ml".to_string(),
            CommunicationType::Pm => "pm".to_string(),
            CommunicationType::Pt => "pt".to_string(),
        }
    }
}

/// Enum representing different types of laws.
///
/// This enum distinguishes between public and private laws.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Default)]
pub enum LawType {
    /// Public Law ([`pub`]).
    #[default]
    Pub, // Public Law

    /// Private Law ([`priv`]).
    Priv, // Private Law
}

impl LawType {
    /// Converts the [`LawType`] variant to its lowercase string representation.
    ///
    /// # Returns
    ///
    /// A [`String`] representing the law type (`"pub"` or `"priv"`).
    pub fn to_string(&self) -> String {
        match self {
            LawType::Pub => "pub".to_string(),
            LawType::Priv => "priv".to_string(),
        }
    }
}

/// Enum representing the types of committee reports.
///
/// This enum categorizes committee reports based on their origin within
/// the legislative chambers.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Default)]
pub enum CommitteeReportType {
    /// House Report ([`hrpt`]).
    Hrpt, // House Report

    /// Senate Report ([`srpt`]).
    Srpt, // Senate Report

    /// House Document ([`hdoc`]).
    Hdoc, // House Document

    /// Senate Document ([`sdoc`]).
    Sdoc, // Senate Document

    /// Conference Report ([`crpt`]).
    #[default]
    Crpt, // Conference Report
}

impl CommitteeReportType {
    /// Converts the [`CommitteeReportType`] variant to its lowercase string representation.
    ///
    /// # Returns
    ///
    /// A [`String`] representing the committee report type (e.g., `"hrpt"`, `"srpt"`, etc.).
    pub fn to_string(&self) -> String {
        match self {
            CommitteeReportType::Hrpt => "hrpt".to_string(),
            CommitteeReportType::Srpt => "srpt".to_string(),
            CommitteeReportType::Hdoc => "hdoc".to_string(),
            CommitteeReportType::Sdoc => "sdoc".to_string(),
            CommitteeReportType::Crpt => "crpt".to_string(),
        }
    }
}
