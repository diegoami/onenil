"""[designed] Detailed native action semantics and full-match controller acceptance."""
import json
import unittest
from unittest.mock import patch
from play_watched import describe, Match, ROOT

class ActionSemantics(unittest.TestCase):
    def event(self, code):
        return dict(code=code,player=2591,secondary=2590,side=0,club=130,minute=3,tick=121)
    def test_named_actions_preserve_original_marker_roles(self):
        names={2591:'Shearer',2590:'Newell'}
        for code,kind,name in [(1,'pass','Shearer'),(8,'receive','Newell'),(11,'tackle','Shearer'),(17,'shot','Shearer'),(38,'goal','Shearer')]:
            event=describe(self.event(code),names)
            self.assertEqual(event['kind'],kind)
            self.assertIn(name,event['text'])
            self.assertNotIn('Play continues',event['text'])
        self.assertEqual(describe(self.event(0),names)['text'],'Shearer replaces Newell.')
    def test_original_28_to_31_are_possession_save_block_deflection(self):
        for code,kind in [(28,'possession'),(29,'save'),(30,'block'),(31,'deflection')]:
            self.assertEqual(describe(self.event(code),{2591:'Shearer'})['kind'],kind)
        self.assertIn('wins', describe(self.event(59),{2591:'Shearer'})['text'])
    def test_unknown_codes_and_missing_named_players_fail_explicitly(self):
        with self.assertRaises(ValueError):describe(self.event(255),{2591:'Shearer'})
        with self.assertRaises(ValueError):describe(self.event(8),{2591:'Shearer'})


class NativeController(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        import onenil_engine as native
        original = native.watched_match
        with patch.object(native, 'watched_match', wraps=original) as compute:
            cls.game = Match()
            cls.game.restart()
            if compute.call_count != 1:
                raise AssertionError('viewer did not compute exactly one native result for its playback cache')

    def setUp(self):
        self.game.restart()

    def test_complete_native_actions_positions_scores_and_halftime(self):
        game = self.game
        self.assertEqual(game.state()['view']['score'], [0, 0])
        game.action('start')
        halves = 0
        for _ in range(600):
            if game.phase == 'full-time':break
            if game.phase == 'half-time':
                halves += 1
                with self.assertRaises(ValueError):game.action('step')
                game.action('continue')
            game.action('step')
        self.assertEqual((game.phase, game.cursor, halves), ('full-time', 4199, 1))
        self.assertEqual(game.view['score'], [2, 1])
        self.assertEqual(game.view['clock'], 93)
        self.assertEqual(len(game.events), 831)
        original = []
        for path in sorted((ROOT/'records/T41/reference').glob('chunk-*.json')):
            original.extend(json.loads(path.read_text())['payload']['actions'])
        fields = ('code', 'player', 'secondary', 'club', 'side', 'tick', 'minute')
        self.assertEqual([tuple(e[k] for k in fields) for e in game.events],
                         [tuple(e[k] for k in fields) for e in original])
        self.assertTrue({'pass','receive','tackle','shot','save','goal','substitution'} <= {e['kind'] for e in game.events})
        self.assertTrue(all('Play continues' not in e['text'] for e in game.events))
        self.assertTrue(all(len(f['view']['players']) == 22 for f in game.frames))
        self.assertTrue(all(e['minute'] <= f['view']['clock'] for f in game.frames for e in f['events']),
                        'display clock trails a later native event on the same tick')
        self.assertGreater(len({tuple((p['x'],p['y']) for p in f['view']['players']) for f in game.frames}), 100)
        self.assertGreater(len({tuple(f['view']['ball'].values()) for f in game.frames}), 100)
        self.assertEqual([e['id'] for e in game.events], list(range(831)))
        with self.assertRaises(ValueError):game.action('step')
        ready = game.action('restart')
        self.assertEqual((ready['phase'],ready['steps'],ready['events']),('ready',0,[]))
        self.assertEqual(ready['view']['score'],[0,0])

    def test_invalid_actions_and_expected_records_are_not_runtime_inputs(self):
        with self.assertRaises(ValueError):self.game.action('step')
        with self.assertRaises(ValueError):self.game.action('continue')
        with self.assertRaises(ValueError):self.game.action('configure')
        source = (ROOT/'tools/play_watched.py').read_text()
        for token in ('replay_', 'records/T41/reference', 'chunk-', 'Unicorn', 'unicorn'):
            self.assertNotIn(token,source)
        self.assertIn('native.watched_match(',source)

if __name__=='__main__':unittest.main()
