#!/usr/bin/env python3
"""Behavior checks for the local presentation controller;no engine model."""
import json
import unittest
from pathlib import Path
from play_match import Match, ROOT


class MatchTests(unittest.TestCase):
    def test_runtime_admits_only_the_initial_preset(self):
        self.assertEqual((ROOT/'demo/reference.json').read_bytes(),
                         (ROOT/'engine/tests/fixtures/full_match/admission.json').read_bytes())
        # Replay/expected streams are not runtime dependencies.
        source = (ROOT/'tools/play_match.py').read_text()
        self.assertNotIn('replay_', source)
        self.assertNotIn('fixtures/', source)

    def test_real_full_match_events_are_chronological_and_restartable(self):
        game = Match()
        game.action('start')
        self.assertEqual(game.state()['phase'], 'first-half')
        while game.state()['phase'] != 'full-time':
            if game.state()['phase'] == 'half-time':
                game.action('continue')
            game.action('step')
        done = game.state()
        self.assertEqual(done['steps'], 95)
        self.assertEqual(done['view']['score_side'], [1, 3])  # assertion only
        self.assertEqual([e['minute'] for e in done['events']], sorted(e['minute'] for e in done['events']))
        goals = [e for e in done['events'] if e['kind'] == 'goal']
        self.assertEqual([(e['player'], e['minute'], e['club']) for e in goals],
                         [(2590,2,130),(2590,9,130),(2669,40,134),(2591,90,130)])
        subs = [e for e in done['events'] if e['kind'] == 'substitution']
        self.assertEqual(len(subs), 2)
        self.assertTrue(all(e['incoming'] != e['outgoing'] for e in subs))
        final_ids = {p['player'] for p in done['view']['formation']}
        for sub in subs:
            self.assertIn(sub['incoming'], final_ids)
            self.assertNotIn(sub['outgoing'], final_ids)
        self.assertEqual(done['events'][-1]['kind'], 'full-time')
        self.assertEqual(len(done['view']['formation']),22)
        with self.assertRaises(ValueError): game.action('step')
        game.action('restart')
        self.assertEqual(game.state()['phase'], 'ready')
        self.assertEqual(game.state()['events'], [])
        self.assertEqual(game.state()['view']['score_side'], [0, 0])
        game.action('start')
        while game.state()['phase'] != 'full-time':
            if game.state()['phase'] == 'half-time': game.action('continue')
            game.action('step')
        self.assertEqual(game.state(), done)

    def test_supported_lineup_and_seed_choices_reach_real_full_time(self):
        states = {}
        for lineup in ('reference', 'reserve'):
            for seed in ('64', '122'):
                game=Match()
                game.configure({'lineup':lineup,'seed':seed})
                ready=game.state()
                ids={p['player'] for p in ready['view']['formation'] if p['club']==130}
                self.assertEqual(2593 in ids,lineup=='reserve')
                self.assertEqual(2590 in ids,lineup=='reference')
                prep=json.loads(game.session.preparation())
                self.assertEqual(prep['draws'][0]['seed_before'],int(seed))
                states[lineup,seed]=ready['view']['seed']
                game.action('start')
                while game.state()['phase']!='full-time':
                    if game.state()['phase']=='half-time':game.action('continue')
                    game.action('step')
                    self.assertLess(game.steps,200)
                self.assertEqual(game.events[-1]['kind'],'full-time')
                self.assertTrue(any(e['kind']=='clock' for e in game.events))
                self.assertEqual(len(game.state()['view']['formation']),22)
                game.action('restart')
                self.assertEqual(game.state(),ready)
        self.assertNotEqual(states['reference','64'],states['reference','122'])

    def test_configuration_is_atomic_and_only_before_kickoff(self):
        game=Match()
        for invalid in ({'lineup':'bad','seed':'64'}, {'lineup':'reference','seed':'0'},
                        {'lineup':'reference','seed':64}, {'seed':'64'},
                        {'lineup':'reference','seed':'64','expected':{}},None):
            before=game.state()
            with self.assertRaises(ValueError):game.configure(invalid)
            self.assertEqual(game.state(),before)
        game.configure({'lineup':'reserve','seed':'122'})
        game.action('start');before=game.state()
        with self.assertRaises(ValueError):game.configure({'lineup':'reference','seed':'64'})
        self.assertEqual(game.state(),before)

    def test_phase_guards_keep_current_state(self):
        game = Match()
        for action in ('step', 'continue', 'bad'):
            before = game.state()
            with self.assertRaises(ValueError): game.action(action)
            self.assertEqual(game.state(), before)
        game.action('start')
        with self.assertRaises(ValueError): game.action('start')
        while game.state()['phase'] != 'half-time': game.action('step')
        before = game.state()
        with self.assertRaises(ValueError): game.action('step')
        self.assertEqual(game.state(), before)
        game.action('continue')
        self.assertEqual(game.state()['phase'], 'second-half')
        with self.assertRaises(ValueError): game.action('continue')


if __name__ == '__main__':
    unittest.main()
