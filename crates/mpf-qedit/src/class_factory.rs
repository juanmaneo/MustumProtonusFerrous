use crate::com::{
    is_equal_guid, Guid, Hresult, CLSID_AMTIMELINE, CLSID_MEDIADET, CLSID_NULLRENDERER,
    CLSID_SAMPLEGRABBER, E_NOINTERFACE, IID_ICLASSFACTORY, IID_IUNKNOWN, S_OK,
};

/// The classes in the `object_creation` table of `dlls/qedit/main.c`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClassId {
    AmTimeline,
    MediaDet,
    NullRenderer,
    SampleGrabber,
}

/// The `object_creation` table in table order. [`ClassId::from_clsid`] returns the first match.
const OBJECT_CREATION: [ClassId; 4] = [
    ClassId::AmTimeline,
    ClassId::MediaDet,
    ClassId::NullRenderer,
    ClassId::SampleGrabber,
];

impl ClassId {
    /// The CLSID that the table maps to this class.
    pub fn clsid(self) -> &'static Guid {
        match self {
            ClassId::AmTimeline => &CLSID_AMTIMELINE,
            ClassId::MediaDet => &CLSID_MEDIADET,
            ClassId::NullRenderer => &CLSID_NULLRENDERER,
            ClassId::SampleGrabber => &CLSID_SAMPLEGRABBER,
        }
    }

    /// The class for `clsid`, or `None` when the table has no entry for it.
    pub fn from_clsid(clsid: &Guid) -> Option<ClassId> {
        OBJECT_CREATION
            .into_iter()
            .find(|class| is_equal_guid(class.clsid(), clsid))
    }
}

/// The `IClassFactoryImpl` object: a reference count and the class it creates. The C code frees
/// the object when the count reaches 0. The port has nothing to free, so the owner drops it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ClassFactory {
    ref_count: u32,
    class: ClassId,
}

impl ClassFactory {
    /// A new factory with a count of 1, as `DllGetClassObject` sets it.
    pub fn new(class: ClassId) -> ClassFactory {
        ClassFactory {
            ref_count: 1,
            class,
        }
    }

    pub fn ref_count(&self) -> u32 {
        self.ref_count
    }

    pub fn class(&self) -> ClassId {
        self.class
    }

    /// `DSCF_QueryInterface`. `IUnknown` and `IClassFactory` succeed and add a reference, because
    /// the C code calls `IClassFactory_AddRef` on the object it returns. Any other IID fails with
    /// `E_NOINTERFACE` and leaves the count alone.
    pub fn query_interface(&mut self, iid: &Guid) -> Hresult {
        if is_equal_guid(iid, &IID_IUNKNOWN) || is_equal_guid(iid, &IID_ICLASSFACTORY) {
            self.add_ref();
            return S_OK;
        }
        E_NOINTERFACE
    }

    /// `DSCF_AddRef`. Returns the new count.
    pub fn add_ref(&mut self) -> u32 {
        self.ref_count += 1;
        self.ref_count
    }

    /// `DSCF_Release`. Returns the new count.
    pub fn release(&mut self) -> u32 {
        self.ref_count -= 1;
        self.ref_count
    }

    /// `DSCF_CreateInstance`. A non-NULL outer is only allowed for `IUnknown`. Otherwise `create`
    /// runs with the class. It stands for the class's `create_instance`, the `QueryInterface` on
    /// the new object for `riid`, and the `Release` of that object. None of those are ported.
    pub fn create_instance(
        &self,
        has_outer: bool,
        iid: &Guid,
        create: impl FnOnce(ClassId) -> Hresult,
    ) -> Hresult {
        if has_outer && !is_equal_guid(&IID_IUNKNOWN, iid) {
            return E_NOINTERFACE;
        }
        create(self.class)
    }

    /// `DSCF_LockServer`. A `FIXME` stub that returns `S_OK` and has no effect.
    pub fn lock_server(&self, _dolock: bool) -> Hresult {
        S_OK
    }
}
