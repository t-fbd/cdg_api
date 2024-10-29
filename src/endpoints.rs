use serde::{Deserialize, Serialize};

pub const BASE_URL: &str = "https://api.congress.gov/v3/";

#[derive(Debug, Serialize, Deserialize)]
pub enum Endpoints {
    Bill {
        congress: Option<i32>,
        bill_type: Option<BillType>,
        bill_number: Option<String>,
        bill_option: Option<BillOption>,
        parameters: Option<CommonParams>,
    },
    Law {
        congress: i32,
        law_type: Option<LawType>,
        law_number: Option<i32>,
        parameters: Option<CommonParams>,
    },
    Amendment {
        congress: Option<i32>,
        amendment_type: Option<AmendmentType>,
        amendment_number: Option<String>,
        amendment_option: Option<AmendmentOption>,
        parameters: Option<CommonParams>,
    },
    Summaries {
        congress: Option<i32>,
        bill_type: Option<BillType>,
        parameters: Option<CommonParams>,
    },
    Congress {
        congress: Option<i32>,
        current: Option<bool>,
        parameters: Option<CommonParams>,
    },
    Member {
        bio_guide_id: Option<String>,
        // bio_guide_id_option is only used when bio_guide_id is provided, and even then it is
        // optional
        bio_guide_id_option: Option<BioGuideIdOption>,
        // congress is only used when /member/congress/{congress} is requested
        congress: Option<i32>,
        // if state_code, district AND congress are provided, the endpoint is
        // /member/congress/{congress}/{state_code}/{district}
        // otherwise, if state_code is provided, the endpoints are /member/{state_code} and
        // /member/{state_code}/{district}
        state_code: Option<String>,
        district: Option<i32>,
        parameters: Option<CommonParams>,
    },
    Committee {
        chamber: Option<ChamberType>,
        congress: Option<i32>,
        committee_code: Option<String>,
        committee_option: Option<CommitteeOption>,
        parameters: Option<CommonParams>,
    },
    CommitteeReport {
        congress: Option<i32>,
        report_type: Option<String>,
        report_number: Option<String>,
        text: bool,
        parameters: Option<CommonParams>,
    },
    CommitteePrint {
        congress: Option<i32>,
        chamber: Option<ChamberType>,
        jacket_number: Option<i32>,
        text: bool,
        parameters: Option<CommonParams>,
    },
    CommitteeMeeting {
        congress: Option<i32>,
        chamber: Option<ChamberType>,
        event_id: Option<String>,
        parameters: Option<CommonParams>,
    },
    Hearing {
        congress: Option<i32>,
        chamber: Option<ChamberType>,
        jacket_number: Option<i32>,
        parameters: Option<CommonParams>,
    },
    CongressionalRecord {
        parameters: Option<CommonParams>,
    },
    DailyCongressionalRecord {
        volume_number: Option<i32>,
        issue_number: Option<i32>,
        articles: bool,
        parameters: Option<CommonParams>,
    },
    BoundCongressionalRecord {
        year: Option<i32>,
        month: Option<i32>,
        day: Option<i32>,
        parameters: Option<CommonParams>,
    },
    HouseCommunication {
        congress: Option<i32>,
        communication_type: Option<CommunicationType>,
        communication_number: Option<String>,
        parameters: Option<CommonParams>,
    },
    HouseRequirement {
        requirement_number: Option<i32>,
        matching_communications: bool,
        parameters: Option<CommonParams>,
    },
    SenateCommunication {
        congress: Option<i32>,
        communication_type: Option<CommunicationType>,
        communication_number: Option<String>,
        parameters: Option<CommonParams>,
    },
    Nomination {
        congress: Option<i32>,
        nomination_number: Option<String>,
        ordinal: Option<i32>,
        // if ordinal is provided, nomination_option is not used
        nomination_option: Option<NominationOption>,
        parameters: Option<CommonParams>,
    },
    Treaty {
        congress: Option<i32>,
        treaty_number: Option<String>,
        treaty_suffix: Option<String>,
        actions: bool,
        committees: bool,
        parameters: Option<CommonParams>,
    },
}

