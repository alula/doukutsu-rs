use num_traits::{abs, clamp};

use crate::caret::CaretType;
use crate::common::{Direction, CDEG_RAD};
use crate::framework::context::Context;
use crate::framework::error::GameResult;
use crate::npc::list::NPCList;
use crate::npc::NPC;
use crate::player::Player;
use crate::rng::RNG;
use crate::shared_game_state::SharedGameState;

impl NPC {
    pub(crate) fn tick_n044_polish(&mut self, state: &mut SharedGameState, npc_list: &NPCList) -> GameResult {
        match self.action_num {
            0 | 1 => {
                self.anim_num = 0;
                self.action_num = match self.direction {
                    Direction::Left => 8,
                    Direction::Right => 2,
                    _ => 8,
                };
            }
            2 => {
                self.vel_y += 0x20;
                if self.vel_y > 0 && self.flags.hit_bottom_wall() {
                    self.vel_y = -0x100;
                    self.vel_x += 0x100;
                }

                if self.flags.hit_right_wall() {
                    self.action_num = 3;
                }
            }
            3 => {
                self.vel_x += 0x20;
                if self.vel_x > 0 && self.flags.hit_right_wall() {
                    self.vel_x = -0x100;
                    self.vel_y -= 0x100;
                }

                if self.flags.hit_top_wall() {
                    self.action_num = 4;
                }
            }
            4 => {
                self.vel_y -= 0x20;
                if self.vel_y < 0 && self.flags.hit_top_wall() {
                    self.vel_y = 0x100;
                    self.vel_x -= 0x100;
                }

                if self.flags.hit_left_wall() {
                    self.action_num = 5;
                }
            }
            5 => {
                self.vel_x -= 0x20;
                if self.vel_x < 0 && self.flags.hit_left_wall() {
                    self.vel_x = 0x100;
                    self.vel_y += 0x100;
                }

                if self.flags.hit_bottom_wall() {
                    self.action_num = 2;
                }
            }
            6 => {
                self.vel_y += 0x20;
                if self.vel_y > 0 && self.flags.hit_bottom_wall() {
                    self.vel_y = -0x100;
                    self.vel_x -= 0x100;
                }

                if self.flags.hit_left_wall() {
                    self.action_num = 7;
                }
            }
            7 => {
                self.vel_x -= 0x20;
                if self.vel_x < 0 && self.flags.hit_left_wall() {
                    self.vel_x = 0x100;
                    self.vel_y -= 0x100;
                }

                if self.flags.hit_top_wall() {
                    self.action_num = 8;
                }
            }
            8 => {
                self.vel_y -= 0x20;
                if self.vel_y < 0 && self.flags.hit_top_wall() {
                    self.vel_y = 0x100;
                    self.vel_x += 0x100;
                }

                if self.flags.hit_right_wall() {
                    self.action_num = 9;
                }
            }
            9 => {
                self.vel_x += 0x20;
                if self.vel_x > 0 && self.flags.hit_right_wall() {
                    self.vel_x = -0x100;
                    self.vel_y += 0x100;
                }

                if self.flags.hit_bottom_wall() {
                    self.action_num = 6;
                }
            }
            _ => {}
        }

        if self.life <= 100 {
            npc_list.create_death_smoke(self.x, self.y, self.display_bounds.right, 8, state, &self.rng);
            state.sound_manager.play_sfx(25);
            self.cond.set_alive(false);

            let mut npc = NPC::create(45, &state.npc_table);
            npc.cond.set_alive(true);
            npc.x = self.x;
            npc.y = self.y;
            for _ in 0..9 {
                let _ = npc_list.spawn(0x100, npc.clone());
            }
        }

        self.vel_x = clamp(self.vel_x, -0x200, 0x200);
        self.vel_y = clamp(self.vel_y, -0x200, 0x200);

        if self.shock > 0 {
            self.x += self.vel_x / 2;
            self.y += self.vel_y / 2;
        } else {
            self.x += self.vel_x;
            self.y += self.vel_y;
        }

        if self.action_num > 1 && self.action_num <= 9 {
            self.anim_num += 1;
            if self.anim_num > 2 {
                self.anim_num = 1;
            }
        }

        let dir_offset = if self.direction == Direction::Left { 0 } else { 3 };

        self.anim_rect = state.constants.npc.n044_polish[self.anim_num as usize + dir_offset];

        Ok(())
    }

