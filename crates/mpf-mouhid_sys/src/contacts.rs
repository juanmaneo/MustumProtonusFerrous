//! Contact tracking: `process_hid_report`, `add_contact` and `release_contacts`.

use std::mem;

use crate::{scale_value, ValueCaps};

const PAGE_GENERIC: u16 = 0x01;
const GENERIC_X: u16 = 0x30;
const GENERIC_Y: u16 = 0x31;
const PAGE_DIGITIZER: u16 = 0x0d;
const DIGITIZER_TIP_SWITCH: u16 = 0x42;
const DIGITIZER_CONTACT_ID: u16 = 0x51;
const DIGITIZER_CONTACT_COUNT: u16 = 0x54;

const POINTER_MESSAGE_FLAG_NEW: u16 = 0x0001;
const POINTER_MESSAGE_FLAG_INRANGE: u16 = 0x0002;
const POINTER_MESSAGE_FLAG_INCONTACT: u16 = 0x0004;
const POINTER_MESSAGE_FLAG_CONFIDENCE: u16 = 0x4000;

/// One tracked contact, `struct contact` in `main.c`. `id` keeps the full 32-bit value.
#[derive(Debug, PartialEq, Eq)]
pub struct Contact {
    pub id: u32,
    pub x: i32,
    pub y: i32,
}

/// The `WM_POINTER*` message sent for a contact (`input.hi.uMsg`).
#[derive(Debug, PartialEq, Eq)]
pub enum PointerKind {
    Down,
    Update,
    Up,
}

/// One `NtUserSendHardwareInput` call. `id` and `flags` are the `WORD` fields `wParamL` and
/// `wParamH`, and `x` and `y` are the two halves of `MAKELPARAM`.
#[derive(Debug, PartialEq, Eq)]
pub struct PointerEvent {
    pub kind: PointerKind,
    pub id: u16,
    pub flags: u16,
    pub x: u16,
    pub y: u16,
}

/// The per-collection caps that `process_hid_report` reads: `id_caps[i].LinkCollection`,
/// `x_caps[i]` and `y_caps[i]`.
pub struct ContactCaps {
    pub link_collection: u16,
    pub x: ValueCaps,
    pub y: ValueCaps,
}

/// The `HidP_GetUsageValue` and `HidP_GetUsages` calls that `process_hid_report` makes.
pub trait ReportReader {
    /// `HidP_GetUsageValue`: `None` unless the status is `HIDP_STATUS_SUCCESS`.
    fn usage_value(&self, page: u16, collection: u16, usage: u16) -> Option<u32>;

    /// `HidP_GetUsages` for a single usage: `None` unless the status is `HIDP_STATUS_SUCCESS`,
    /// otherwise the number of usages set.
    fn usages(&self, page: u16, collection: u16, usage: u16) -> Option<u32>;
}

/// The contact list of one device, `impl->contacts`, in list order.
#[derive(Default)]
pub struct ContactTracker {
    contacts: Vec<Contact>,
}

impl ContactTracker {
    pub fn contacts(&self) -> &[Contact] {
        &self.contacts
    }

    /// `process_hid_report`. Does nothing when the contact count cannot be read. Otherwise
    /// the previous contacts are matched against this report: matched ones are updated, and
    /// the rest are released.
    pub fn process_report<R: ReportReader>(
        &mut self,
        reader: &R,
        caps: &[ContactCaps],
        events: &mut Vec<PointerEvent>,
    ) {
        if reader
            .usage_value(PAGE_DIGITIZER, 0, DIGITIZER_CONTACT_COUNT)
            .is_none()
        {
            return;
        }

        let mut old_contacts = mem::take(&mut self.contacts);
        for cap in caps {
            let collection = cap.link_collection;
            if reader
                .usages(PAGE_DIGITIZER, collection, DIGITIZER_TIP_SWITCH)
                .unwrap_or(0)
                == 0
            {
                continue;
            }
            let Some(id) = reader.usage_value(PAGE_DIGITIZER, collection, DIGITIZER_CONTACT_ID)
            else {
                continue;
            };
            let Some(raw_x) = reader.usage_value(PAGE_GENERIC, collection, GENERIC_X) else {
                continue;
            };
            let x = scale_value(raw_x, &cap.x, 0, 65535);
            let Some(raw_y) = reader.usage_value(PAGE_GENERIC, collection, GENERIC_Y) else {
                continue;
            };
            let y = scale_value(raw_y, &cap.y, 0, 65535);
            self.add_contact(&mut old_contacts, id, x, y, events);
        }
        release_contacts(old_contacts, events);
    }

    /// Releases every tracked contact, as the device-removal path does.
    pub fn release_all(&mut self, events: &mut Vec<PointerEvent>) {
        release_contacts(mem::take(&mut self.contacts), events);
    }

    /// `add_contact`. A contact with the same id in `old_contacts` is moved to the tracked
    /// list as `Update` (the first match wins). Otherwise a new contact starts with `Down`.
    fn add_contact(
        &mut self,
        old_contacts: &mut Vec<Contact>,
        id: u32,
        x: i32,
        y: i32,
        events: &mut Vec<PointerEvent>,
    ) {
        let mut flags = POINTER_MESSAGE_FLAG_INRANGE
            | POINTER_MESSAGE_FLAG_INCONTACT
            | POINTER_MESSAGE_FLAG_CONFIDENCE;
        let (kind, contact) = match old_contacts.iter().position(|contact| contact.id == id) {
            Some(index) => {
                let mut contact = old_contacts.remove(index);
                contact.x = x;
                contact.y = y;
                (PointerKind::Update, contact)
            }
            None => {
                flags |= POINTER_MESSAGE_FLAG_NEW;
                (PointerKind::Down, Contact { id, x, y })
            }
        };
        events.push(PointerEvent::new(kind, &contact, flags));
        self.contacts.push(contact);
    }
}

/// `release_contacts`: sends `WM_POINTERUP` for each contact, in list order.
fn release_contacts(contacts: Vec<Contact>, events: &mut Vec<PointerEvent>) {
    for contact in contacts {
        events.push(PointerEvent::new(
            PointerKind::Up,
            &contact,
            POINTER_MESSAGE_FLAG_CONFIDENCE,
        ));
    }
}

impl PointerEvent {
    fn new(kind: PointerKind, contact: &Contact, flags: u16) -> Self {
        Self {
            kind,
            id: contact.id as u16,
            flags,
            x: contact.x as u16,
            y: contact.y as u16,
        }
    }
}
