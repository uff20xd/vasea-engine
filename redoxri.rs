#![allow(dead_code)]
#![allow(unused_mut)]
/// Welcome to Redoxri

use std::{
    process::{
        Command,
        exit,
    },
    fs,
    time::{
        Duration,
    },
    path::{
        Path,
    },
    fmt::{
        Debug
    },
};

pub type Cmd = Command;
pub type RxiError = Box<dyn std::error::Error>;

static mut FULL_MUTE: bool = false;

#[derive(Clone, Debug)]
pub struct Redoxri {
    settings: Vec<String>,
    pub args: Vec<String>,
    mcule: Mcule,
}

impl Redoxri {
    pub fn new<T>(in_settings: &[&T]) -> Self 
    where T: ?Sized + AsRef<str> + Debug {
        let args: Vec<String> = std::env::args().collect();
        let mut compile_step: Vec<&str> = Vec::new();
        let main_file_name = args[0].clone() + ".rs";
        let mut force_compile = false;
        compile_step.push("rustc");
        compile_step.push(&main_file_name);
        compile_step.push("--cfg");
        compile_step.push("bootstrapped");

        let mut settings = Vec::new();
        for setting in in_settings {
            if setting.as_ref() != "" {
                settings.push(setting.as_ref().to_string());
            }
        }

        if args.len() > 1 {
            if Self::parse_args_to_settings(&args, &mut settings) {force_compile = true}
        }

        for setting in &settings {
            compile_step.push(setting);
        }

        let mut mcule = Mcule::new("redoxri_script", &args[0])
            .with(&[
                main_file_name.clone().into(),
                "redoxri.rs".into(),
            ])
            .add_step(&compile_step[..]);

        #[cfg(mute_self)]
        mcule.mute();

        let mut me = Self {
            settings,
            args,
            mcule,
        };
        _ = me.self_compile(force_compile);
        me
    }

    fn parse_args_to_settings(args: &Vec<String>, settings: &mut Vec<String>) -> bool{
        let start_index = 1;
        let setting = match args[start_index].as_str() {
            "rebuild" => {"rebuild_all"},
            "self" => {"self_build"},
            "clean" => {"clean"},
            "get" => {"get_pkgs"},
            "run" => {"run"},
            _ => {""},
        };
        if setting != "" { 
            settings.push("--cfg".to_owned()); settings.push(setting.to_owned());
            return true;
        }
        false
    }

    pub fn get_info() -> Vec<(bool, Box<Path>)> {
        todo!("Implement a way to get all mcules into the output form")
    }