    pub(crate) fn tick_n045_baby(&mut self, state: &mut SharedGameState) -> GameResult {
        if self.action_num == 0 {
            self.action_num = 2;
            self.vel_x = if self.rng.next_u16() & 1 != 0 {
                self.rng.range(-0x200..-0x100) as i32
            } else {
                self.rng.range(0x100..0x200) as i32
            };
            self.vel_y = if self.rng.next_u16() & 1 != 0 {
                self.rng.range(-0x200..-0x100) as i32
            } else {
                self.rng.range(0x100..0x200) as i32
            };
            self.vel_x2 = self.vel_x;
            self.vel_y2 = self.vel_y;
        }

        match self.action_num {
            1 | 2 => {
                self.anim_num += 1;
                if self.anim_num > 2 {
                    self.anim_num = 1;
                }
            }
            _ => {}
        }

        if self.vel_x2 < 0 && self.flags.hit_left_wall() {
            self.vel_x2 = -self.vel_x2;
        }

        if self.vel_x2 > 0 && self.flags.hit_right_wall() {
            self.vel_x2 = -self.vel_x2;
        }

        if self.vel_y2 < 0 && self.flags.hit_top_wall() {
            self.vel_y2 = -self.vel_y2;
        }

        if self.vel_y2 > 0 && self.flags.hit_bottom_wall() {
            self.vel_y2 = -self.vel_y2;
        }

        self.vel_x2 = clamp(self.vel_x2, -0x200, 0x200);
        self.vel_y2 = clamp(self.vel_y2, -0x200, 0x200);

        if self.shock > 0 {
            self.x += self.vel_x2 / 2;
            self.y += self.vel_y2 / 2;
        } else {
            self.x += self.vel_x2;
            self.y += self.vel_y2;
        }

        self.anim_rect = state.constants.npc.n045_baby[self.anim_num as usize];

        Ok(())
    }

    pub(crate) fn tick_n047_sandcroc(&mut self, state: &mut SharedGameState, players: [&mut Player; 2]) -> GameResult {
        match self.action_num {
            0 | 1 => {
                if self.action_num == 0 {
                    self.action_num = 1;
                    self.action_counter = 0;
                    self.anim_num = 0;
                    self.target_y = self.y;
                    self.npc_flags.set_shootable(false);
                    self.npc_flags.set_ignore_solidity(false);
                    self.npc_flags.set_invulnerable(false);
                    self.npc_flags.set_solid_soft(false);
                }

                let player = self.get_closest_player_mut(players);
                if abs(self.x - player.x) < 8 * 0x200 && player.y > self.y && player.y < self.y + 8 * 0x200 {
                    self.action_num = 2;
                    self.action_counter = 0;
                    state.sound_manager.play_sfx(102);
                }

                self.x += (player.x - self.x).signum() * 2 * 0x200;
            }
            2 => {
                self.anim_counter += 1;
                if self.anim_counter > 3 {
                    self.anim_num += 1;
                    self.anim_counter = 0;
                }

                match self.anim_num {
                    3 => self.damage = 10,
                    4 => {
                        self.action_num = 3;
                        self.action_counter = 0;
                        self.npc_flags.set_shootable(true);
                    }
                    _ => {}
                }
            }
            3 => {
                self.damage = 0;
                self.npc_flags.set_solid_soft(true);

                self.action_counter += 1;
                if self.shock > 0 {
                    self.action_num = 4;
                    self.action_counter = 0;
                }
            }
            4 => {
                self.npc_flags.set_ignore_solidity(true);
                self.y += 0x200;
                self.action_counter += 1;
                if self.action_counter == 32 {
                    self.action_num = 5;
                    self.action_counter = 0;
                    self.npc_flags.set_solid_soft(false);
                    self.npc_flags.set_shootable(false);
                }
            }
            5 => {
                if self.action_counter > 99 {
                    self.y = self.target_y;
                    self.action_num = 0;
                    self.anim_num = 0;
                } else {
                    self.action_counter += 1;
                }
            }
            _ => {}
        }

        self.anim_rect = state.constants.npc.n047_sandcroc[self.anim_num as usize];

        Ok(())
    }

