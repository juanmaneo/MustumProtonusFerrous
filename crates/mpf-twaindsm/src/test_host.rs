//! Shared fake host and data source for the oracle and edge tests. The configuration and the
//! call logs live in one `World` so a test can change a result after the Source Manager has
//! taken ownership of the host.

use crate::codes::*;
use crate::data::Data;
use crate::devices::Device;
use crate::dsm::{Dsm, Host};
use crate::identity::Identity;
use crate::source::DataSource;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub struct World {
    pub get_groups: u32,
    pub get_rc: u16,
    pub open_rc: u16,
    pub close_rc: u16,
    pub fwd_rc: u16,
    pub callback_rc: u16,
    pub peek_found: bool,
    pub register_message: u32,
    pub selection: Option<usize>,
    pub ds_calls: Vec<(u32, u16, u16)>,
    pub loads: Vec<String>,
    pub callbacks: Vec<(usize, usize, Option<u32>, u16)>,
    pub peeks: Vec<(usize, u32)>,
    pub offers: Vec<(usize, Vec<String>)>,
}

pub fn shared_world() -> Rc<RefCell<World>> {
    let world = World {
        register_message: 0xc001,
        ..World::default()
    };
    Rc::new(RefCell::new(world))
}

pub struct FakeHost {
    world: Rc<RefCell<World>>,
}

pub struct FakeSource {
    world: Rc<RefCell<World>>,
}

impl FakeHost {
    pub fn new(world: Rc<RefCell<World>>) -> Self {
        Self { world }
    }
}

impl Host for FakeHost {
    type Source = FakeSource;

    fn load_source(&mut self, modname: &str) -> Option<FakeSource> {
        self.world.borrow_mut().loads.push(modname.to_owned());
        if modname == "bad.ds" {
            None
        } else {
            Some(FakeSource {
                world: self.world.clone(),
            })
        }
    }

    fn call_app_callback(
        &mut self,
        proc_addr: usize,
        ref_con: usize,
        _origin: &Identity,
        dest_id: Option<u32>,
        msg: u16,
    ) -> u16 {
        let mut world = self.world.borrow_mut();
        world.callbacks.push((proc_addr, ref_con, dest_id, msg));
        world.callback_rc
    }

    fn register_event_message(&mut self) -> u32 {
        self.world.borrow().register_message
    }

    fn peek_event_message(&mut self, hwnd: usize, message: u32) -> bool {
        let mut world = self.world.borrow_mut();
        world.peeks.push((hwnd, message));
        world.peek_found
    }

    fn user_select(&mut self, parent: usize, choices: &[Identity]) -> Option<usize> {
        let mut world = self.world.borrow_mut();
        let names = choices
            .iter()
            .map(|choice| choice.product_name.clone())
            .collect();
        world.offers.push((parent, names));
        world.selection
    }
}

impl DataSource for FakeSource {
    fn entry(
        &mut self,
        _origin: Option<&Identity>,
        dg: u32,
        dat: u16,
        msg: u16,
        data: Data<'_>,
    ) -> u16 {
        let mut world = self.world.borrow_mut();
        world.ds_calls.push((dg, dat, msg));
        match (dat, msg) {
            (DAT_IDENTITY, MSG_GET) => {
                if let Data::Identity(identity) = data {
                    identity.supported_groups = world.get_groups;
                }
                world.get_rc
            }
            (DAT_ENTRYPOINT, MSG_SET) => TWRC_SUCCESS,
            (DAT_IDENTITY, MSG_OPENDS) => world.open_rc,
            (DAT_IDENTITY, MSG_CLOSEDS) => world.close_rc,
            _ => world.fwd_rc,
        }
    }
}

pub fn ident(id: u32, groups: u32, name: &str) -> Identity {
    Identity::new(id, groups, name)
}

pub fn device(modname: &str, groups: u32, name: &str) -> Device {
    Device {
        modname: modname.to_owned(),
        identity: Identity::new(0, groups, name),
    }
}

pub fn new_dsm(devices: Vec<Device>) -> (Dsm<FakeHost>, Rc<RefCell<World>>) {
    let world = shared_world();
    let dsm = Dsm::new(FakeHost::new(world.clone()), devices);
    (dsm, world)
}

pub fn adopt(dsm: &mut Dsm<FakeHost>, world: &Rc<RefCell<World>>, id: u32, name: &str) {
    dsm.adopt_source(
        ident(id, 0, name),
        FakeSource {
            world: world.clone(),
        },
    );
}
