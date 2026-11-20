//! [confirmed:native #204 probe at 10db6ff] Saved campaign images the result and
//! week APIs cannot produce must refuse at decode. [confirmed:archaeology 788b833
//! T93] The witnessed Sevilla result is `[1, 1]` with 18 fixed conditions; the
//! completed week advances the revision once past the second recorded result.
use onenil_campaign::Campaign;

const FIRST_SCORE: [u16; 2] = [1, 4];
const SEVILLA_SCORE: [u16; 2] = [1, 1];

fn witnessed_conditions() -> Vec<(u16, i8)> {
    let v: serde_json::Value =
        serde_json::from_str(include_str!("../fixtures/sevilla-post-match.json")).unwrap();
    v["witnessed"]["conditions"]
        .as_array()
        .unwrap()
        .iter()
        .map(|row| {
            (
                row["player"].as_u64().unwrap() as u16,
                row["condition_after"].as_i64().unwrap() as i8,
            )
        })
        .collect()
}

/// The legitimate week-one context after the friendly week completes, with the
/// first result recorded and the Sevilla fixture advertised.
fn week_one() -> Campaign {
    let mut c = Campaign::initial().unwrap();
    let ticket = c.match_ticket().unwrap();
    c.record_match(&ticket, FIRST_SCORE, "0AB9:EBE2").unwrap();
    c.record_week_completion(38_761, 69_270, 1, 176, 0, "Sevilla")
        .unwrap();
    c
}

/// The legitimate week-one context holding the recorded Sevilla result.
fn sevilla_recorded() -> Campaign {
    let mut c = week_one();
    let ticket = c.match_ticket().unwrap();
    c.record_sevilla_result(&ticket, SEVILLA_SCORE, "0AB9:EBE2", &witnessed_conditions())
        .unwrap();
    c
}

/// The legitimate completed week-two context.
fn week_two() -> Campaign {
    let mut c = sevilla_recorded();
    c.record_week_completion(69_410, 69_270, 1, 28, 0, "Celtic")
        .unwrap();
    c
}

/// Every forged image must be refused by `validate()` and by the real load path,
/// which leaves the live campaign untouched.
fn refuses(name: &str, forged: &Campaign) {
    assert!(forged.validate().is_err(), "{name} accepted by validate()");
    let live = if forged.week_index == 1 {
        sevilla_recorded()
    } else {
        week_two()
    };
    let mut target = live.clone();
    assert!(
        target
            .replace_from_bytes(&serde_json::to_vec(forged).unwrap())
            .is_err(),
        "{name} accepted through a save image"
    );
    assert_eq!(target, live, "{name} replaced the live campaign");
}

#[test]
fn a_forged_sevilla_score_refuses_before_it_can_complete_the_week() {
    // [confirmed:#204 probe] A loaded `[1, 4]` second result then completed the
    // week transaction: the score was never bound to the witnessed game.
    let mut forged = sevilla_recorded();
    forged.second_fixture.as_mut().unwrap().score = [1, 4];
    refuses("forged score", &forged);
    let refused = Campaign::decode(&serde_json::to_vec(&forged).unwrap());
    assert!(refused.is_err());
}

#[test]
fn edited_post_sevilla_conditions_refuse_in_week_one() {
    // The witnessed 18 outcomes are set by the caller and preserved by the week
    // route; no API edits them afterwards.
    let mut forged = sevilla_recorded();
    forged.players[0].availability.condition_signed ^= 1;
    refuses("edited condition", &forged);
}

#[test]
fn a_first_result_ordered_after_the_second_refuses() {
    // The friendly week completes between the two recorded results, so the
    // second request revision must strictly follow the first.
    let mut forged = sevilla_recorded();
    let second = forged.second_fixture.as_ref().unwrap().request_revision;
    forged.played_fixture.as_mut().unwrap().request_revision = second;
    refuses("equal revisions", &forged);
    let mut forged = sevilla_recorded();
    let second = forged.second_fixture.as_ref().unwrap().request_revision;
    forged.played_fixture.as_mut().unwrap().request_revision = second + 3;
    refuses("reversed revisions", &forged);
}

#[test]
fn a_week_one_context_without_its_first_result_refuses() {
    // The week-one cash and following fixture are only reachable through the
    // week completion, which requires the recorded first result.
    let mut forged = week_one();
    forged.played_fixture = None;
    refuses("missing first result", &forged);
}

#[test]
fn a_week_two_context_missing_the_week_transaction_revision_refuses() {
    // The completed week increments the revision once past the second result.
    let mut forged = week_two();
    let second = forged.second_fixture.as_ref().unwrap().request_revision;
    forged.revision = second + 1;
    refuses("missing week revision", &forged);
}

#[test]
fn legitimate_edited_histories_and_higher_revisions_still_load() {
    // Edits between the two matches are allowed when undone before Sevilla; the
    // revision and the request revisions carry the history, not equality.
    let mut c = Campaign::initial().unwrap();
    let original_instructions = c.players[0].individual_instructions;
    c.set_instructions(c.players[0].id, original_instructions ^ 0x01)
        .unwrap();
    let edited_revision = c.revision;
    let ticket = c.match_ticket().unwrap();
    c.record_match(&ticket, FIRST_SCORE, "0AB9:EBE2").unwrap();
    c.set_instructions(c.players[0].id, original_instructions)
        .unwrap();
    c.record_week_completion(38_761, 69_270, 1, 176, 0, "Sevilla")
        .unwrap();
    let ticket = c.match_ticket().unwrap();
    c.record_sevilla_result(&ticket, SEVILLA_SCORE, "0AB9:EBE2", &witnessed_conditions())
        .unwrap();
    c.record_week_completion(69_410, 69_270, 1, 28, 0, "Celtic")
        .unwrap();
    assert_eq!(
        c.played_fixture.as_ref().unwrap().request_revision,
        edited_revision
    );
    assert_eq!(c.week_index, 2);
    let reloaded = Campaign::decode(&c.encode().unwrap()).unwrap();
    assert_eq!(reloaded, c);

    // A later edit raises the revision above the week transaction's increment and
    // an earlier edit gap stays legal: only the lower bound is bound.
    let mut raised = c.clone();
    raised.set_position(raised.players[0].id, raised.players[0].current_position).unwrap();
    if raised.revision == c.revision {
        raised.set_instructions(raised.players[0].id, 0x02).unwrap();
    }
    assert!(raised.revision > c.revision);
    assert!(raised.validate().is_ok());
    assert_eq!(Campaign::decode(&raised.encode().unwrap()).unwrap(), raised);
    let mut gapped = week_two();
    gapped.revision += 5;
    assert!(gapped.validate().is_ok());
    assert_eq!(Campaign::decode(&gapped.encode().unwrap()).unwrap(), gapped);
}