    pub(crate) fn tick_n049_skullhead(
        &mut self,
        state: &mut SharedGameState,
        players: [&mut Player; 2],
        npc_list: &NPCList,
    ) -> GameResult {
        let parent = self.get_parent_ref_mut(npc_list);

        if self.action_num > 9 && parent.as_ref().map(|n| n.npc_type == 3).unwrap_or(false) {
            self.action_num = 3;
            self.vel_x = 0;
            self.vel_y = 0;
            self.action_counter2 = 1;
        }

        if self.flags.hit_left_wall() {
            self.direction = Direction::Right;
            self.vel_x = 0x100;
        }

        if self.flags.hit_right_wall() {
            self.direction = Direction::Left;
            self.vel_x = -0x100;
        }

        match self.action_num {
            0 | 1 => {
                if self.action_num == 0 {
                    self.action_num = if parent.is_some() { 10 } else { 1 };
                }

                self.action_counter += 1;
                if self.action_counter > 3 {
                    self.vel_y = -0x400;
                    self.action_num = 3;
                    self.anim_num = 2;

                    if self.action_counter2 > 0 {
                        self.vel_x = if self.direction == Direction::Left { -0x200 } else { 0x200 };
                    } else if self.direction != Direction::Left {
                        self.vel_x = 0x100;
                    } else {
                        self.vel_x = -0x100;
                    }
                }

                self.anim_num = 1;
            }
            3 => {
                if self.flags.hit_bottom_wall() {
                    self.action_num = 1;
                    self.action_counter = 0;
                    self.vel_x = 0;
                }

                self.anim_num = if self.flags.hit_bottom_wall() || self.vel_y > 0 { 1 } else { 2 };
            }
            10 => {
                if self.vel_y2 >= 50 {
                    let player = self.get_closest_player_mut(players);

                    if abs(self.x - player.x) < 0x10000 && abs(self.y - player.y) < 0xc000 {
                        self.action_num = 11;
                        self.action_counter = 0;
                        self.anim_num = 2;
                    }
                } else {
                    self.vel_y2 += 1;
                }
            }
            11 => {
                self.action_counter += 1;
                if self.action_counter == 30 || self.action_counter == 35 {
                    let player = self.get_closest_player_mut(players);

                    let angle = f64::atan2((self.y + 0x800 - player.y) as f64, (self.x - player.x) as f64);

                    let mut npc = NPC::create(50, &state.npc_table);
                    npc.cond.set_alive(true);
                    npc.x = self.x;
                    npc.y = self.y;
                    npc.vel_x = (angle.cos() * -1024.0) as i32;
                    npc.vel_y = (angle.sin() * -1024.0) as i32;

                    let _ = npc_list.spawn(0x100, npc);
                    state.sound_manager.play_sfx(39);
                }

                if self.action_counter > 50 {
                    self.action_num = 10;
                    self.vel_y2 = 0;
                    self.anim_num = 1;
                }
            }
            _ => {}
        }

        if self.action_num > 9 {
            if let Some(parent) = parent {
                self.x = parent.x;
                self.y = parent.y + 0x2000;
                self.direction = parent.direction;
                parent.vel_y2 -= 1;
            }
        }

        self.vel_y += 0x40;
        if self.vel_y > 0x5ff {
            self.vel_y = 0x5ff;
        }

        self.x += self.vel_x;
        self.y += self.vel_y;

        let dir_offset = if self.direction == Direction::Left { 0 } else { 3 };

        self.anim_rect = state.constants.npc.n049_skullhead[self.anim_num as usize + dir_offset];

        Ok(())
    }

    pub(crate) fn tick_n050_skeleton_projectile(&mut self, state: &mut SharedGameState) -> GameResult {
        match self.action_num {
            0 | 1 => {
                if self.action_num == 0 {
                    if self.direction == Direction::Right {
                        self.action_num = 2;
                    }
                }

                self.x += self.vel_x;
                self.y += self.vel_y;

                if self.flags.hit_left_wall() {
                    self.action_num = 2;
                    self.vel_x = 0x200;
                    self.action_counter2 += 1;
                }

                if self.flags.hit_right_wall() {
                    self.action_num = 2;
                    self.vel_x = -0x200;
                    self.action_counter2 += 1;
                }

                if self.flags.hit_top_wall() {
                    self.action_num = 2;
                    self.vel_y = 0x200;
                    self.action_counter2 += 1;
                }

                if self.flags.hit_bottom_wall() {
                    self.action_num = 2;
                    self.vel_y = -0x200;
                    self.action_counter2 += 1;
                }
            }
            2 => {
                self.vel_y += 0x40;
                self.x += self.vel_x;
                self.y += self.vel_y;

                if self.flags.hit_bottom_wall() {
                    self.action_counter2 += 1;
                    if self.action_counter2 > 1 {
                        state.create_caret(self.x, self.y, CaretType::ProjectileDissipation, Direction::Left);
                        self.cond.set_alive(false);
                    }
                }
            }
            _ => {}
        }

        self.vel_y = clamp(self.vel_y, -0x5ff, 0x5ff);

        self.anim_counter += 1;
        if self.anim_counter > 1 {
            self.anim_counter = 0;

            self.anim_num = if self.direction == Direction::Left {
                (self.anim_num + 1) % 4
            } else {
                self.anim_num.wrapping_sub(1) % 4
            }
        }

        self.anim_rect = state.constants.npc.n050_skeleton_projectile[self.anim_num as usize];

        Ok(())
    }