pub trait NewEndpoint {
    fn new_bill(congress: Option<i32>, bill_type: Option<BillType>, bill_number: Option<String>, bill_option: Option<BillOption>, parameters: Option<CommonParams>) -> Self;
    fn new_law(congress: i32, law_type: Option<LawType>, law_number: Option<i32>, parameters: Option<CommonParams>) -> Self;
    fn new_amendment(congress: Option<i32>, amendment_type: Option<AmendmentType>, amendment_number: Option<String>, amendment_option: Option<AmendmentOption>, parameters: Option<CommonParams>) -> Self;
    fn new_summaries(congress: Option<i32>, bill_type: Option<BillType>, parameters: Option<CommonParams>) -> Self;
    fn new_congress(congress: Option<i32>, current: Option<bool>, parameters: Option<CommonParams>) -> Self;
    fn new_member(bio_guide_id: Option<String>, bio_guide_id_option: Option<BioGuideIdOption>, congress: Option<i32>, state_code: Option<String>, district: Option<i32>, parameters: Option<CommonParams>) -> Self;
    fn new_committee(chamber: Option<ChamberType>, congress: Option<i32>, committee_code: Option<String>, committee_option: Option<CommitteeOption>, parameters: Option<CommonParams>) -> Self;
    fn new_committee_report(congress: Option<i32>, report_type: Option<String>, report_number: Option<String>, text: bool, parameters: Option<CommonParams>) -> Self;
    fn new_committee_print(congress: Option<i32>, chamber: Option<ChamberType>, jacket_number: Option<i32>, text: bool, parameters: Option<CommonParams>) -> Self;
    fn new_committee_meeting(congress: Option<i32>, chamber: Option<ChamberType>, event_id: Option<String>, parameters: Option<CommonParams>) -> Self;
    fn new_hearing(congress: Option<i32>, chamber: Option<ChamberType>, jacket_number: Option<i32>, parameters: Option<CommonParams>) -> Self;
    fn new_congressional_record(parameters: Option<CommonParams>) -> Self;
    fn new_daily_congressional_record(volume_number: Option<i32>, issue_number: Option<i32>, articles: bool, parameters: Option<CommonParams>) -> Self;
    fn new_bound_congressional_record(year: Option<i32>, month: Option<i32>, day: Option<i32>, parameters: Option<CommonParams>) -> Self;
    fn new_house_communication(congress: Option<i32>, communication_type: Option<CommunicationType>, communication_number: Option<String>, parameters: Option<CommonParams>) -> Self;
    fn new_house_requirement(requirement_number: Option<i32>, matching_communications: bool, parameters: Option<CommonParams>) -> Self;
    fn new_senate_communication(congress: Option<i32>, communication_type: Option<CommunicationType>, communication_number: Option<String>, parameters: Option<CommonParams>) -> Self;
    fn new_nomination(congress: Option<i32>, nomination_number: Option<String>, ordinal: Option<i32>, nomination_option: Option<NominationOption>, parameters: Option<CommonParams>) -> Self;
    fn new_treaty(congress: Option<i32>, treaty_number: Option<String>, treaty_suffix: Option<String>, actions: bool, committees: bool, parameters: Option<CommonParams>) -> Self;
}

impl NewEndpoint for Endpoints {
    fn new_bill(congress: Option<i32>, bill_type: Option<BillType>, bill_number: Option<String>, bill_option: Option<BillOption>, parameters: Option<CommonParams>) -> Endpoints {
        Endpoints::Bill { congress, bill_type, bill_number, bill_option, parameters }
    }

    fn new_law(congress: i32, law_type: Option<LawType>, law_number: Option<i32>, parameters: Option<CommonParams>) -> Endpoints {
        Endpoints::Law { congress, law_type, law_number, parameters }
    }

    fn new_amendment(congress: Option<i32>, amendment_type: Option<AmendmentType>, amendment_number: Option<String>, amendment_option: Option<AmendmentOption>, parameters: Option<CommonParams>) -> Endpoints {
        Endpoints::Amendment { congress, amendment_type, amendment_number, amendment_option, parameters }
    }

    fn new_summaries(congress: Option<i32>, bill_type: Option<BillType>, parameters: Option<CommonParams>) -> Endpoints {
        Endpoints::Summaries { congress, bill_type, parameters }
    }

    fn new_congress(congress: Option<i32>, current: Option<bool>, parameters: Option<CommonParams>) -> Endpoints {
        Endpoints::Congress { congress, current, parameters }
    }

    fn new_member(bio_guide_id: Option<String>, bio_guide_id_option: Option<BioGuideIdOption>, congress: Option<i32>, state_code: Option<String>, district: Option<i32>, parameters: Option<CommonParams>) -> Endpoints {
        Endpoints::Member { bio_guide_id, bio_guide_id_option, congress, state_code, district, parameters }
    }

    fn new_committee(chamber: Option<ChamberType>, congress: Option<i32>, committee_code: Option<String>, committee_option: Option<CommitteeOption>, parameters: Option<CommonParams>) -> Endpoints {
        Endpoints::Committee { chamber, congress, committee_code, committee_option, parameters }
    }

    fn new_committee_report(congress: Option<i32>, report_type: Option<String>, report_number: Option<String>, text: bool, parameters: Option<CommonParams>) -> Endpoints {
        Endpoints::CommitteeReport { congress, report_type, report_number, text, parameters }
    }

