use crate::{
    Buddy,
    combat::{
        breakbar::BreakbarHit,
        buff::{Buff, BuffApply},
        cast::{Cast, CastState},
        player::Player,
        transfer::{Apply, Condition, Remove},
    },
};
use arcdps::{Agent, CombatResult, Event, StateChange};

impl Buddy {
    pub fn event(
        event: Option<&Event>,
        src: Option<&Agent>,
        dst: Option<&Agent>,
        skill_name: Option<&str>,
    ) {
        if let Some(src) = src {
            if let Some(event) = event {
                Self::combat_event(event, src, dst, skill_name);
            } else if let Some(dst) = dst {
                // check for tracking addition
                if src.elite == 0 && src.prof != 0 {
                    let mut buddy = Self::lock();
                    if src.prof != 0 {
                        // player added
                        let player = Player::from_tracking_change(src, dst);
                        buddy.add_player(player, dst.is_self != 0);
                    } else {
                        // player removed
                        buddy.remove_player(src.id);
                    }
                }
            }
        }
    }

    pub fn combat_event(event: &Event, src: &Agent, dst: Option<&Agent>, skill_name: Option<&str>) {
        let src_self = src.is_self != 0;
        match event.get_statechange() {
            StateChange::SquadCombatStart => Self::lock().start_fight(event, dst),

            StateChange::LogNPCUpdate => Self::lock().fight_target(event, dst),

            StateChange::SquadCombatEnd => Self::lock().end_fight(event, dst),

            StateChange::AnimationStart if src_self => {
                let mut buddy = Self::lock();
                if let Some(time) = buddy.history.relative_time(event.time) {
                    if buddy.data.contains(event.skill_id) {
                        buddy.cast_start(event, skill_name, time)
                    }
                }
            }

            StateChange::AnimationStop if src_self => {
                let mut buddy = Self::lock();
                if let Some(time) = buddy.history.relative_time(event.time) {
                    if buddy.data.contains(event.skill_id) {
                        buddy.cast_end(event, skill_name, time)
                    }
                }
            }

            StateChange::BuffApply => {
                if let Some(dst) = dst {
                    let buff = event.skill_id;
                    if let Ok(buff) = buff.try_into() {
                        // only care about buff applies to other where source and dest are different
                        if dst.is_self == 0 && dst.id != src.id {
                            Self::lock().apply_buff(event, buff, src, dst)
                        }
                    } else if let Ok(condi) = buff.try_into() {
                        // only care about condi applies from self to other and ignore extensions
                        if src_self && dst.is_self == 0 && event.is_offcycle == 0 {
                            Self::lock().apply_condi(event, condi, dst)
                        }
                    }
                }
            }

            StateChange::BuffRemoveAll => {
                if let Some(dst) = dst {
                    // only care about removes from self to self
                    if src_self && dst.is_self != 0 {
                        if let Ok(condi) = event.skill_id.try_into() {
                            Self::lock().remove_buff(event, condi)
                        }
                    }
                }
            }

            StateChange::Combat => {
                let mut buddy = Self::lock();
                if let (Some(dst), Some(time)) = (dst, buddy.history.relative_time(event.time)) {
                    buddy.strike(event, skill_name, src, dst, time)
                }
            }

            _ => {}
        }
    }

    pub fn add_player(&mut self, player: Player, is_self: bool) {
        if is_self {
            self.self_instance_id = Some(player.instance_id);
            log::debug!("own instance id changed to {}", player.instance_id);
        }
        self.players.push(player);
    }

    pub fn remove_player(&mut self, id: usize) {
        if let Some(pos) = self.players.iter().position(|player| player.id == id) {
            self.players.swap_remove(pos);
        }
    }

    fn get_master(&self, event: &Event) -> Option<&crate::combat::player::Player> {
        self.players
            .iter()
            .find(|player| player.instance_id == event.src_master_instance_id)
    }

    fn is_own_minion(&self, event: &Event) -> bool {
        match self.self_instance_id {
            Some(id) => event.src_master_instance_id == id,
            None => false,
        }
    }

    fn start_fight(&mut self, event: &Event, target: Option<&Agent>) {
        let species = event.src_agent as u32;
        log::debug!("log start for {species}, {target:?}");
        self.history
            .add_fight_with_target(event.time, species, target);
    }

    fn fight_target(&mut self, event: &Event, target: Option<&Agent>) {
        let species = event.src_agent as u32;
        log::debug!("log target change to {species}, {target:?}");
        self.history
            .update_fight_target(event.time, species, target);
    }

    fn end_fight(&mut self, event: &Event, target: Option<&Agent>) {
        let species = event.src_agent;
        log::debug!("log end for {species}, {target:?}");
        self.history.end_latest_fight(event.time);
    }