    pub(crate) fn tick_n051_crow_and_skullhead(
        &mut self,
        state: &mut SharedGameState,
        players: [&mut Player; 2],
        npc_list: &NPCList,
    ) -> GameResult {
        let player = self.get_closest_player_mut(players);

        match self.action_num {
            0 | 1 => {
                if self.action_num == 0 {
                    self.action_num = 1;
                    self.target_x = self.x;
                    self.target_y = self.y;
                    self.vel_y = 0x400;

                    let mut npc = NPC::create(49, &state.npc_table);
                    npc.cond.set_alive(true);
                    npc.parent_id = self.id;
                    let _ = npc_list.spawn(0, npc);
                }

                self.direction = if player.x >= self.x { Direction::Right } else { Direction::Left };
                self.vel_y += (self.target_y - self.y).signum() * 0x0a;
                self.vel_y = clamp(self.vel_y, -0x200, 0x200);

                if self.vel_y2 >= 10 {
                    self.action_num = 2;
                } else {
                    self.vel_y2 += 1;
                }
            }
            2 => {
                self.direction = if player.x >= self.x { Direction::Right } else { Direction::Left };

                self.vel_x += if self.y <= player.y + 0x4000 {
                    (player.x - self.x).signum() * 0x10
                } else {
                    (self.x - player.x).signum() * 0x10
                };

                self.vel_y += (player.y - self.y).signum() * 0x10;

                if self.shock > 0 {
                    self.vel_x = 0;
                    self.vel_y += 0x20;
                }
            }
            _ => {}
        }

        if self.vel_x < 0 && self.flags.hit_left_wall() {
            self.vel_x = 0x100;
        }

        if self.vel_x > 0 && self.flags.hit_right_wall() {
            self.vel_x = -0x100;
        }

        if self.vel_y < 0 && self.flags.hit_top_wall() {
            self.vel_y = 0x100;
        }

        if self.vel_y > 0 && self.flags.hit_bottom_wall() {
            self.vel_y = -0x100;
        }

        self.vel_x = clamp(self.vel_x, -0x400, 0x400);
        self.vel_y = clamp(self.vel_y, -0x200, 0x200);

        self.x += self.vel_x;
        self.y += self.vel_y;

        if self.shock > 0 {
            self.anim_num = 4;
        } else if self.action_num == 2 && self.y < player.y - 0x4000 {
            self.anim_num = 0;
        } else if self.action_num != 0 {
            self.animate(1, 0, 1);
        }

        let dir_offset = if self.direction == Direction::Left { 0 } else { 5 };

        self.anim_rect = state.constants.npc.n051_crow_and_skullhead[self.anim_num as usize + dir_offset];

        Ok(())
    }

    pub(crate) fn tick_n053_skullstep_leg(&mut self, state: &mut SharedGameState, npc_list: &NPCList) -> GameResult {
        let parent = self.get_parent_ref_mut(npc_list);
        if parent.is_none() || parent.as_ref().unwrap().npc_type == 3 {
            self.vanish(state);
            npc_list.create_death_smoke(self.x, self.y, self.display_bounds.right, 4, state, &self.rng);
            return Ok(());
        }

        let parent = parent.unwrap();

        let angle = self.vel_x + parent.vel_y2;

        if self.action_num < 2 {
            if self.action_num == 0 {
                self.action_num = 1;
                self.action_counter2 = 10;
            }

            if self.direction == Direction::Left && self.flags.hit_left_slope() {
                parent.y -= 0x400;
                parent.vel_y -= 0x100;
            }

            if self.direction == Direction::Right && self.flags.hit_right_slope() {
                parent.y -= 0x400;
                parent.vel_y -= 0x100;
            }

            if self.flags.hit_bottom_wall() {
                parent.y -= 0x400;
                parent.vel_y -= 0x100;
                parent.vel_x += parent.direction.vector_x() * 0x80;
            }

            self.x = parent.x + (self.action_counter2 as f64 * (angle as f64 * CDEG_RAD).cos() * 512.0) as i32;
            self.y = parent.y + (self.action_counter2 as f64 * (angle as f64 * CDEG_RAD).sin() * 512.0) as i32;
        }

        self.direction = parent.direction;
        self.anim_num = if angle < 0x14 || angle > 0x6c { 1 } else { 0 };

        let dir_offset = if self.direction == Direction::Left { 0 } else { 2 };

        self.anim_rect = state.constants.npc.n053_skullstep_leg[self.anim_num as usize + dir_offset];

        Ok(())
    }