    fn new_committee_print(congress: Option<i32>, chamber: Option<ChamberType>, jacket_number: Option<i32>, text: bool, parameters: Option<CommonParams>) -> Endpoints {
        Endpoints::CommitteePrint { congress, chamber, jacket_number, text, parameters }
    }

    fn new_committee_meeting(congress: Option<i32>, chamber: Option<ChamberType>, event_id: Option<String>, parameters: Option<CommonParams>) -> Endpoints {
        Endpoints::CommitteeMeeting { congress, chamber, event_id, parameters }
    }

    fn new_hearing(congress: Option<i32>, chamber: Option<ChamberType>, jacket_number: Option<i32>, parameters: Option<CommonParams>) -> Endpoints {
        Endpoints::Hearing { congress, chamber, jacket_number, parameters }
    }

    fn new_congressional_record(parameters: Option<CommonParams>) -> Endpoints {
        Endpoints::CongressionalRecord { parameters }
    }

    fn new_daily_congressional_record(volume_number: Option<i32>, issue_number: Option<i32>, articles: bool, parameters: Option<CommonParams>) -> Endpoints {
        Endpoints::DailyCongressionalRecord { volume_number, issue_number, articles, parameters }
    }

    fn new_bound_congressional_record(year: Option<i32>, month: Option<i32>, day: Option<i32>, parameters: Option<CommonParams>) -> Endpoints {
        Endpoints::BoundCongressionalRecord { year, month, day, parameters }
    }

    fn new_house_communication(congress: Option<i32>, communication_type: Option<CommunicationType>, communication_number: Option<String>, parameters: Option<CommonParams>) -> Endpoints {
        Endpoints::HouseCommunication { congress, communication_type, communication_number, parameters }
    }

    fn new_house_requirement(requirement_number: Option<i32>, matching_communications: bool, parameters: Option<CommonParams>) -> Endpoints {
        Endpoints::HouseRequirement { requirement_number, matching_communications, parameters }
    }

    fn new_senate_communication(congress: Option<i32>, communication_type: Option<CommunicationType>, communication_number: Option<String>, parameters: Option<CommonParams>) -> Endpoints {
        Endpoints::SenateCommunication { congress, communication_type, communication_number, parameters }
    }

    fn new_nomination(congress: Option<i32>, nomination_number: Option<String>, ordinal: Option<i32>, nomination_option: Option<NominationOption>, parameters: Option<CommonParams>) -> Endpoints {
        Endpoints::Nomination { congress, nomination_number, ordinal, nomination_option, parameters }
    }

    fn new_treaty(congress: Option<i32>, treaty_number: Option<String>, treaty_suffix: Option<String>, actions: bool, committees: bool, parameters: Option<CommonParams>) -> Endpoints {
        Endpoints::Treaty { congress, treaty_number, treaty_suffix, actions, committees, parameters }
    }
}


// Common parameters for all endpoints and models for the values of the parameters

#[derive(Debug, Serialize, Deserialize)]
pub struct CommonParams {
    pub format: Option<Format>,
    pub offset: Option<i32>,
    pub limit: Option<i32>,
    // Date format: YYYY-MM-DDTHH:MM:SSZ
    pub from_date_time: Option<String>,
    pub to_date_time: Option<String>,
    pub sort: Option<Sort>,
}

impl CommonParams {
    pub fn new() -> CommonParams {
        CommonParams {
            format: None,
            offset: None,
            limit: None,
            from_date_time: None,
            to_date_time: None,
            sort: None,
        }
    }

    pub fn to_string(&self) -> String {
        let mut params = Vec::new();
        if let Some(format) = &self.format {
            params.push(format.to_string());
        }
        if let Some(offset) = &self.offset {
            params.push(format!("offset={}", offset));
        }
        if let Some(limit) = &self.limit {
            params.push(format!("limit={}", limit));
        }
        if let Some(from_date_time) = &self.from_date_time {
            params.push(format!("fromDateTime={}", from_date_time));
        }
        if let Some(to_date_time) = &self.to_date_time {
            params.push(format!("toDateTime={}", to_date_time));
        }
        if let Some(sort) = &self.sort {
            params.push(sort.to_string());
        }
        params.join("&")
    }
}



// Values for the common parameters

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum Sort {
    DateAsc,
    DateDesc,
}

