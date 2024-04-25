const std = @import("std");
const os = @import("os");
const sdl = @cImport({
    @cInclude("SDL2/SDL.h");
});

fn panicExit(thing: anytype, message: []const u8) void {
    // Checks existence of thing, and quits program
    if (!(&thing)) {
        sdl.SDL_Log(message + ":", sdl.SDL_GetError());
        sdl.SDL_Quit();

        game.running = false;
        os.exit();
    }
}

const Display = struct {
    width: i32,
    height: i32,
    window: *sdl.SDL_Window,
    renderer: *sdl.SDL_Renderer,

    fn init() Display {
        var self = Display{
            .width = 1280,
            .height = 720,
            .window = sdl.SDL_CreateWindow(
                "war",
                sdl.SDL_WINDOWPOS_UNDEFINED,
                sdl.SDL_WINDOWPOS_UNDEFINED,
                1280,
                720,
                null,
            ),
        };
        panicExit(&self.window, "Failed to create window");

        self.renderer = sdl.SDL_CreateRenderer(self.window, null, null);
        panicExit(&self.renderer, "Failed to create renderer");

        return self;
    }
};

const Game = struct {
    running: bool,

    fn init() Game {
        return Game{
            .running = true,
        };
    }
};

var display = Display.init();
var game = Game.init();

pub fn main() !void {
    if (sdl.SDL_Init(sdl.SDL_INIT_VIDEO) != 0) {
        sdl.SDL_Log("Failed to initialize SDL: %s", sdl.SDL_GetError());
        return error.SDLInitializationFailed;
    }
    defer sdl.SDL_Quit();
    defer sdl.SDL_DestroyWindow(display.window);

    while (game.running) {
        var events: sdl.SDL_Event = undefined;
        while (sdl.SDL_PollEvent(&events) != 0) {
            switch (events.type) {
                sdl.SDL_QUIT => break,
                else => {},
            }
        }

        sdl.SDL_RenderClear(display.renderer);
        sdl.SDL_RenderPresent(display.renderer);
    }
}
