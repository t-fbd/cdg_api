use cdg_api::response_models::{BillsResponse, GenericResponse, serialize_response, parse_response};
use cdg_api::unwrap_option_string;

const RAW_BILL_DATA: &str = r#"{
    "bills": [
        {
            "congress": 117,
            "latestAction": {
                "actionDate": "2022-04-06",
                "text": "Became Public Law No: 117-108."
            },
            "number": "3076",
            "originChamber": "House",
            "originChamberCode": "H",
            "title": "Postal Service Reform Act of 2022",
            "type": "HR",
            "updateDate": "2022-09-29",
            "updateDateIncludingText": "2022-09-29T03:27:05Z",
            "url": "https://api.congress.gov/v3/bill/117/hr/3076?format=json"
        },
        {
            "congress": 117,
            "latestAction": {
                "actionDate": "2022-04-06",
                "text": "Read twice. Placed on Senate Legislative Calendar under General Orders. Calendar No. 343."
            },
            "number": "3599",
            "originChamber": "House",
            "originChamberCode": "H",
            "title": "Federal Rotational Cyber Workforce Program Act of 2021",
            "type": "HR",
            "updateDate": "2022-09-29",
            "updateDateIncludingText": "2022-09-29",
            "url": "https://api.congress.gov/v3/bill/117/hr/3599?format=json"
        }
    ]
}"#;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bills: BillsResponse = serde_json::from_str(RAW_BILL_DATA)?;
    
    for bill in bills.bills[..].iter() {
        println!("{}, {}, {} -- {}\n", 
            unwrap_option_string(bill.title.clone()),
            unwrap_option_string(bill.bill_type.clone()),
            unwrap_option_string(bill.number.clone()),
            unwrap_option_string(bill.url.clone())
        );
    }

    println!("Total bills: {}", bills.bills.len());

    println!("===============================");

    let bills: GenericResponse = serde_json::from_str(RAW_BILL_DATA)?;

    let bills: BillsResponse = match parse_response(&bills) {
        Ok(bills) => bills,
        Err(e) => {
            eprintln!("Error parsing generic response: {}", e);
            println!("{}", serialize_response(&bills, true)?);
            return Ok(());
        }
    };

    for bill in bills.bills[..].iter() {
        println!("{}, {}, {} -- {}\n", 
            unwrap_option_string(bill.title.clone()),
            unwrap_option_string(bill.bill_type.clone()),
            unwrap_option_string(bill.number.clone()),
            unwrap_option_string(bill.url.clone())
        );
    }

    println!("Total bills: {}", bills.bills.len());
    
    Ok(())
}
