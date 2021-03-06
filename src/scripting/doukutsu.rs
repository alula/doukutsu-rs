use lua_ffi::ffi::luaL_Reg;
use lua_ffi::{c_int, LuaObject, State};

use crate::scripting::LuaScriptingState;

pub struct Doukutsu {
    pub ptr: *mut LuaScriptingState,
}

#[allow(unused)]
impl Doukutsu {
    pub fn new(ptr: *mut LuaScriptingState) -> Doukutsu {
        Doukutsu { ptr }
    }

    unsafe fn lua_play_sfx(&self, state: &mut State) -> c_int {
        if let Some(index) = state.to_int(2) {
            let game_state = &mut (*(*self.ptr).state_ptr);

            game_state.sound_manager.play_sfx(index as u8);
        }

        0
    }

    unsafe fn lua_play_song(&self, state: &mut State) -> c_int {
        if let Some(index) = state.to_int(2) {
            let game_state = &mut (*(*self.ptr).state_ptr);
            let ctx = &mut (*(*self.ptr).ctx_ptr);

            let _ =
                game_state.sound_manager.play_song(index as usize, &game_state.constants, &game_state.settings, ctx);
        }

        0
    }

    unsafe fn lua_set_setting(&self, state: &mut State) -> c_int {
        if let Some(index) = state.to_int(2) {
            let game_state = &mut (*(*self.ptr).state_ptr);

            state.push(game_state.get_flag(index.max(0) as usize));
        } else {
            state.push_nil();
        }

        1
    }

    unsafe fn lua_get_flag(&self, state: &mut State) -> c_int {
        if let Some(index) = state.to_int(2) {
            let game_state = &mut (*(*self.ptr).state_ptr);

            state.push(game_state.get_flag(index.max(0) as usize));
        } else {
            state.push_nil();
        }

        1
    }
}

impl LuaObject for Doukutsu {
    fn name() -> *const i8 {
        c_str!("Doukutsu")
    }

    fn lua_fns() -> Vec<luaL_Reg> {
        vec![
            lua_method!("playSfx", Doukutsu, Doukutsu::lua_play_sfx),
            lua_method!("playSong", Doukutsu, Doukutsu::lua_play_song),
            lua_method!("getFlag", Doukutsu, Doukutsu::lua_get_flag),
        ]
    }
}
