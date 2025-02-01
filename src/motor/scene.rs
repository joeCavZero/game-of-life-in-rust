use super::{motor::{Motor, RenderTextureModeDrawHandle}, widget::Widget};

pub trait Scene {
    fn init(&mut self, motor: &mut Motor);
    fn update(&mut self, motor: &mut Motor);
    fn render(&mut self, motor: &mut Motor, d: &mut RenderTextureModeDrawHandle<'_>);  

    fn get_base_scene(&mut self) -> &mut BaseScene;
}

pub struct BaseScene {
    pub motor: Option<*mut Motor>,
    pub widgets: Vec<Box<dyn Widget>>,
}

impl BaseScene {
    pub fn new() -> BaseScene {
        BaseScene {
            motor: None,
            widgets: Vec::new(),
        }
    }

    pub fn init(&mut self, motor: &mut Motor) {
        self.motor = Some(motor);
    }

    pub fn update(&mut self, motor: &mut Motor) {
        for wid in self.widgets.iter_mut() {
            wid.update(motor);
        }
    }

    pub fn render(&mut self, motor: &mut Motor, d: &mut RenderTextureModeDrawHandle<'_>) {
        
        for wid in self.widgets.iter_mut() {
            wid.render(motor, d);
        }
        
    }

    pub fn add_widget(&mut self, mut wid: Box<dyn Widget>) {
        
        unsafe { wid.init(&mut *self.motor.unwrap()) }
        self.widgets.push(wid);

        self.widgets.sort_by(|a, b| a.get_base_widget().get_layer().cmp(&b.get_base_widget().get_layer()));

    }


    pub fn get_widget_by_name(&mut self, name: &str) -> Option<&mut Box<dyn Widget>> {
        for wid in self.widgets.iter_mut() {
            if wid.get_base_widget().get_name() == name {
                return Some(wid);
            }
        }
        None
    }
}