use raylib::prelude::*;

pub struct TextureManager {
    rl: *mut RaylibHandle,
    thread: *const RaylibThread,
    pub textures: std::collections::HashMap<String, Texture2D>,
}

impl TextureManager {
    pub fn new( rl: *mut RaylibHandle, thread: *const RaylibThread) -> TextureManager {
        TextureManager {
            rl: rl,
            thread: thread,
            textures: std::collections::HashMap::new(),
        }
    }

    pub fn load_texture(&mut self, name: &str, path: &str) -> Result<(), String> {
        let rl = unsafe { &mut *self.rl };
        let thread = unsafe { &*self.thread };
        match rl.load_texture(thread, path) {
            Ok(texture) => {
                self.textures.insert(name.to_string(), texture);
                Ok(())
            },
            Err(e) => {
                let err = format!("---> Error loading texture {}: {}", path, e);
                eprintln!("{}", err);
                Err(err)
            },
        }
    }

    pub fn get_texture(&self, name: &str) -> Result<*mut Texture2D, String> {
        match self.textures.get(&name.to_string()) {
            Some(texture) => {
                // essa conversão é segura pois o ponteiro é apenas lido
                let texture = texture as *const Texture2D as *mut Texture2D;
                Ok(texture)
            }
            None => {
                let err = format!("---> Texture {} has not been loaded", name);
                eprintln!("{}", err);
                Err(err)
            },
        }
    }
}