#include <stdio.h>

#include <lua.h>
#include <lauxlib.h>
#include <lualib.h>

void LUA_ON_UPDATE(const char* script) {
    lua_State* L;
    L = luaL_newstate();
    luaL_openlibs(L);

    //legion.h function
    /* LUA_NEW_WORLD(L); */

    luaL_dofile(L, script);
     
    lua_getglobal(L, "on_update");

    if(lua_pcall(L, 0, 0, 0) != 0) {
        printf("error running function `on_update`: %s\n", lua_tostring(L, -1));
    }

    lua_close(L);
}

void LUA_ON_CREATE(const char* script) {
    lua_State* L;
    L = luaL_newstate();
    luaL_openlibs(L);

    luaL_dofile(L, script);
     
    lua_getglobal(L, "on_create");

    if(lua_pcall(L, 0, 0, 0) != 0) {
        printf("error running function `on_create`: %s\n", lua_tostring(L, -1));
    }

    lua_close(L);
}
