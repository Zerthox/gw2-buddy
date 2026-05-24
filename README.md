# GW2 Buddy
Addon for [Guild Wars 2](https://guildwars2.com) assisting with evaluating combat gameplay.

Published releases are found [here](../../releases).
Available as [Nexus](https://github.com/RaidcoreGG/Nexus) addon ~~or [ArcDPS](https://deltaconnected.com/arcdps) extension~~.
Requires [ArcDPS](https://deltaconnected.com/arcdps) to work.

## Features
- Track skill cast states, durations & hit counts
- Track buff applications to other players & NPCs
- Track breakbar damage hits
- Track condition transfers

<p>
  <img src="./screenshots/casts.png" alt="Casts screenshot" width="300"/>
  <img src="./screenshots/buffs.png" alt="Buffs screenshot" width="300"/>
  <img src="./screenshots/breakbar.png" alt="Breakbar screenshot" width="300"/>
  <img src="./screenshots/transfer.png" alt="Transfer screenshot" width="300"/>
</p>

## Casts
Displays casted skills and their durations and hit counts.
Duration is colored based on the animation as categorized by ArcDPS (full aftercast, cancelled aftercast, interrupt).
Hit count is colored based on the expected and maximum amount of hits.

You can supply custom skill definitions via `arcdps_buddy_skills.yml`.
The plugin ships with the default definitions in [src/data/skills](./src/data/skills/).
A single entry for a skill looks like this:

```yml
- id: 12345 # id of casted skill.
  hit_ids: [23456, 34567] # optional: additional skill ids to count for hits.
  hits: 5 # optional: number of hits. enables hit tracking. set to 0 to track with unknown hits.
  expected: 4 # optional: number of expected hits. threshold for yellow color. defaults to >= half hits.
  max_duration: 10000 # optional: maximum duration (ms). hits after the duration + error margin count towards a new cast.
  minion: true # optional: whether to include hits from own minions.
```

Individual default skill definitions can be overwritten or disabled:

```yml
- id: 12345 # overwrite the default entry entirely
  hits: 5
- id: 12345 # disable the default entry
  enabled: false
```

## Buffs
Displays buffs applied to other players and NPCs.
Includes buff applications from own minions.
Ignores applications from self to self, own minion to self and own minion to same minion.
Target is colored based on being a player or NPC/minion.
The following buff applications are currently tracked:

- [Quickness](https://wiki.guildwars2.com/wiki/Quickness)
- [Alacrity](https://wiki.guildwars2.com/wiki/Alacrity)
- Thief [Venoms](https://wiki.guildwars2.com/wiki/Venom)
- Elementalist [Arcane Power](https://wiki.guildwars2.com/wiki/Arcane_Power_(effect))
- Revenant [Rite of the Great Dwarf](https://wiki.guildwars2.com/wiki/Rite_of_the_Great_Dwarf_(effect))
- Firebrand [Ashes of the Just](https://wiki.guildwars2.com/wiki/Ashes_of_the_Just)
- Soulbeast [Stances](https://wiki.guildwars2.com/wiki/Stance) (for [Leader of the Pack](https://wiki.guildwars2.com/wiki/Leader_of_the_Pack))


## Breakbar
Displays [defiance bar](https://wiki.guildwars2.com/wiki/Defiance_bar) damage hits and their respective skill name, damage amount and target.
Optionally all defiance bar damage from group/squad members can be displayed.
Target is colored based on main log target species.

## Transfer
Displays [transferred conditions](https://wiki.guildwars2.com/wiki/Condition#Skills_that_transfer_conditions) and their respective stack count and target.
Target is colored based on main log target species.
