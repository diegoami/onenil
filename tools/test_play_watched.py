"""[designed] Detailed native action semantics and full-match controller acceptance."""
import json
import tempfile
import unittest
from pathlib import Path
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
    def test_partial_browser_evidence_is_never_reused(self):
        from smoke_watched_viewer import next_stem
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            self.assertEqual(next_stem(root),'T42-r0001')
            (root/'T42-r0001-desktop.png').write_bytes(b'partial-test-evidence')
            self.assertEqual(next_stem(root),'T42-r0002')


class NativeController(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        import onenil_engine as native
        from play_watched import STATIC_INPUT
        allowed = {ROOT/'engine/tests/fixtures/prepared_match/admission.json', ROOT/'demo/metadata.json', STATIC_INPUT}
        reads = set()
        original = native.NativePreparedWatchedSession
        def audit(function):
            def read(path, *args, **kwargs):
                if path not in allowed:
                    raise AssertionError(f'unexpected product runtime input: {path}')
                reads.add(path)
                return function(path, *args, **kwargs)
            return read
        with patch.object(native, 'watched_match', side_effect=AssertionError('captured postsetup backend forbidden')), \
             patch.object(native, 'NativePreparedWatchedSession', wraps=original) as compute, \
             patch.object(Path, 'read_bytes', audit(Path.read_bytes)), \
             patch.object(Path, 'read_text', audit(Path.read_text)):
            cls.game = Match()
            for lineup in ('reference', 'reserve'):
                for seed in ('64', '122'):
                    cls.game.configure(dict(lineup=lineup, seed=seed))
                    if cls.game.phase != 'ready':
                        raise AssertionError(cls.game.error)
            if compute.call_count != 4:
                raise AssertionError('native computation was not exactly once per supported choice')
            if reads != allowed:
                raise AssertionError('runtime did not use exactly declared initial/name inputs')

    def setUp(self):
        self.game.restart()
        self.game.configure(dict(lineup='reference', seed='64'))

    def finish(self, game):
        game.action('start');halves=0
        while game.phase != 'full-time':
            if game.phase == 'half-time':
                halves += 1
                with self.assertRaises(ValueError):game.action('step')
                game.action('continue')
            game.action('step')
        self.assertEqual(halves, 1)
        self.assertEqual(game.cursor, len(game.frames))

    def test_complete_original_reference_actions_positions_scores_and_halftime(self):
        game=self.game
        self.assertEqual(game.view['score'],[0,0])
        self.finish(game)
        self.assertEqual((game.cursor,game.view['score'],game.view['clock']),(4199,[2,1],93))
        watched=[e for e in game.events if e['tick']>0]
        self.assertEqual(len(watched),831)
        self.assertEqual(game.events[0]['code'],59)
        original=[]
        for path in sorted((ROOT/'records/T41/reference').glob('chunk-*.json')):
            original.extend(json.loads(path.read_text())['payload']['actions'])
        fields=('code','player','secondary','club','side','tick','minute')
        self.assertEqual([tuple(e[k]for k in fields)for e in watched], [tuple(e[k]for k in fields)for e in original])
        self.assertTrue({'pass','receive','tackle','shot','save','goal','substitution'} <= {e['kind']for e in watched})
        self.assertTrue(all('Play continues'not in e['text']for e in game.events))
        self.assertTrue(all(len(f['view']['players'])==22 for f in game.frames))
        self.assertTrue(all(e['minute']<=f['view']['clock'] for f in game.frames for e in f['events']))
        self.assertGreater(len({tuple((p['x'],p['y'])for p in f['view']['players'])for f in game.frames}),100)
        self.assertGreater(len({tuple(f['view']['ball'].values())for f in game.frames}),100)
        self.assertEqual([e['id']for e in game.events],list(range(832)))
        with self.assertRaises(ValueError):game.action('step')

    def test_all_choices_actual_starting_player_seed_fulltime_and_restart(self):
        expected=json.loads((ROOT/'records/T46/native-completions-r0004.json').read_text())['sessions']
        for record in expected:
            game=self.game;game.restart();game.configure(record['choices'])
            initial=json.dumps(game.state(),sort_keys=True)
            starter=2590 if record['choices']['lineup']=='reference' else 2593
            other=2593 if starter==2590 else 2590
            self.assertIn(starter,game.selection['players']);self.assertNotIn(other,game.selection['players'])
            self.assertEqual(game.selection['seed'],int(record['choices']['seed']))
            self.assertEqual(game.selection['players'],record['actual_starting_players'])
            self.finish(game)
            self.assertEqual(game.view['score'],record['score'])
            self.assertEqual(game.cursor,record['ticks'])
            self.assertEqual(len([e for e in game.events if e['tick']>0]),record['actions'])
            self.assertEqual(game._data['native_return'],'0AB9:EBE2')
            game.restart();self.assertEqual(json.dumps(game.state(),sort_keys=True),initial)

    def test_invalid_choices_actions_and_cached_native_results(self):
        import onenil_engine as native
        game=self.game;before=json.dumps(game.state(),sort_keys=True)
        for value in (None,{},dict(lineup='unknown',seed='64'),dict(lineup='reserve',seed=122),dict(lineup='reference',seed='64',expected={})):
            with self.assertRaises(ValueError):game.configure(value)
            self.assertEqual(json.dumps(game.state(),sort_keys=True),before)
        for action in ('step','continue','configure'):
            with self.assertRaises(ValueError):game.action(action)
        with patch.object(native,'NativePreparedWatchedSession',side_effect=AssertionError('cache recomputed native match')):
            for lineup in ('reference','reserve'):
                for seed in ('64','122'):
                    game.configure(dict(lineup=lineup,seed=seed));game.restart()
        game.action('start');started=json.dumps(game.state(),sort_keys=True)
        with self.assertRaises(ValueError):game.configure(dict(lineup='reference',seed='64'))
        self.assertEqual(json.dumps(game.state(),sort_keys=True),started)
        source=(ROOT/'tools/play_watched.py').read_text()
        for token in ('native.watched_match(', 'fixtures/watched_match/initial.json', 'records/T41/reference', 'Unicorn', 'unicorn'):
            self.assertNotIn(token,source)
        self.assertIn('NativePreparedWatchedSession',source)

    def test_preparation_state_is_visible_and_invalid_request_does_not_replace_it(self):
        from threading import Event
        entered=Event();release=Event()
        # Block before native computation,then reuse this process's genuinely
        # computed presentation;this test isolates controller concurrency only.
        def build(game,choices):
            entered.set();release.wait(5)
            return self.game._cache[('reference','64')]
        with patch.object(Match,'_build',build):
            game=Match(background=True)
            self.assertTrue(entered.wait(2));state=game.state()
            self.assertEqual(state['phase'],'preparing');self.assertIsNone(state['view'])
            before=json.dumps(state,sort_keys=True)
            with self.assertRaises(ValueError):game.configure(dict(lineup='invalid',seed='64'))
            self.assertEqual(json.dumps(game.state(),sort_keys=True),before)
            release.set();game._worker.join(5)
            self.assertEqual(game.state()['phase'],'ready')

if __name__=='__main__':unittest.main()
