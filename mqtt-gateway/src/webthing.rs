use super::{config, EventType, Package, Service};
use log::debug;
use serde_json::json;
use std::collections::HashMap;
use std::sync::{Arc, RwLock, Weak};
use std::thread;
use std::thread::JoinHandle;
use std::time::{Duration, Instant};
use uuid::Uuid;
use webthing::property::ValueForwarder;
use webthing::server::ActionGenerator;
use webthing::{
    Action, BaseAction, BaseEvent, BaseProperty, BaseThing, Event, Thing, ThingsType,
    WebThingServer,
};

struct OnValueForwarder;
struct Generator;

impl ValueForwarder for OnValueForwarder {
    fn set_value(&mut self, value: serde_json::Value) -> Result<serde_json::Value, &'static str> {
        println!("On-State is now {}", value);
        Ok(value)
    }
}

pub struct ToggleAction(BaseAction);

impl ToggleAction {
    fn new(
        input: Option<serde_json::Map<String, serde_json::Value>>,
        thing: Weak<RwLock<Box<Thing>>>,
    ) -> ToggleAction {
        ToggleAction(BaseAction::new(
            Uuid::new_v4().to_string(),
            "toggle".to_owned(),
            input,
            thing,
        ))
    }
}

impl Action for ToggleAction {
    fn set_href_prefix(&mut self, prefix: String) {
        self.0.set_href_prefix(prefix)
    }

    fn get_id(&self) -> String {
        self.0.get_id()
    }

    fn get_name(&self) -> String {
        self.0.get_name()
    }

    fn get_href(&self) -> String {
        self.0.get_href()
    }

    fn get_status(&self) -> String {
        self.0.get_status()
    }

    fn get_time_requested(&self) -> String {
        self.0.get_time_requested()
    }

    fn get_time_completed(&self) -> Option<String> {
        self.0.get_time_completed()
    }

    fn get_input(&self) -> Option<serde_json::Map<String, serde_json::Value>> {
        self.0.get_input()
    }

    fn get_thing(&self) -> Option<Arc<RwLock<Box<Thing>>>> {
        self.0.get_thing()
    }

    fn set_status(&mut self, status: String) {
        self.0.set_status(status)
    }

    fn perform_action(&mut self) {
        let thing = self.get_thing();
        if thing.is_none() {
            return;
        }

        let thing = thing.unwrap();
        let name = self.get_name();
        let id = self.get_id();

        thread::spawn(move || {
            debug!("Performing TOGGLE action!");
            let mut thing = thing.write().unwrap();
            let prev_state = thing
                .get_property("on".to_owned())
                .unwrap()
                .as_bool()
                .unwrap();
            thing
                .set_property("on".to_owned(), json!(!prev_state))
                .expect("Can't set property?");
            thing.finish_action(name, id);
        });
    }

    fn start(&mut self) {
        self.0.start()
    }

    fn cancel(&mut self) {
        self.0.cancel()
    }

    fn finish(&mut self) {
        self.0.finish()
    }
}

impl ActionGenerator for Generator {
    fn generate(
        &self,
        thing: Weak<RwLock<Box<Thing>>>,
        name: String,
        input: Option<&serde_json::Value>,
    ) -> Option<Box<Action>> {
        let input = match input {
            Some(v) => match v.as_object() {
                Some(o) => Some(o.clone()),
                None => None,
            },
            None => None,
        };

        //        thing.upgrade().unwrap().write().unwrap().property_notify("on".to_owned(), json!(true));

        let name: &str = &name;
        match name {
            "toggle" => Some(Box::new(ToggleAction::new(input, thing))),
            _ => None,
        }
    }
}

pub struct PressedEvent(BaseEvent);

impl PressedEvent {
    fn new() -> PressedEvent {
        PressedEvent(BaseEvent::new("pressed".to_owned(), None))
    }
}

impl Event for PressedEvent {
    fn get_name(&self) -> String {
        self.0.get_name()
    }

    fn get_data(&self) -> Option<serde_json::Value> {
        self.0.get_data()
    }

    fn get_time(&self) -> String {
        self.0.get_time()
    }
}

struct WebThingService {
    thread: Option<JoinHandle<()>>,
    buttons: HashMap<u8, Arc<RwLock<Box<Thing + 'static>>>>,
    lights: HashMap<u8, Arc<RwLock<Box<Thing + 'static>>>>,
    last_button_state: [Option<(Instant, bool)>; 32],
}

impl WebThingService {
    fn new(
        thread: JoinHandle<()>,
        buttons: HashMap<u8, Arc<RwLock<Box<Thing + 'static>>>>,
        lights: HashMap<u8, Arc<RwLock<Box<Thing + 'static>>>>,
    ) -> WebThingService {
        WebThingService {
            thread: Some(thread),
            buttons,
            lights,
            last_button_state: [None; 32],
        }
    }
}