    pub fn latest_cast_mut(&mut self, id: u32) -> Option<&mut Cast> {
        self.history.latest_fight_mut().and_then(|fight| {
            fight
                .data
                .casts
                .iter_mut()
                .rev()
                .find(|cast| cast.skill == id)
        })
    }

    pub fn add_cast(&mut self, cast: Cast) {
        if let Some(fight) = self.history.latest_fight_mut() {
            let casts = &mut fight.data.casts;
            let index = casts
                .iter()
                .rev()
                .position(|other| other.time <= cast.time)
                .unwrap_or(0);
            casts.insert(casts.len() - index, cast);
        }
    }

    fn cast_start(&mut self, event: &Event, skill_name: Option<&str>, time: i32) {
        let id = event.skill_id;
        let skill = self.skills.try_register(id, skill_name);
        log::debug!("start {skill:?}");
        let cast = Cast::from_start(time, id, CastState::Casting);
        self.add_cast(cast);
    }

    fn cast_end(&mut self, event: &Event, skill_name: Option<&str>, time: i32) {
        let state = event.get_animation_progress().into();
        let duration = event.value;
        let id = event.skill_id;
        self.skills.try_register(id, skill_name);
        if let Some(cast) = self.latest_cast_mut(event.skill_id) {
            cast.complete(id, state, duration, time);
            log::debug!("complete {cast:?}");
        } else {
            let cast = Cast::from_end(time - duration, id, state, duration);
            log::debug!("complete without start {cast:?}");
            self.add_cast(cast);
        }
    }

    fn apply_buff(&mut self, event: &Event, buff: Buff, src: &Agent, dst: &Agent) {
        if src.is_self != 0 || self.is_own_minion(event) {
            if let Some((time, fight)) = self.history.fight_and_time(event.time) {
                // TODO: "effective" duration excluding overstack?
                let duration = event.value;
                let apply = BuffApply::new(time, buff, duration, dst.into());
                fight.data.buffs.push(apply)
            }
        }
    }

    fn apply_condi(&mut self, event: &Event, condi: Condition, target: &Agent) {
        if let Some((time, fight)) = self.history.fight_and_time(event.time) {
            let apply = Apply::new(time, condi, event.value, target.into());
            fight.data.transfers.add_apply(apply);
        }
    }

    fn remove_buff(&mut self, event: &Event, condi: Condition) {
        if let Some((time, fight)) = self.history.fight_and_time(event.time) {
            let remove = Remove::new(time, condi, event.value);
            fight.data.transfers.add_remove(remove)
        }
    }

    fn strike(
        &mut self,
        event: &Event,
        skill_name: Option<&str>,
        attacker: &Agent,
        target: &Agent,
        time: i32,
    ) {
        let id = event.skill_id;
        self.skills.try_register(id, skill_name);
        let is_minion = self.is_own_minion(event);
        let is_own = attacker.is_self != 0 || is_minion;

        match event.get_combat_result() {
            CombatResult::StrikeDamage
            | CombatResult::StrikeDamageCrit
            | CombatResult::StrikeDamageGlance => {
                if is_own {
                    self.damage_hit(is_minion, id, target, time)
                }
            }
            CombatResult::BreakbarDamage => {
                let attacker = self
                    .get_master(event)
                    .map(|player| player.into())
                    .unwrap_or(attacker.into());
                self.breakbar_hit(id, attacker, is_own, target, event.value, time)
            }
            _ => {}
        }
    }

    fn damage_hit(&mut self, is_minion: bool, skill: u32, target: &Agent, time: i32) {
        // TODO: use local combat events for hits?
        if let Some(info) = self.data.get(skill) {
            if info.minion || !is_minion {
                let max = info.max_duration;
                let id = info.id;
                self.skills.try_duplicate(id, skill);
                match self.latest_cast_mut(id) {
                    Some(cast) if time - cast.time <= max => {
                        cast.hit(target);
                        log::debug!("hit {:?}, {target:?}", cast.skill);
                    }
                    _ => {
                        let cast = Cast::from_hit(time, id, target);
                        log::debug!("hit without start {:?}, {target:?}", cast.skill);
                        self.add_cast(cast);
                    }
                }
            }
        }
    }

    fn breakbar_hit(
        &mut self,
        skill: u32,
        attacker: crate::combat::Agent,
        is_own: bool,
        target: &Agent,
        damage: i32,
        time: i32,
    ) {
        // TODO: minion indicator?
        if let Some(fight) = self.history.latest_fight_mut() {
            log::debug!("breakbar {damage} {skill:?} from {attacker:?} to {target:?}");
            let hit = BreakbarHit::new(time, skill, damage, attacker, is_own, target.into());
            fight.data.breakbar.push(hit);
        }
    }
}
