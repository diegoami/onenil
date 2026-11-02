//! [designed] T24 cross-slice contracts;these expose real pre-migration disagreements.
use crate::{
    extra_event, neutral_input, rearrangement, role_scores, scene_assignment, step_tail, Engine,
    Input,
};
use serde_json::Value;
fn old() -> extra_event::State {
    let c: Value = serde_json::from_str(
        include_str!("../tests/fixtures/extra_event/T17-r0006.jsonl")
            .lines()
            .next()
            .unwrap(),
    )
    .unwrap();
    serde_json::from_value(c["input"].clone()).unwrap()
}
fn assignment() -> scene_assignment::State {
    let c: Value = serde_json::from_str(
        include_str!("../tests/fixtures/scene_assignment/cases.jsonl")
            .lines()
            .next()
            .unwrap(),
    )
    .unwrap();
    serde_json::from_value(c["input"].clone()).unwrap()
}
fn parent() -> rearrangement::State {
    let c: Value = serde_json::from_str(
        include_str!("../tests/fixtures/rearrangement/cases.jsonl")
            .lines()
            .next()
            .unwrap(),
    )
    .unwrap();
    serde_json::from_value(c["input"].clone()).unwrap()
}
fn blank() -> Engine {
    Engine::new(&Input {
        k: 0,
        seed: 0x1234,
        ratings: [0; 8],
        goals: [0; 2],
    })
}
#[test]
fn shared_image_legacy_player_load_is_visible_to_new_assignment_snapshot() {
    let old = old();
    let p = old
        .players
        .iter()
        .find(|p| p.id != 0 && p.p8_7 != 0)
        .unwrap();
    let mut view = assignment();
    view.players.retain(|q| q.id == p.id);
    assert_eq!(view.players.len(), 1);
    let mut e = blank();
    extra_event::load_state(&mut e, &old, false).unwrap();
    let got = scene_assignment::snapshot(&e, &view);
    assert_eq!(got.players[0].style, p.p8_7);
    assert_eq!(got.players[0].club, p.p8_0);
}
#[test]
fn shared_image_new_player_load_is_visible_to_legacy_extra_event_projection() {
    let new = assignment();
    let p = new
        .players
        .iter()
        .find(|p| p.id != 0 && p.style != 0)
        .unwrap();
    let e = role_scores::image(scene_assignment::cells(&new).unwrap());
    let got = extra_event::project(&e, &old());
    let q = got.players.iter().find(|q| q.id == p.id).unwrap();
    assert_eq!(q.p8_7, p.style);
    assert_eq!(q.p8_0, p.club);
}
#[test]
fn shared_image_new_club_is_visible_to_legacy_extra_event_projection() {
    let mut new = parent();
    let id = new.pairing.clubs[0].id;
    new.pairing.clubs[0].byte57 = 0xA5;
    let e = role_scores::image(rearrangement::cells(&new).unwrap());
    let got = extra_event::project(&e, &old());
    assert_eq!(got.clubs.iter().find(|c| c.id == id).unwrap().byte57, 0xA5);
}
#[test]
fn shared_image_legacy_club_load_is_visible_to_new_pairing_snapshot() {
    let mut new = parent();
    let id = new.pairing.clubs[0].id;
    new.pairing.clubs[0].byte57 = 0;
    let mut old = old();
    old.clubs.iter_mut().find(|c| c.id == id).unwrap().byte57 = 0xA5;
    let mut e = role_scores::image(rearrangement::cells(&new).unwrap());
    extra_event::load_state(&mut e, &old, false).unwrap();
    let got = rearrangement::snapshot(&mut e, &new);
    assert_eq!(
        got.pairing
            .clubs
            .iter()
            .find(|c| c.id == id)
            .unwrap()
            .byte57,
        0xA5
    );
}
#[test]
fn shared_image_prefix_history_write_preserves_canonical_player_bytes() {
    let c: Value = serde_json::from_str(
        include_str!("../tests/fixtures/step_tail/T16-r0006.jsonl")
            .lines()
            .next()
            .unwrap(),
    )
    .unwrap();
    let mut s: step_tail::PrefixInput =
        serde_json::from_value(c["prefix"]["input"].clone()).unwrap();
    s.clubs = [1, 2];
    s.history.base_offset = 8;
    s.history.selector = 1;
    s.history.value = 0xAB;
    let mut e = Engine::new(&Input {
        k: 0,
        seed: s.placement.seed,
        ratings: [0; 8],
        goals: s.placement.generated,
    });
    e.image[0xD09F] = s.placement.actor;
    e.image[0x10000 + 8] = 0xAB;
    let out = e.finish_prefix(&s, &s.placement).unwrap();
    assert_eq!(out.output.history_offset, 8);
    assert_eq!(
        e.image[0x10000 + 8],
        0xAB,
        "history pool overlaps canonicalP8 player1"
    );
}
#[test]
fn shared_image_neutral_speed_reads_the_config_object_used_by_prefix() {
    let mut e = blank();
    e.image[0x49002] = 37;
    e.image[0x40081] = 5;
    let out = neutral_input::neutral_image(&mut e);
    assert_eq!(out.output.speed, 37);
    assert_eq!(e.image[0x40081], 5);
}

fn bonus(c: &scene_assignment::Count) -> crate::ratings::BonusInput {
    crate::ratings::BonusInput {
        club: c.id,
        staff_ids: [c.index40, c.index42],
        staff: vec![
            crate::ratings::Staff {
                id: c.index40,
                byte2: c.cell40,
            },
            crate::ratings::Staff {
                id: c.index42,
                byte2: c.cell42,
            },
        ],
    }
}
#[test]
fn shared_staff_count_cells_feed_the_existing_rating_bonus_reader() {
    let new = assignment();
    let c = &new.club_counts[0];
    let expected = crate::ratings::club_bonus(&bonus(c)).unwrap().bonus;
    let e = role_scores::image(scene_assignment::cells(&new).unwrap());
    assert_eq!(e.rating_bonus(c.id), expected);
}
#[test]
fn shared_staff_legacy_loader_updates_assignment_count_cells() {
    let mut new = assignment();
    new.club_counts[0].cell40 = 0;
    new.club_counts[0].cell42 = 0;
    let mut b = bonus(&new.club_counts[0]);
    b.staff[0].byte2 = 0xA5;
    b.staff[1].byte2 = 0x5A;
    let mut e = role_scores::image(scene_assignment::cells(&new).unwrap());
    crate::ratings::load_bonus(&mut e, &b);
    let got = scene_assignment::snapshot(&e, &new);
    assert_eq!(got.club_counts[0].cell40, 0xA5);
    assert_eq!(got.club_counts[0].cell42, 0x5A);
}
#[test]
fn shared_staff_count_high_index_bits_alias_the_same_original_word_shifted_entry() {
    let mut new = assignment();
    let expected = crate::ratings::club_bonus(&bonus(&new.club_counts[0]))
        .unwrap()
        .bonus;
    new.club_counts[0].index40 += 0x4000;
    new.club_counts[0].index42 += 0x4000;
    let e = role_scores::image(scene_assignment::cells(&new).unwrap());
    assert_eq!(e.rating_bonus(new.club_counts[0].id), expected);
}
