use std::any::Any;

/// Signal is a type used for creating delegates that can call functions after certain events happen.
/// None of Signal's internals fields are accessible for safety reasons.
/// 
/// You can't access any of the stored delegates because it's not safe and bad practice.
/// `This` takes a struct type, and `Args` takes a single type as the function parameter. 
/// `Args` can be a tuple, struct, or list.
/// 
/// Whenever you see the word This or this, it's being used as a replacement to mean self.
pub struct Signal<'a, This: Any, Args: Sized> {
    delegates: Vec<Delegate<'a, This, Args>>
}

/// Internal type used for storing delegates.
struct Delegate<'a, This: Any, Args: Sized> {
    object: &'a mut This,
    callback: fn(&mut This, &mut Args) -> (),
}

impl<'a, This: Any, Args: Sized> Signal<'a, This, Args> {
    /// Use new to initialize Signal
    pub fn new() -> Signal<'a, This, Args> {
        Signal { delegates: Vec::new() }
    }

    /// Uses this function to register new delegates to the Signal.
    pub fn add(&mut self, object: &'a mut This, callback: fn(&mut This, &mut Args) -> ()) -> () {
        self.delegates.push( Delegate { object, callback } );
    }

    /// Use the function to execute all the registered delegates in the signal.
    pub fn broadcast(&mut self, parameters: &mut Args) -> () {
        for obj in self.delegates.iter_mut() {
            (obj.callback)((*obj).object, parameters);
            
            // unsafe code from before I added lifetimes:
            /*
            let this = unsafe { 
                // This is copy-pasted logic from Any::downcast_mut_unchecked because it's locked as unstable feature...
                &mut *(obj.object.as_mut().unwrap() as *mut dyn Any as *mut This)
            };
            (obj.callback)(this, parameters);
            */
        }
    }
}