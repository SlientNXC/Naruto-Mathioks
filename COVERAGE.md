# Coverage of the native rebuild

Status: development prototype, not a complete Mathioks port.

| System | Implemented here | Still needed |
| --- | --- | --- |
| Character basics | Original 25 starting chakra, Ninjutsu capacity | Original nature-paper initialization and full stat sheet |
| Passive chakra | Immediate eligible recovery, then 100-tick cadence | Sharingan/Byakugan and other mode interactions |
| Progression | Original XP threshold, one level/tick, SP/JP brackets and level-50 bonus | Original combat XP, quests, exams and skill learners |
| Persistence | Per-world, per-UUID character JSON; previous save retained | Original world/player NBT import and migration |
| HUD and input | Chakra bar, level, selected jutsu, rebindable keys | Original menus, inventory items and full controls |
| Fireball | Native Horizon projectile and original chakra cost | Exact original flight/spawn/damage scaling/explosion parity; unlock/nature rules |
| Rasengan | Aimed melee mob damage, original cost/damage formula, knockback approximation | Original charged hand/item, animation, exact knockback and player/boss targeting |
| Chidori | Aimed melee mob damage and original cost/damage formula | Original dash window, path damage, paralysis and visuals |
| Clans and bloodlines | Not implemented | All clan rolls, bonuses and techniques |
| Story and NPCs | Not implemented | Storyline, characters, dialogue, missions, exams and shops |
| Other jutsu and modes | Not implemented | Shadow clones, Sharingan/Byakugan, transformations, sage modes, tailed beasts and remaining techniques |
| Items, clothing, models, audio | Not implemented | Local asset converter and format adaptation, with appropriate distribution rights |
| Multiplayer | Not supported | Server/client synchronization, remote HUD, guest actions and dedicated server build |

The three named jutsu are prototypes rather than parity-complete implementations. Training, charging, and the short global cooldown are temporary development mechanics. Do not present this package as the original mod or as a completed native port.
