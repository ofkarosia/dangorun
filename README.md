# dangorun
Wuthering Waves Dango Run (Cubie Derby) Simulator 

English | [中文文档](./README.zh_CN.md)

## Disclaimer
- This project uses `ChaCha20Rng` as the random number generator to ensure cross-platform reproducibility as much as possible.
- To simplify the logic (due to my limited expertise), some logic depends on the specific game map and uses `assert` checks. There is no guarantee that custom maps will be logically correct or playable.
- Some non-critical code sections lack boundary checks.
- It is assumed that all Dangos start at the origin (starting point).*
- Due to ambiguities in some skill descriptions, logic may not be 100% consistent with the official game.
  - The exact triggering timing for some skills remains unclear; therefore, differences in trigger order may lead to different results.

## Notes on Dango Skills
- Abby: Tests show that Abby may return to the starting point. Handling: Before the condition to return to the finish line is triggered at the end of a round, it will remain at the starting point.
- Sigrika: Skill logic only considers the opponent's initial points. If the points are not greater than 1, no action is taken. The marking effect is applied after character skill bonuses.
- Augusta / Changli: For skills that force a unit to act last in the next round, if multiple are triggered, the last one to trigger will be the final unit to act.
- Augusta and Changli's skills consume two rounds and will not trigger consecutively.
- Jinhsi: After every Dango's movement, a check is triggered. If Jinhsi is on a normal tile, it checks if the Dango "stepped on" her head. If she is on a rearrangement tile, it only checks if the Dango is on the same tile as Jinhsi.

*: STC

## References / Similar projects
[wuwa_CubieDerby](https://github.com/EEEEEEEEdison/wuwa_CubieDerby)

[wuwa-dango-race-sim](https://github.com/Maxim00191/wuwa-dango-race-sim)

[wuwa-cubie-derby-sim](https://github.com/Boatkungg/wuwa-cubie-derby-sim)

## License
[MIT](./LICENSE)
