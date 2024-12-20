use cdg_api::requests::endpoints::Endpoints::*;

fn main() {
    println!(
        "{}\n{}\n\n=====================",
        "Generic",
        Generic(String::from("example-endpoint"), Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "BillList",
        BillList(Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "BillByCongress",
        BillByCongress(117, Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "BillByType",
        BillByType(117, Default::default(), Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "BillDetails",
        BillDetails(117, Default::default(), 1234, Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "BillActions",
        BillActions(117, Default::default(), 1234, Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "BillAmendments",
        BillAmendments(117, Default::default(), 1234, Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "BillCommittees",
        BillCommittees(117, Default::default(), 1234, Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "BillCosponsors",
        BillCosponsors(117, Default::default(), 1234, Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "BillRelated",
        BillRelated(117, Default::default(), 1234, Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "BillSubjects",
        BillSubjects(117, Default::default(), 1234, Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "BillSummaries",
        BillSummaries(117, Default::default(), 1234, Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "BillText",
        BillText(117, Default::default(), 1234, Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "BillTitles",
        BillTitles(117, Default::default(), 1234, Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "LawByType",
        LawByType(117, Default::default(), Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "LawByCongress",
        LawByCongress(117, Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "LawDetails",
        LawDetails(117, Default::default(), 5678, Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "AmendmentList",
        AmendmentList(Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "AmendmentByCongress",
        AmendmentByCongress(117, Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "AmendmentByType",
        AmendmentByType(117, Default::default(), Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "AmendmentDetails",
        AmendmentDetails(117, Default::default(), 567, Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "AmendmentActions",
        AmendmentActions(
            117,
            Default::default(),
            String::from("567"),
            Default::default()
        )
    );
    println!(
        "{}\n{}\n\n=====================",
        "AmendmentCosponsors",
        AmendmentCosponsors(
            117,
            Default::default(),
            String::from("567"),
            Default::default()
        )
    );
    println!(
        "{}\n{}\n\n=====================",
        "AmendmentAmendments",
        AmendmentAmendments(
            117,
            Default::default(),
            String::from("567"),
            Default::default()
        )
    );
    println!(
        "{}\n{}\n\n=====================",
        "AmendmentText",
        AmendmentText(
            117,
            Default::default(),
            String::from("567"),
            Default::default()
        )
    );
    println!(
        "{}\n{}\n\n=====================",
        "SummariesList",
        SummariesList(Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "SummariesByCongress",
        SummariesByCongress(117, Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "SummariesByType",
        SummariesByType(117, Default::default(), Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "CongressList",
        CongressList(Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "CongressDetails",
        CongressDetails(117, Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "CongressCurrent",
        CongressCurrent(Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "MemberList",
        MemberList(Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "MemberByCongress",
        MemberByCongress(117, Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "MemberByState",
        MemberByState(String::from("NY"), Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "MemberByStateDistrict",
        MemberByStateDistrict(String::from("NY"), 10, Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "MemberByCongressStateDistrict",
        MemberByCongressStateDistrict(117, String::from("NY"), 10, Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "MemberDetails",
        MemberDetails(String::from("A000001"), Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "SponsorshipList",
        SponsorshipList(String::from("A000001"), Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "CosponsorshipList",
        CosponsorshipList(String::from("A000001"), Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "CommitteeList",
        CommitteeList(Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "CommitteeByChamber",
        CommitteeByChamber(Default::default(), Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "CommitteeByCongress",
        CommitteeByCongress(117, Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "CommitteeByCongressChamber",
        CommitteeByCongressChamber(117, Default::default(), Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "CommitteeDetails",
        CommitteeDetails(Default::default(), String::from("H01"), Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "CommitteeBills",
        CommitteeBills(Default::default(), String::from("H01"), Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "CommitteeReports",
        CommitteeReports(Default::default(), String::from("H01"), Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "CommitteeNominations",
        CommitteeNominations(Default::default(), String::from("H01"), Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "CommitteeHouseCommunication",
        CommitteeHouseCommunication(Default::default(), String::from("H01"), Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "CommitteeSenateCommunication",
        CommitteeSenateCommunication(Default::default(), String::from("H01"), Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "CommitteeReportList",
        CommitteeReportList(Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "CommitteeReportByCongress",
        CommitteeReportByCongress(117, Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "CommitteeReportByType",
        CommitteeReportByType(117, Default::default(), Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "CommitteeReportDetails",
        CommitteeReportDetails(117, Default::default(), 42, Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "CommitteeReportText",
        CommitteeReportText(117, Default::default(), 42, Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "CommitteePrintList",
        CommitteePrintList(Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "CommitteePrintByCongress",
        CommitteePrintByCongress(117, Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "CommitteePrintByCongressChamber",
        CommitteePrintByCongressChamber(117, Default::default(), Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "CommitteePrintByJacketNumber",
        CommitteePrintByJacketNumber(117, 789, Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "CommitteePrintText",
        CommitteePrintText(117, String::from("H01"), 789, Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "CommitteeMeetingList",
        CommitteeMeetingList(Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "CommitteeMeetingByCongress",
        CommitteeMeetingByCongress(117, Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "CommitteeMeetingByChamber",
        CommitteeMeetingByChamber(117, Default::default(), Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "CommitteeMeetingByEvent",
        CommitteeMeetingByEvent(
            117,
            Default::default(),
            String::from("EVT123"),
            Default::default()
        )
    );
    println!(
        "{}\n{}\n\n=====================",
        "HearingList",
        HearingList(Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "HearingByCongress",
        HearingByCongress(117, Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "HearingByChamber",
        HearingByChamber(117, Default::default(), Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "HearingByJacketNumber",
        HearingByJacketNumber(117, Default::default(), 100, Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "CongressionalRecordList",
        CongressionalRecordList(Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "DailyCongressionalRecordList",
        DailyCongressionalRecordList(Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "DailyCongressionalRecordVolume",
        DailyCongressionalRecordVolume(117, Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "DailyCongressionalRecordVolumeIssue",
        DailyCongressionalRecordVolumeIssue(117, 5, Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "DailyCongressionalRecordArticles",
        DailyCongressionalRecordArticles(117, 5, Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "BoundCongressionalRecordList",
        BoundCongressionalRecordList(Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "BoundCongressionalRecordByYear",
        BoundCongressionalRecordByYear(2021, Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "BoundCongressionalRecordByYearMonth",
        BoundCongressionalRecordByYearMonth(2021, 8, Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "BoundCongressionalRecordByYearMonthDay",
        BoundCongressionalRecordByYearMonthDay(2021, 8, 15, Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "HouseCommunicationList",
        HouseCommunicationList(Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "HouseCommunicationByCongress",
        HouseCommunicationByCongress(117, Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "HouseCommunicationByType",
        HouseCommunicationByType(117, Default::default(), Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "HouseCommunicationDetails",
        HouseCommunicationDetails(117, Default::default(), 345, Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "HouseRequirementList",
        HouseRequirementList(Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "HouseRequirementDetails",
        HouseRequirementDetails(567, Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "HouseRequirementMatching",
        HouseRequirementMatching(567, Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "SenateCommunicationList",
        SenateCommunicationList(Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "SenateCommunicationByCongress",
        SenateCommunicationByCongress(117, Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "SenateCommunicationByType",
        SenateCommunicationByType(117, Default::default(), Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "SenateCommunicationDetails",
        SenateCommunicationDetails(117, Default::default(), 345, Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "NominationList",
        NominationList(Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "NominationByCongress",
        NominationByCongress(117, Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "NominationDetails",
        NominationDetails(117, String::from("NOM123"), Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "Nominees",
        Nominees(117, String::from("NOM123"), 1, Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "NominationActions",
        NominationActions(117, String::from("NOM123"), Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "NominationCommittees",
        NominationCommittees(117, String::from("NOM123"), Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "NominationHearings",
        NominationHearings(117, String::from("NOM123"), Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "TreatyList",
        TreatyList(Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "TreatyByCongress",
        TreatyByCongress(117, Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "TreatyDetails",
        TreatyDetails(117, 345, Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "TreatyPartitioned",
        TreatyPartitioned(117, 345, String::from("suffix"), Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "TreatyCommittees",
        TreatyCommittees(117, 345, Default::default())
    );
    println!(
        "{}\n{}\n\n=====================",
        "TreatyActions",
        TreatyActions(117, 345, Default::default())
    );
}