impl Service for WebThingService {
    fn handle_package(&mut self, package: &Package) {
        for i in 0..31 {
            let light_state = package.state & (1 << i) != 0;
            let light = self.lights.get(&i);
            if light.is_some() {
                let mut light = light.unwrap().write().unwrap();
                let prev_state = light
                    .get_property("on".to_owned())
                    .unwrap()
                    .as_bool()
                    .unwrap();
                if light_state != prev_state {
                    light
                        .set_property("on".to_owned(), json!(light_state))
                        .expect("Can't set property?");
                }
            }
        }
        for event in &package.events {
            if event.valid() {
                let last_state = self.last_button_state[event.button() as usize];
                let state = event.event_type() == EventType::PressStart;
                let now = Instant::now();
                let button = self.buttons.get(&event.button());
                if button.is_some() {
                    let mut button = button.unwrap().write().unwrap();
                    if let Some(last_state) = last_state {
                        if last_state.1 && !state {
                            let duration = now - last_state.0;
                            if duration < Duration::from_millis(500) {
                                button.add_event(Box::new(PressedEvent::new()));
                            }
                        }
                    }
                    button.set_property("pushed".to_owned(), json!(state))
                        .expect("Can't set property?");
                }
                self.last_button_state[event.button() as usize] = Some((now, state));
            }
        }
    }

    fn join(&mut self) {
        if self.thread.is_some() {
            self.thread.take().unwrap().join().expect("Can't join?");
        }
    }
}

pub fn init(
    config: &config::Config,
) -> Result<Option<Box<dyn Service>>, Box<dyn std::error::Error + 'static>> {
    if !config.webthing.enabled {
        return Ok(None);
    }

    let mut things: Vec<Arc<RwLock<Box<Thing + 'static>>>> = Vec::new();
    let mut lights: HashMap<u8, Arc<RwLock<Box<Thing + 'static>>>> = HashMap::new();
    let mut buttons: HashMap<u8, Arc<RwLock<Box<Thing + 'static>>>> = HashMap::new();

    // FIXME: implement buttons
    for (button_id, button_config) in &config.buttons {
        let mut thing = BaseThing::new(
            format!("http://abittechnical.com/standaertha/buttons/{}", button_id),
            if button_config.name.is_empty() {
                button_id.clone()
            } else {
                button_config.name.clone()
            },
            Some(vec!["PushButton".to_owned()]),
            None,
        );
        let pushed_desc = json!({
            "@type": "PushedProperty",
            "title": "Pushed",
            "type": "boolean",
            "description": "Whether the button is currently being pushed",
        });
        let pushed_desc = pushed_desc.as_object().unwrap().clone();
        thing.add_property(Box::new(BaseProperty::new(
            "pushed".to_owned(),
            json!(false),
            None,
            Some(pushed_desc),
        )));
        let pressed_event_desc = json!({
            "description": "The button has been pressed",
        });
        let pressed_event_desc = pressed_event_desc.as_object().unwrap().clone();
        thing.add_available_event("pressed".to_owned(), pressed_event_desc);
        let thing: Arc<RwLock<Box<Thing + 'static>>> = Arc::new(RwLock::new(Box::new(thing)));
        things.push(thing.clone());
        buttons.insert(button_config.index, thing);
    }

    for (light_id, light_config) in &config.lights {
        let mut thing = BaseThing::new(
            format!("http://abittechnical.com/standaertha/lights/{}", light_id),
            if light_config.name.is_empty() {
                light_id.clone()
            } else {
                light_config.name.clone()
            },
            Some(vec!["OnOffSwitch".to_owned(), "Light".to_owned()]),
            None,
        );
        let on_description = json!({
            "@type": "OnOffProperty",
            "title": "On/Off",
            "type": "boolean",
            "description": "Whether the light is turned on",
        });
        let on_description = on_description.as_object().unwrap().clone();
        thing.add_property(Box::new(BaseProperty::new(
            "on".to_owned(),
            json!(false),
            Some(Box::new(OnValueForwarder)),
            Some(on_description),
        )));
        let toggle_metadata = json!({
            "title": "Toggle",
            "description": "Toggle the light",
        });
        let toggle_metadata = toggle_metadata.as_object().unwrap().clone();
        thing.add_available_action("toggle".to_owned(), toggle_metadata);
        let thing: Arc<RwLock<Box<Thing + 'static>>> = Arc::new(RwLock::new(Box::new(thing)));
        things.push(thing.clone());
        lights.insert(light_config.index, thing);
    }

    let handle = thread::spawn(move || {
        let mut server = WebThingServer::new(
            ThingsType::Multiple(things, "MyDevice".to_owned()),
            Some(8888),
            None,
            None,
            Box::new(Generator),
            None,
            None,
        );
        server.create();
        server.start();
    });

    Ok(Some(Box::new(WebThingService::new(
        handle, buttons, lights,
    ))))
}
