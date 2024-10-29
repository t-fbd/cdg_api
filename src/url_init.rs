use crate::{BASE_URL, Endpoints};

impl Endpoints {
    pub fn to_url(&self) -> String {
        let mut url = BASE_URL.to_string();
        let api_key = std::env::var("CDG_API_KEY").unwrap();
        match self {
            Endpoints::Bill { congress, bill_type, bill_number, bill_option, parameters } => {
                url.push_str("bills");
                let mut flag = 0;

                if let Some(congress) = congress {
                    url.push_str(&format!("/{}", congress));
                }

                if let Some(bill_type) = bill_type {
                    if flag == 1 {
                        url.push_str(&format!("/{}", bill_type.to_string()));
                        flag += 1;
                    } 
                }

                if let Some(bill_number) = bill_number {
                    if flag == 2 {
                        url.push_str(&format!("/{}", bill_number));
                        flag += 1;
                    }
                }

                if let Some(bill_option) = bill_option {
                    if flag == 3 {
                        url.push_str(&format!("/{}", bill_option.to_string()));
                    }
                }

                if let Some(parameters) = parameters {
                    url.push_str(&format!("?{}", parameters.to_string()));
                }

                return format!("{}&api_key={}", url, api_key);
            },
            Endpoints::Law { congress, law_type, law_number, parameters } => {
                url.push_str("laws");
                url.push_str(&format!("/{}", congress));
                let mut flag = 0;

                if let Some(law_type) = law_type {
                    url.push_str(&format!("/{}", law_type.to_string()));
                    flag += 1;
                }

                if let Some(law_number) = law_number {
                    if flag == 1 {
                        url.push_str(&format!("/{}", law_number));
                    }
                }

                if let Some(parameters) = parameters {
                    url.push_str(&format!("?{}", parameters.to_string()));
                }

                return format!("{}&api_key={}", url, api_key);

            },
            Endpoints::Amendment { congress, amendment_type, amendment_number, amendment_option, parameters } => {
                url.push_str("amendments");
                let mut flag = 0;

                if let Some(congress) = congress {
                    url.push_str(&format!("/{}", congress));
                    flag += 1;
                }

                if let Some(amendment_type) = amendment_type {
                    if flag == 1 {
                        url.push_str(&format!("/{}", amendment_type.to_string()));
                        flag += 1;
                    }
                }

                if let Some(amendment_number) = amendment_number {
                    if flag == 2 {
                        url.push_str(&format!("/{}", amendment_number));
                        flag += 1;
                    }
                }

                if let Some(amendment_option) = amendment_option {
                    if flag == 3 {
                        url.push_str(&format!("/{}", amendment_option.to_string()));
                    }
                }

                if let Some(parameters) = parameters {
                    url.push_str(&format!("?{}", parameters.to_string()));
                }

                return format!("{}&api_key={}", url, api_key);
            },
            Endpoints::Summaries { congress, bill_type, parameters } => {
                url.push_str("summaries");
                let mut flag = 0;

                if let Some(congress) = congress {
                    url.push_str(&format!("/{}", congress));
                    flag += 1;
                }

                if let Some(bill_type) = bill_type {
                    if flag == 1 {
                        url.push_str(&format!("/{}", bill_type.to_string()));
                    }
                }

                if let Some(parameters) = parameters {
                    url.push_str(&format!("?{}", parameters.to_string()));
                }

                return format!("{}&api_key={}", url, api_key);
            },
            Endpoints::Congress { congress, current, parameters } => {
                url.push_str("congress");

                if congress.is_some() && current.is_none() {
                    url.push_str(&format!("/{}", congress.unwrap()));
                } else if current.is_some() && congress.is_none() {
                    url.push_str("/current");
                }

                if let Some(parameters) = parameters {
                    url.push_str(&format!("?{}", parameters.to_string()));
                }

                return format!("{}&api_key={}", url, api_key);
            },
            Endpoints::Member { bio_guide_id, bio_guide_id_option, congress, state_code, district, parameters } => {
                url.push_str("member");
                let bio_guide_id_flag = bio_guide_id.is_some();
                let state_code_flag = state_code.is_some();
                let district_flag = district.is_some();
                let congress_flag = congress.is_some();

                if bio_guide_id_flag {
                    url.push_str(&format!("/{}", bio_guide_id.clone().unwrap()));
                    if let Some(bio_guide_id_option) = bio_guide_id_option {
                        url.push_str(&format!("/{}", bio_guide_id_option.to_string()));
                    }
                } else {
                    if congress_flag && !state_code_flag {
                        url.push_str(&format!("/congress/{}", congress.unwrap()));
                    } else if congress_flag && state_code_flag && district_flag {
                        url.push_str(&format!("/congress/{}/{}", congress.unwrap(), state_code.clone().unwrap()));
                        url.push_str(&format!("/{}", district.unwrap()));
                    } else if state_code_flag && district_flag {
                        url.push_str(&format!("/{}", state_code.clone().unwrap()));
                        url.push_str(&format!("/{}", district.unwrap()));
                    } else if state_code_flag {
                        url.push_str(&format!("/{}", state_code.clone().unwrap()));
                    }
                    
                }

                if let Some(parameters) = parameters {
                    url.push_str(&format!("?{}", parameters.to_string()));
                }

                return format!("{}&api_key={}", url, api_key);
            },
            Endpoints::Committee { chamber, congress, committee_code, committee_option, parameters } => {
                url.push_str("committees");
                let chamber_flag = chamber.is_some();
                let congress_flag = congress.is_some();

                if chamber_flag && congress_flag {
                    url.push_str(&format!("/{}", congress.unwrap()));
                    url.push_str(&format!("/{}", chamber.unwrap().to_string()));
                } else if chamber_flag && !committee_code.is_some() {
                    url.push_str(&format!("/{}", chamber.unwrap().to_string()));
                } else if congress_flag && !chamber_flag {
                    url.push_str(&format!("/{}", congress.unwrap()));
                } else if chamber_flag && committee_code.is_some() {
                    url.push_str(&format!("/{}", chamber.unwrap().to_string()));
                    url.push_str(&format!("/{}", committee_code.clone().unwrap()));
                    if let Some(committee_option) = committee_option {
                        url.push_str(&format!("/{}", committee_option.to_string()));
                    }
                }

                if let Some(parameters) = parameters {
                    url.push_str(&format!("?{}", parameters.to_string()));
                }

                return format!("{}&api_key={}", url, api_key);
            },
            Endpoints::CommitteeReport { congress, report_type, report_number, text, parameters } => {
                url.push_str("committee-reports");
                let mut flag = 0;
                let congress_flag = congress.is_some();

                if congress_flag {
                    url.push_str(&format!("/{}", congress.unwrap()));
                    flag += 1;
                }

                if let Some(report_type) = report_type {
                    if flag == 1 {
                        url.push_str(&format!("/{}", report_type));
                        flag += 1;
                    }
                }

                if let Some(report_number) = report_number {
                    if flag == 2 {
                        url.push_str(&format!("/{}", report_number));
                        flag += 1;
                    }
                }

                if *text {
                    if flag == 3 {
                        url.push_str("/text");
                    }
                }

                if let Some(parameters) = parameters {
                    url.push_str(&format!("?{}", parameters.to_string()));
                }

                return format!("{}&api_key={}", url, api_key);
            },
            Endpoints::CommitteePrint { congress, chamber, jacket_number, text, parameters } => {
                url.push_str("committee_prints");
                let mut flag = 0;
                let congress_flag = congress.is_some();
                let chamber_flag = chamber.is_some();

                if congress_flag {
                    url.push_str(&format!("/{}", congress.unwrap()));
                    flag += 1;
                }

                if chamber_flag {
                    if flag == 1 {
                        url.push_str(&format!("/{}", chamber.unwrap().to_string()));
                        flag += 1;
                    }
                }

                if let Some(jacket_number) = jacket_number {
                    if flag == 2 {
                        url.push_str(&format!("/{}", jacket_number));
                        flag += 1;
                    }
                }

                if *text {
                    if flag == 3 {
                        url.push_str("/text");
                    }
                }

                if let Some(parameters) = parameters {
                    url.push_str(&format!("?{}", parameters.to_string()));
                }

                return format!("{}&api_key={}", url, api_key);
            },
            Endpoints::CommitteeMeeting { congress, chamber, event_id, parameters } => {
                url.push_str("committee-meetings");
                let mut flag = 0;
                let congress_flag = congress.is_some();
                let chamber_flag = chamber.is_some();

                if congress_flag {
                    url.push_str(&format!("/{}", congress.unwrap()));
                    flag += 1;
                }

                if chamber_flag {
                    if flag == 1 {
                        url.push_str(&format!("/{}", chamber.unwrap().to_string()));
                        flag += 1;
                    }
                }

                if let Some(event_id) = event_id {
                    if flag == 2 {
                        url.push_str(&format!("/{}", event_id));
                    }
                }

                if let Some(parameters) = parameters {
                    url.push_str(&format!("?{}", parameters.to_string()));
                }

                return format!("{}&api_key={}", url, api_key);
            },
            Endpoints::Hearing { congress, chamber, jacket_number, parameters } => {
                url.push_str("hearings");
                let mut flag = 0;
                let congress_flag = congress.is_some();
                let chamber_flag = chamber.is_some();

                if congress_flag {
                    url.push_str(&format!("/{}", congress.unwrap()));
                    flag += 1;
                }

                if chamber_flag {
                    if flag == 1 {
                        url.push_str(&format!("/{}", chamber.unwrap().to_string()));
                        flag += 1;
                    }
                }

                if let Some(jacket_number) = jacket_number {
                    if flag == 2 {
                        url.push_str(&format!("/{}", jacket_number));
                    }
                }

                if let Some(parameters) = parameters {
                    url.push_str(&format!("?{}", parameters.to_string()));
                }

                return format!("{}&api_key={}", url, api_key);
            },
            Endpoints::CongressionalRecord { parameters } => {
                url.push_str("congressional-record");
                if let Some(parameters) = parameters {
                    url.push_str(&format!("?{}", parameters.to_string()));
                }
                return format!("{}&api_key={}", url, api_key);
            },
            Endpoints::DailyCongressionalRecord { volume_number, issue_number, articles, parameters } => {
                url.push_str("daily-congressional-record");
                let mut flag = 0;

                if volume_number.is_some() {
                    url.push_str(&format!("/{}", volume_number.unwrap()));
                    flag += 1;
                }

                if issue_number.is_some() {
                    if flag == 1 {
                        url.push_str(&format!("/{}", issue_number.unwrap()));
                        flag += 1;
                    }
                }

                if *articles {
                    if flag == 2 {
                        url.push_str("/articles");
                    }
                }

                if let Some(parameters) = parameters {
                    url.push_str(&format!("?{}", parameters.to_string()));
                }

                return format!("{}&api_key={}", url, api_key);
            },
            Endpoints::BoundCongressionalRecord { year, month, day, parameters } => {
                url.push_str("bound-congressional-record");
                let mut flag = 0;

                if year.is_some() {
                    url.push_str(&format!("/{}", year.unwrap()));
                    flag += 1;
                }

                if month.is_some() {
                    if flag == 1 {
                        url.push_str(&format!("/{}", month.unwrap()));
                        flag += 1;
                    }
                }

                if day.is_some() {
                    if flag == 2 {
                        url.push_str(&format!("/{}", day.unwrap()));
                    }
                }

                if let Some(parameters) = parameters {
                    url.push_str(&format!("?{}", parameters.to_string()));
                }

                return format!("{}&api_key={}", url, api_key);
            },
            Endpoints::HouseCommunication { congress, communication_type, communication_number, parameters } => {
                url.push_str("house-communications");
                let mut flag = 0;
                let congress_flag = congress.is_some();

                if congress_flag {
                    url.push_str(&format!("/{}", congress.unwrap()));
                    flag += 1;
                }

                if let Some(communication_type) = communication_type {
                    if flag == 1 {
                        url.push_str(&format!("/{}", communication_type.to_string()));
                        flag += 1;
                    }
                }

                if let Some(communication_number) = communication_number {
                    if flag == 2 {
                        url.push_str(&format!("/{}", communication_number));
                    }
                }

                if let Some(parameters) = parameters {
                    url.push_str(&format!("?{}", parameters.to_string()));
                }

                return format!("{}&api_key={}", url, api_key);
            },
            Endpoints::HouseRequirement { requirement_number, matching_communications, parameters } => {
                url.push_str("house-requirements");
                let mut flag = 0;

                if requirement_number.is_some() {
                    url.push_str(&format!("/{}", requirement_number.unwrap()));
                    flag += 1;
                }

                if *matching_communications {
                    if flag == 1 {
                        url.push_str("/matching-communications");
                    }
                }

                if let Some(parameters) = parameters {
                    url.push_str(&format!("?{}", parameters.to_string()));
                }

                return format!("{}&api_key={}", url, api_key);
            },
            Endpoints::SenateCommunication { congress, communication_type, communication_number, parameters } => {
                url.push_str("senate-communications");
                let mut flag = 0;
                let congress_flag = congress.is_some();

                if congress_flag {
                    url.push_str(&format!("/{}", congress.unwrap()));
                    flag += 1;
                }

                if let Some(communication_type) = communication_type {
                    if flag == 1 {
                        url.push_str(&format!("/{}", communication_type.to_string()));
                        flag += 1;
                    }
                }

                if let Some(communication_number) = communication_number {
                    if flag == 2 {
                        url.push_str(&format!("/{}", communication_number));
                    }
                }

                if let Some(parameters) = parameters {
                    url.push_str(&format!("?{}", parameters.to_string()));
                }

                return format!("{}&api_key={}", url, api_key);
            },
            Endpoints::Nomination { congress, nomination_number, ordinal, nomination_option, parameters } => {
                url.push_str("nominations");
                let mut flag = 0;
                let ord_flag = ordinal.is_some();
                let nomination_option = {
                    if ord_flag {
                        None
                    } else {
                        Some(nomination_option.unwrap())
                    }
                };

                if congress.is_some() {
                    url.push_str(&format!("/{}", congress.unwrap()));
                    flag += 1;
                }

                if let Some(nomination_number) = nomination_number {
                    if flag == 1 {
                        url.push_str(&format!("/{}", nomination_number));
                        flag += 1;
                    }
                }

                if ord_flag {
                    if flag == 2 {
                        url.push_str(&format!("/{}", ordinal.unwrap()));
                    }
                } else if let Some(nomination_option) = nomination_option {
                    if flag == 2 {
                        url.push_str(&format!("/{}", nomination_option.to_string()));
                    }
                }

                if let Some(parameters) = parameters {
                    url.push_str(&format!("?{}", parameters.to_string()));
                }

                return format!("{}&api_key={}", url, api_key);
            },
            Endpoints::Treaty { congress, treaty_number, treaty_suffix, actions, committees, parameters } => {
                url.push_str("treaties");
                let congress_flag = congress.is_some();

                if congress_flag {
                    url.push_str(&format!("/{}", congress.unwrap()));
                }

                if let Some(treaty_number) = treaty_number {
                    url.push_str(&format!("/{}", treaty_number));

                    if treaty_suffix.is_some() && *actions {
                        url.push_str(&format!("/{}", treaty_suffix.clone().unwrap()));
                        url.push_str("/actions");
                    } else if treaty_suffix.is_some() && !*actions {
                        url.push_str(&format!("/{}", treaty_suffix.clone().unwrap()));
                    } else if *actions {
                        url.push_str("/actions");
                    } else if *committees {
                        url.push_str("/committees");
                    }
                }
                
                if let Some(parameters) = parameters {
                    url.push_str(&format!("?{}", parameters.to_string()));
                }

                return format!("{}&api_key={}", url, api_key);
            }
        };
    }
}



