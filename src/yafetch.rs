use mlua::prelude::*;

use crate::modules;

pub struct Yafetch {
    pub lua: Lua,
}

impl Yafetch {
    /// run yafetch with the given configuration file, or say why it could not be run
    pub fn run(&self, path: &std::path::Path) -> Result<(), String> {
        let source = match std::fs::read_to_string(path) {
            Ok(source) => source,
            Err(error) => return Err(format!("{}: {error}", path.display())),
        };
        let Err(error) = self.lua.load(&source).exec() else {
            return Ok(());
        };
        Err(format!("{}: {error}", path.display()))
    }

    // TODO arch

    fn host(&self) -> mlua::Result<mlua::Function> {
        self.lua
            .create_function(|_, _: ()| Ok(modules::host::get()))
    }

    fn cpu(&self) -> mlua::Result<mlua::Function> {
        self.lua.create_function(|_, _: ()| Ok(modules::cpu::get()))
    }

    fn os(&self) -> mlua::Result<mlua::Function> {
        self.lua.create_function(|_, _: ()| Ok(modules::os::get()))
    }

    fn uptime(&self) -> mlua::Result<mlua::Function> {
        self.lua
            .create_function(|_, _: ()| Ok(modules::uptime::get()))
    }

    fn user(&self) -> mlua::Result<mlua::Function> {
        self.lua
            .create_function(|_, _: ()| Ok(modules::user::get()))
    }

    fn hostname(&self) -> mlua::Result<mlua::Function> {
        self.lua
            .create_function(|_, _: ()| Ok(modules::hostname::get()))
    }

    fn local_ip(&self) -> mlua::Result<mlua::Function> {
        self.lua
            .create_function(|_, _: ()| Ok(modules::local_ip::get()))
    }

    fn battery(&self) -> mlua::Result<mlua::Function> {
        self.lua
            .create_function(|_, _: ()| Ok(modules::battery::get()))
    }

    fn disk_total(&self) -> mlua::Result<mlua::Function> {
        self.lua
            .create_function(|_, path: String| Ok(modules::disk::get_total(path)))
    }

    fn disk_free(&self) -> mlua::Result<mlua::Function> {
        self.lua
            .create_function(|_, path: String| Ok(modules::disk::get_free(path)))
    }

    fn mem_used(&self) -> mlua::Result<mlua::Function> {
        self.lua
            .create_function(|_, _: ()| Ok(modules::mem::get_used()))
    }

    fn mem_total(&self) -> mlua::Result<mlua::Function> {
        self.lua
            .create_function(|_, _: ()| Ok(modules::mem::get_total()))
    }

    fn current_datetime(&self) -> mlua::Result<mlua::Function> {
        self.lua
            .create_function(|_, _: ()| Ok(modules::datetime::get()))
    }

    pub fn register(&self) {
        let globals = self.lua.globals();
        let exports = self.lua.create_table().unwrap();

        // host

        exports
            .set("host", self.host().unwrap())
            .expect("could not register host function");

        exports
            .set("uptime", self.uptime().unwrap())
            .expect("could not register uptime function");

        exports
            .set("cpu", self.cpu().unwrap())
            .expect("could not register cpu function");

        exports
            .set("user", self.user().unwrap())
            .expect("could not register user function");

        exports
            .set("hostname", self.hostname().unwrap())
            .expect("could not register hostname function");

        exports
            .set("local_ip", self.local_ip().unwrap())
            .expect("could not register local_ip function");

        exports
            .set("battery", self.battery().unwrap())
            .expect("could not register battery function");

        exports
            .set("os", self.os().unwrap())
            .expect("could not register os function");

        //
        // mem
        //

        exports
            .set("mem_used", self.mem_used().unwrap())
            .expect("could not register mem_used function");

        exports
            .set("mem_total", self.mem_total().unwrap())
            .expect("could not register mem_total function");

        //
        // disk
        //

        exports
            .set("disk_free", self.disk_free().unwrap())
            .expect("could not register disk_free function");

        exports
            .set("disk_total", self.disk_total().unwrap())
            .expect("could not register disk_total function");

        //
        // other
        //
        exports
            .set("current_datetime", self.current_datetime().unwrap())
            .expect("could not register current_datetime function");

        globals.set("yafetch", exports).unwrap();
    }
}