    pub fn self_compile(&mut self, always_compile: bool) -> Result<(), Box<dyn std::error::Error>> {

        #[cfg(isolate)]
        {
        }

        #[cfg(any(clean, run))]
        {
            self.mcule.mute();
            unsafe { FULL_MUTE = true; }
            self.mcule.report_and_just_compile();
        }

        #[cfg(not(bootstrapped))]
        {
            self.mcule.report_and_just_compile();
            //println!("Not Bootstrapped");
        }

        if always_compile {
            self.mcule.mute();
            unsafe { FULL_MUTE = true; }
            self.mcule.report_and_just_compile();
            unsafe { FULL_MUTE = false; }
            self.mcule.unmute();
            self.mcule.required_run();
            exit(0)
        }

        #[cfg(not(clean))]
        if !self.mcule.is_up_to_date() && !always_compile {
            println!("Detected Change!");
            println!("Recompiling build script...");
            self.mcule.report_and_just_compile();
            if !self.mcule.is_successful() {
                println!("Recompilation Failed!");
                println!("Exiting...");
                exit(2)
            }
            println!("Recompilation Successful!");
            println!("Executing new build script...");
            self.mcule.required_run();
            exit(0);
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Mcule {
    pub name: String,
    pub outpath: String,
    inputs: Vec<Mcule>,
    recipe: Vec<Vec<String>>,
    last_changed: (),
    success: bool,
    status_chain: Vec<i32>,
    mute: bool,
}


impl Mcule {
    pub fn new<T, A>(name: &T, outpath: &A) -> Self 
    where T: ?Sized + AsRef<str> + Debug,
    A: ?Sized + AsRef<str> + Debug {

        let mut outpath = outpath.as_ref().to_owned();

        if &outpath[0..1] == "/" {
            panic!("Please dont use absolute paths as the Outpath of a generative Mcule, as it destroys compatibility!
In Mcule: {}; with outpath: {}", name.as_ref(), outpath);
        }

        #[cfg(isolate)]
        if &outpath[0..2] != "./out" {
            outpath = "./out/".to_owned() + &outpath;
        }

        #[cfg(not(isolate))]
        if &outpath[0..2] != "./" {
            outpath = "./".to_owned() + &outpath;
        }

        Self::raw (
            // Name
            name.as_ref().to_owned(),

            // Outpath
            outpath,

            // Inputs
            Vec::new(),

            // Recipe
            Vec::new(),

            // Last changed here

            // Success
            true,

            // Mute
            #[cfg(mute_on_default)]
            true,

            #[cfg(not(mute_on_default))]
            false,

            // Status chain
            Vec::new(),
        )
    }
    pub fn raw(
        name: String,
        outpath: String,
        inputs: Vec<Mcule>,
        recipe: Vec<Vec<String>>,
        success: bool,
        mute: bool,
        status_chain: Vec<i32>,
    ) -> Self {
        Self {
            name,
            outpath,
            inputs,
            recipe,
            last_changed: (),
            success,
            mute,
            status_chain,
        }
    }

    pub fn with(mut self, inputs: &[Mcule]) -> Self {
        for i in inputs {
            self.inputs.push(i.clone());
        }
        self
    }

    pub fn is_up_to_date(&self) -> bool {
        let _last_change = match self.get_comp_date() {
            Ok(time_since_last_change) => {
                for i in &self.inputs {
                    let comp_date_i = i.get_comp_date().unwrap();
                    if comp_date_i < time_since_last_change {
                        return false;
                    }
                }
            },
            Err(_) => {
                return false;
            },
        };
        true
    }

    fn get_comp_date(&self) -> Result<Duration, Box<dyn std::error::Error>> {
        let this_file = fs::File::open(&self.outpath)?;

        let time = this_file.metadata()?.modified()?.elapsed()?;

        Ok(time)
    }

    pub fn compile(&mut self) -> Self {
        let mut need_to_compile = false;

        #[cfg(not(clean))]
        let _last_change = match self.get_comp_date() {
            Ok(time_since_last_change) => {
                for i in &self.inputs {
                    i.clone().compile();
                    let comp_date_i = i.get_comp_date().unwrap();
                    if comp_date_i < time_since_last_change {
                        need_to_compile = true;
                    }
                }
            },

            Err(_) => {
                need_to_compile = true;
            },
        };


        #[cfg(clean)]
        {
            if self.recipe.len() == 0 { return self.to_owned(); }
            let file_to_delete = Path::new(&self.outpath);
            if file_to_delete.is_file() {
                println!("Cleaning: {} at {}", &self.name, &self.outpath);
                fs::remove_file(file_to_delete).unwrap();
            }
            return self.to_owned();
        }


        #[cfg(not(clean))]
        if need_to_compile {
            #[cfg(debug)]
            println!("Compiling {}", &self.outpath);
            self.status_chain = self.just_compile();
            let mut success = true;
            for i in self.status_chain.clone() {
                if i != 0 {
                    success = false;
                }
            }
            self.success = success;

            #[cfg(unmute_on_fail)]
            if !self.is_successful() {
                self.mute = false;
                _ = self.just_compile();
            }

        }
        self.to_owned()
        //Ok(())
    }

    pub fn just_compile(&self) -> Vec<i32> {
        let mut recipe = self.recipe.clone();
        let mut output_chain = Vec::new();
        for step in &mut recipe {
            let mut cmd = Command::new(step.remove(0));
            for command in step {
                _ = cmd.arg(&command);
            }

            if self.mute {
                unsafe { if !FULL_MUTE {
                    println!("Muted Compilation of: {} {}", &self.name, &self.outpath);
                } }
                _ = match cmd.output() {
                    Ok(out) => {
                        if let Some(excode) = out.status.code() {
                            output_chain.push(excode);
                        }
                        else {output_chain.push(-0x7999_9998_i32);}
                    },
                    Err(_) => {
                        output_chain.push(-0x7999_9997_i32);
                    }
                };
            }
            else {
                //println!("unmute");
                _ = match cmd.status() {
                    Ok(exit_code) => {
                        if let Some(excode) = exit_code.code() {
                            output_chain.push(excode);
                        }
                        else {output_chain.push(-0x7999_9999_i32);}
                    },
                    Err(_) => {
                        output_chain.push(-0x80000000_i32);
                    },
                };
            }
        }
        output_chain

    }

    fn report_and_just_compile(&mut self) -> Self {
        self.status_chain = self.just_compile();
        let mut success = true;
        for i in self.status_chain.clone() {
            if i != 0 {
                success = false;
            }
        }
        self.success = success;

        #[cfg(unmute_on_fail)]
        if !self.is_successful() {
            self.mute = false;
            _ = self.just_compile();
        }
        self.to_owned()
    }

    pub fn add_step<T>(mut self, step: &[&T]) -> Self 
    where T: ?Sized + AsRef<str> + Debug {
        let mut new_step: Vec<String> = Vec::new();
        for arg in step {
            if arg.as_ref() == "$out" {
                new_step.push(self.outpath.clone());
            }
            else {new_step.push(arg.as_ref().to_string());}
        }
        self.recipe.push(new_step);
        self
    }

    pub fn copy_to(&self, to: &str) -> &Self {
        _ = fs::copy(self.outpath.clone(), to);
        self
    }

    pub fn required_run(&self) -> Self {
        let mut cmd = Command::new(self.outpath.clone());
        if self.mute {
            _ = cmd.output();
        } else {
            _ = cmd.status();
        }
        self.to_owned()
    }

    pub fn run(&self) -> Self {
        if cfg!(run) {
            let mut cmd = Command::new(self.outpath.clone());
            if self.mute {
                _ = cmd.output();
            } else {
                _ = cmd.status();
            }
        }
        self.to_owned()
    }

    pub fn mute(&mut self) -> Self {
        self.mute = true;
        self.to_owned()
    }

    pub fn unmute(&mut self) -> Self {
        self.mute = false;
        self.to_owned()
    }

    #[inline(always)]
    pub fn is_successful(&self) -> bool {
        let mut success = self.success;
        for i in self.inputs.clone() {
            if !i.is_successful() {
                success = false;
            }
        }
        success
    }
}

impl From<&str> for Mcule {
    fn from(item: &str) -> Self {
        Self {
            name: "".to_owned(),
            outpath: item.to_owned(),
            inputs: Vec::new(),
            recipe: Vec::new(),
            last_changed: (),
            success: true,
            mute: false,
            status_chain: Vec::new(),
        }
    }
}

impl From<String> for Mcule {
    fn from(item: String) -> Self {
        Self {
            name: "".to_owned(),
            outpath: item,
            inputs: Vec::new(),
            recipe: Vec::new(),
            last_changed: (),
            success: true,
            mute: false,
            status_chain: Vec::new(),
        }
    }
}

pub struct CMcule {
    file: String,
    deps: (),
}

pub enum RustCrateType {
    ProcMacro,
    Bin,
    Lib,
    Rlib,
    Empty,
}

pub struct RustMcule {
    name: String,
    crate_type: RustCrateType,
    outpath: String,
    src: String,
    root: String,
    file: String,
    flags: Vec<String>,
    deps: Vec<Mcule>,
    pre_steps: Vec<Vec<String>>,
    post_steps: Vec<Vec<String>>,
}

impl RustMcule {
    pub fn new(name: &str, root: &str) -> Self {
        Self {
            name: name.to_owned(), 
            crate_type: RustCrateType::Lib,
            outpath: "".to_owned(),
            src: "src".to_owned(),
            root: root.to_owned(),
            file: "main.rs".to_owned(),
            deps: Vec::new(),
            flags: Vec::new(),
            pre_steps: Vec::new(),
            post_steps: Vec::new(),
        }
    }

    pub fn finish(&self) -> Mcule {
        "".into()
    }

    pub fn make_lib(&mut self) -> &mut Self {
        self.crate_type = match &self.crate_type {
            RustCrateType::Empty => {panic!("Cant change an empty crate to a library! (fn make_lib)")},
            _ => { RustCrateType::Lib }
        };
        self
    }

    pub fn make_bin(&mut self) -> &mut Self {
        self.crate_type = match &self.crate_type {
            RustCrateType::Empty => {panic!("Cant change an empty crate to a library! (fn make_bin)")},
            _ => { RustCrateType::Bin }
        };
        self
    }

    pub fn set_root(&mut self, new_root: &str) -> &mut Self {
        self.root = new_root.to_owned();
        self
    }

    pub fn set_src(&mut self, new_src: &str) -> &mut Self {
        self.src = new_src.to_owned();
        self
    }

    pub fn set_main(&mut self, new_main: &str) -> &mut Self {
        self.file = new_main.to_owned();
        self
    }

    pub fn add_pre_step<T>(&mut self, step: &[&T]) -> &mut Self 
    where T: ?Sized + AsRef<str> + Debug {
        let mut pre_step = Vec::new();
        for i in step {
            pre_step.push(i.as_ref().to_string());
        }
        self.pre_steps.push(pre_step);
        self
    }
}

mod tja {
}
