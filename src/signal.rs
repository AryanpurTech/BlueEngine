use std::any::Any;

/// Signal is a type used for creating delegates that can call functions after certain events happen.
/// None of Signal's internals fields are accessible for safety reasons.
/// 
/// You can't access any of the stored delegates because it's not safe and bad practice.
/// `This` takes a struct type, and `Args` takes a single type as the function parameter. 
/// `Args` can be a tuple, struct, or list.
/// 
/// Whenever you see the word This or this, it's being used as a replacement to mean self.
pub struct Signal<This: Any, Args: Sized> {
    delegates: Vec<Delegate<This, Args>>
}

/// Internal type used for storing delegates.
struct Delegate<This: Any, Args: Sized> {
    object: *mut This,
    callback: fn(&mut This, &mut Args) -> (),
}

impl<This: Any, Args: Sized> Signal<This, Args> {
    /// Use new to initialize Signal
    pub fn new() -> Signal<This, Args> {
        Signal { delegates: Vec::new() }
    }

    /// Uses this function to register new delegates to the Signal.
    pub fn add(&mut self, object: *mut This, callback: fn(&mut This, &mut Args)) {
        self.delegates.push( Delegate { object, callback } );
    }

    /// Enables you to removed registered delegates from a signal.
    pub fn remove(&mut self, object: *const This, callback: fn(&mut This, &mut Args)) {
        for x in 0..self.delegates.len() {
            if (self.delegates[x].object as *const This) == (object as *const This)
            && (self.delegates[x].callback as *const fn(&mut This, &mut Args)) == (callback as *const fn(&mut This, &mut Args)) {
                self.delegates.remove(x);
                return;
            }
        }
    }

    /// Use the function to execute all the registered delegates in the signal.
    pub fn broadcast(&mut self, parameters: &mut Args) {
        for obj in self.delegates.iter_mut() {
            let this = unsafe { &mut *(obj.object) };
            (obj.callback)(this, parameters);
        }
    }
}