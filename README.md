**Final RUST project** by Aleksei Vasiukov and Mauricio Barros (2026)

An Idea of the future project.
It's the game of two parties fighting each other. 
Left party (ours) consist of selected (4 maximum) characters. The opposite party (enemy, on the right) are generated with equal power.
The calculations of units types parameters proportions can be found in Docs/UnitsParametersCalculations.xlsx

CURRENT STAGE: BASIC FUNCTIONALITY FOR MAIN PROJECT "GRAND AUTO-BATTLER" IN DEDICATED BRANCH IS PLACED

**MAIN PROJECT**:
NAVIGATE YOURSELF TO \FinalRustProject\GrandAutoBattler\ in VScode, Make the "GrandAutoBattler_Console_Runner" your active directory, then "cargo run".
-> **GrandAutoBattler_Console_Runner** (Just an entry point. In future there will be "runners" for ratatui port and REST API server)
-> **GrandAutoBattler_Console**  (Intended to keep States of the game to be presented in console window (using alternate screen/raw_mode/cursor_off) and process user's input. Now it is only requesting new game instance id)
->  **GrandAutoBattler_GameEngine** (Functional engine to parse TOML files of data-driven game files. It now initializing usable data for game. In future: Returning possible playable units. Generating whole gameplay sequence of command-pattern chained events, taking all gameplay calculations and parameters tracking on it. Intented to be agnostic for caller client.)

FOLLOWING IMPLEMENTATIONS (MUST)
**GrandAutoBattler_GameEngine**
    -Generating playable factions/units in one method, and accepting chosen with the other.
    -Generating command-pattern events of units blowing the hits on each other on their turn, calculactions are heavily based on units' parameters

**GrandAutoBattler_Console** 
    -State Machine looping user through game stages.
    -Input with buttons debouncing and catching not only spacebar and enter, but also arrows and letters
    -Console output in raw mode, alternative screen and cursor hidden, redrawn with new game stage applied.

 
FOLLOWING IMPLEMENTATIONS (NICE TO HAVE)
**GrandAutoBattler_GameEngine**
    -Gameplay progression. Most simple promotion of survived units considering events journal size increase after improvements
**GrandAutoBattler_Console** 
    -Graphics with pixel units representation, considering data-driven approach with unability to draw sprites for new units.
    -ratatui UI rework. Since game engine should be agnostic to client, the implementation is only limited by time and difficulity.

REST API + WebUI FrontEnd client.
    -First learn Tokio.
    -Then learn Axum.
    -Then investigate what's better Json REST requests or gRPC?
    -Then decisions on further implementations.

TEST PROJECTS:
(DONE)
- "CharacterGeneration" Generating an instance of character out of parameters Health, Attack, Range, Defense (the H.A.R.D system) 
- "NamesConcatenationTest" Data-driven prototype of concatenating random characters names out of First and Last names arranged in TOML files.
- "RustDungeonCrawler" The older prototype of Data-driven concept of randomly generated events. RNG test. TOML -> to "repository" test.
- "CommandPatternEvents" Try to adopt the Command Pattern for compact presentation of generated serie of the game events. Very basic, only about attack. Also test short GUID/UUID for characters to identify.
- "ParametersProportionDistribution" Units' parameters (Health, Attack, Range, Defense) generation system, considering the unit-type defining proportions with limits and additional bonus random points. (The Self-balancing  calculator app won't be implemented, use Docs/UnitsParametersCalculations.xlsx instead)
- "BattleSimulation" Test function, simulating the battle, calculating probability whom to hit based on Range, and damage based on Attack, Health and Defence percentages affected. 
- "StateMachineEngine" Game engine test project that handles 4 main stages Preparation, Generation, Battle and Result. Preparation is allowing to select characters. Generation stage is generating friends and foes. Battle stage is calculating serie of game events based on characters data. And Result stage making final conclusion.
- "UserInput" !!! Will not be implemented separatelly: for input tests refer to "StateMachineEngine" project!!! Try out the key on keyboard and mouse input. How letters keys, spacebar, and mouse buttons are registered. How Mouse coords could be cought?
- "GraphicsLayout" !!! Will not be implemented separatelly: for console  tests refer to "StateMachineEngine" project!!! Test how much we control the field for the battle in console. Colors, Tabulation to separate parties. Arrows or attack lines to show who hit whom.
  
(PLANNED)
- "RatatuiConcept" The project to try out TUI made with ratatui. Is it easy to work with?

...

Possible gameplay improvement to discuss: 
- Throughout progression. After battle finished, promote survived characters of own party, and empower the enemies. Will it complex the battle log too much?
- Time control. In general, the battle goes in straight pace, but when some character dies - slow there, to highlight the moment. Fast forward? Rewind? 

UI:
- (NOW) Console app with stdout and cls
- (MAYBE) ratatui TUI app with better graphics
- (Nice to have) REST API server powered with AXUM/Tokio and webUI (powered by ??)

Music - no 

Sound effects - probably no

Characters sprites/drawings - probably no

LLM generation of additional game data - even if implemented, will be only in demonstration limited form.

