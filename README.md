**Final RUST project** by Aleksei Vasiukov and Mauricio Barros (2026)

Get an Idea of the future project.
It's the game of two parties fighting each other. 
Left party (ours) consist of selected (4 maximum) characters. The opposite party (enemy, on the right) are generated with equal power.

CURRENT STAGE: MAKING THE TEST PROJECTS TO TRY OUT GAME MECHANICS

(DONE)
-"CharacterGeneration" Generating an instance of character out of parameters Health, Attack, Range, Defense (the H.A.R.D system) 
-"NamesConcatenationTest" Data-driven prototype of concatenating random characters names out of First and Last names arranged in TOML files.
-"RustDungeonCrawler" The older prototype of Data-driven concept of randomly generated events. RNG test. TOML -> to "repository" test.

(PLANNED)
-"UserInput" Try out the key on keyboard and mouse input. How letters keys, spacebar, and mouse buttons are registered. How Mouse coords could be cought?
-"CommandPatternEvents" Try to adopt the Command Pattern for compact presentation of generated serie of the game events. Very basic, only about attack. Also test short GUID/UUID for characters to identify.
-"GraphicsLayout" Test how much we control the field for the battle in console. Colors, Tabulation to separate parties. Arrows or attack lines to show who hit whom.
-"RatatuiConcept" The project to try out TUI made with ratatui. Is it easy to work with?

...

Possible gameplay improvement to discuss: 
-Throughout progression. After battle finished, promote survived characters of own party, and empower the enemies. Will it complex the battle log too much?
-Time control. In general, the battle goes in straight pace, but when some character dies - slow there, to highlight the moment. Fast forward? Rewind? 

UI:
(NOW) Console app with stdout and cls
(MAYBE) ratatui TUI app with better graphics
(Nice to have) REST API server powered with AXUM/Tokio and webUI (powered by ??)

Music - no 

Sound effects - probably no

Characters sprites/drawings - probably no