impl Sort {
    pub fn to_string(&self) -> String {
        match self {
            Sort::DateAsc => String::from("sort=updateDate+asc"),
            Sort::DateDesc => String::from("sort=updateDate+desc"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum Format {
    Json,
    Xml,
}

impl Format {
    pub fn to_string(&self) -> String {
        match self {
            Format::Json => String::from("format=json"),
            Format::Xml => String::from("format=xml"),
        }
    }
}





// Options for the endpoint values

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum BillOption {
    None,
    Actions,
    Amendments,
    Committees,
    Cosponsors,
    RelatedBills,
    Subjects,
    Summaries,
    Text,
    Titles
}

impl BillOption {
    pub fn to_string(&self) -> String {
        match self {
            BillOption::None => String::from(""),
            BillOption::Actions => String::from("actions"),
            BillOption::Amendments => String::from("amendments"),
            BillOption::Committees => String::from("committees"),
            BillOption::Cosponsors => String::from("cosponsors"),
            BillOption::RelatedBills => String::from("relatedBills"),
            BillOption::Subjects => String::from("subjects"),
            BillOption::Summaries => String::from("summaries"),
            BillOption::Text => String::from("text"),
            BillOption::Titles => String::from("titles"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum AmendmentOption {
    None,
    Actions,
    Cosponsors,
    Amendments,
    Text,
}

impl AmendmentOption {
    pub fn to_string(&self) -> String {
        match self {
            AmendmentOption::None => String::from(""),
            AmendmentOption::Actions => String::from("actions"),
            AmendmentOption::Cosponsors => String::from("cosponsors"),
            AmendmentOption::Amendments => String::from("amendments"),
            AmendmentOption::Text => String::from("text"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum BioGuideIdOption {
    None,
    SponsoredLegislation,
    CosponsoredLegislation,
}

impl BioGuideIdOption {
    pub fn to_string(&self) -> String {
        match self {
            BioGuideIdOption::None => String::from(""),
            BioGuideIdOption::SponsoredLegislation => String::from("sponsored"),
            BioGuideIdOption::CosponsoredLegislation => String::from("cosponsored"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum CommitteeOption {
    None,
    Bills,
    Reports,
    Nominations,
    HouseCommunication,
    SenateCommunication,
}

impl CommitteeOption {
    pub fn to_string(&self) -> String {
        match self {
            CommitteeOption::None => String::from(""),
            CommitteeOption::Bills => String::from("bills"),
            CommitteeOption::Reports => String::from("reports"),
            CommitteeOption::Nominations => String::from("nominations"),
            CommitteeOption::HouseCommunication => String::from("house-communication"),
            CommitteeOption::SenateCommunication => String::from("senate-communication"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum NominationOption {
    None,
    Actions,
    Committees,
    Hearings,
}

impl NominationOption {
    pub fn to_string(&self) -> String {
        match self {
            NominationOption::None => String::from(""),
            NominationOption::Actions => String::from("actions"),
            NominationOption::Committees => String::from("committees"),
            NominationOption::Hearings => String::from("hearings"),
        }
    }
}

// Types for the endpoint values

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum ChamberType {
    House,
    Senate,
    Joint,
}

impl ChamberType {
    pub fn to_string(&self) -> String {
        match self {
            ChamberType::House => String::from("house"),
            ChamberType::Senate => String::from("senate"),
            ChamberType::Joint => String::from("joint"),
        }
    }
}


#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum LawType {
    Public,
    Private,
}

impl LawType {
    pub fn to_string(&self) -> String {
        match self {
            LawType::Public => String::from("pub"),
            LawType::Private => String::from("priv"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum BillType {
    Hr, // House Resolution
    S, // Senate
    Hjres, // House Joint Resolution
    Sjres, // Senate Joint Resolution
    Hconres, // House Concurrent Resolution
    Sconres, // Senate Concurrent Resolution
    Hres, // House Simple Resolution
    Sres, // Senate Simple Resolution
}

impl BillType {
    pub fn to_string(&self) -> String {
        match self {
            BillType::Hr => String::from("hr"),
            BillType::S => String::from("s"),
            BillType::Hjres => String::from("hjres"),
            BillType::Sjres => String::from("sjres"),
            BillType::Hconres => String::from("hconres"),
            BillType::Sconres => String::from("sconres"),
            BillType::Hres => String::from("hres"),
            BillType::Sres => String::from("sres"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum AmendmentType {
    Hamdt, // House Amendment
    Samdt, // Senate Amendment
    Suamdt, // Senate Unnumbered Amendment
}

impl AmendmentType {
    pub fn to_string(&self) -> String {
        match self {
            AmendmentType::Hamdt => String::from("hamdt"),
            AmendmentType::Samdt => String::from("samdt"),
            AmendmentType::Suamdt => String::from("suamdt"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum CommunicationType {
    Ec, // Executive Communication
    Ml, // Message from the President
    Pm, // Presidential Message
    Pt, // Petition
}

impl CommunicationType {
    pub fn to_string(&self) -> String {
        match self {
            CommunicationType::Ec => String::from("ec"),
            CommunicationType::Ml => String::from("ml"),
            CommunicationType::Pm => String::from("pm"),
            CommunicationType::Pt => String::from("pt"),
        }
    }
}