    pub(crate) fn tick_n054_skullstep(&mut self, state: &mut SharedGameState, npc_list: &NPCList) -> GameResult {
        match self.action_num {
            0 | 1 => {
                if self.action_num == 0 {
                    self.action_num = 1;
                    self.anim_num = 1;

                    let mut leg = NPC::create(53, &state.npc_table);
                    leg.cond.set_alive(true);
                    leg.direction = self.direction;
                    leg.parent_id = self.id;

                    npc_list.spawn(256, leg.clone());

                    leg.vel_x = 0x80;
                    npc_list.spawn(0, leg);
                }

                self.vel_y2 += self.direction.vector_x() * 6;

                if self.flags.hit_bottom_wall() {
                    self.vel_x = self.vel_x * 3 / 4;
                    self.action_counter += 1;
                    if self.action_counter > 60 {
                        self.action_num = 2;
                        self.action_counter = 0;
                    }
                } else {
                    self.action_counter = 0;
                }

                if self.direction == Direction::Left && self.flags.hit_left_wall() {
                    self.action_counter2 += 1;
                    if self.action_counter2 > 8 {
                        self.direction = Direction::Right;
                        self.vel_x = -self.vel_x;
                    }
                } else if self.direction == Direction::Right && self.flags.hit_right_wall() {
                    self.action_counter2 += 1;
                    if self.action_counter2 > 8 {
                        self.direction = Direction::Left;
                        self.vel_x = -self.vel_x;
                    }
                } else {
                    self.action_counter2 = 0;
                }
            }
            2 => {
                self.action_counter += 1;
                self.shock += self.action_counter & 0xff;
                if self.action_counter > 50 {
                    state.sound_manager.play_sfx(25);
                    self.vanish(state);
                    npc_list.create_death_smoke(self.x, self.y, self.display_bounds.right, 8, state, &self.rng);
                }
            }
            _ => {}
        }

        self.vel_y += 0x80;
        self.vel_x = clamp(self.vel_x, -0x2ff, 0x2ff);
        self.vel_y = clamp(self.vel_y, -0x2ff, 0x2ff);

        self.x += self.vel_x;
        self.y += self.vel_y;

        let dir_offset = if self.direction == Direction::Left { 0 } else { 3 };

        self.anim_rect = state.constants.npc.n054_skullstep[self.anim_num as usize + dir_offset];

        Ok(())
    }

    pub(crate) fn tick_n120_colon_a(&mut self, state: &mut SharedGameState) -> GameResult {
        let anim = if self.direction == Direction::Left { 0 } else { 1 };

        self.anim_rect = state.constants.npc.n120_colon_a[anim];

        Ok(())
    }

    pub(crate) fn tick_n124_sunstone(&mut self, state: &mut SharedGameState) -> GameResult {
        match self.action_num {
            0 | 1 => {
                if self.action_num == 0 {
                    self.action_num = 1;
                    self.x += 8 * 0x200;
                    self.y += 8 * 0x200;
                }

                self.npc_flags.set_ignore_solidity(false);
                self.anim_num = 0;
            }
            10 | 11 => {
                if self.action_num == 10 {
                    self.action_num = 11;
                    self.action_counter = 0;
                    self.anim_num = 1;

                    self.npc_flags.set_ignore_solidity(true);
                }

                match self.direction {
                    Direction::Left => self.x -= 0x80,
                    Direction::Up => self.y -= 0x80,
                    Direction::Right => self.x += 0x80,
                    Direction::Bottom => self.y += 0x80,
                    Direction::FacingPlayer => {}
                }

                state.quake_counter = 20;
                if self.action_counter % 8 == 0 {
                    state.sound_manager.play_sfx(26);
                }
            }
            _ => {}
        }

        self.anim_rect = state.constants.npc.n124_sunstone[self.anim_num as usize];

        Ok(())
    }

    pub(crate) fn tick_n131_puppy_sleeping(&mut self, state: &mut SharedGameState) -> GameResult {
        self.action_counter += 1;
        if self.action_counter > 100 {
            self.action_counter = 0;
            state.create_caret(self.x, self.y, CaretType::Zzz, Direction::Left);
        }

        let anim = if self.direction == Direction::Left { 0 } else { 1 };

        self.anim_rect = state.constants.npc.n131_puppy_sleeping[anim];

        Ok(())
    }

    pub(crate) fn tick_n143_jenka_collapsed(&mut self, state: &mut SharedGameState) -> GameResult {
        let anim = if self.direction == Direction::Left { 0 } else { 1 };

        self.anim_rect = state.constants.npc.n143_jenka_collapsed[anim];

        Ok(())
    }
}
