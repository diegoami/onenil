"""[designed] Detailed native action semantics and full-match controller acceptance."""
import unittest
from play_watched import describe

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
    def test_unknown_codes_and_missing_named_players_fail_explicitly(self):
        with self.assertRaises(ValueError):describe(self.event(255),{2591:'Shearer'})
        with self.assertRaises(ValueError):describe(self.event(8),{2591:'Shearer'})

if __name__=='__main__':unittest.main()
