use std::any::Any;
use std::fmt::Debug;

pub trait AnyMsg: Any + 'static + Debug + AsAny {}

pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
}

impl<T: AnyMsg> AsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl dyn AnyMsg {
    pub fn as_msg<T: AnyMsg>(&self) -> Option<&T> {
        self.as_any().downcast_ref::<T>()
    }
}
