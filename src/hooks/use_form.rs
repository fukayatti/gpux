use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone)]
pub struct FormState<T: Clone> {
    pub values: T,
    pub errors: HashMap<String, String>,
    pub is_submitting: bool,
}

impl<T: Clone> FormState<T> {
    pub fn new(values: T) -> Self {
        Self {
            values,
            errors: HashMap::new(),
            is_submitting: false,
        }
    }
}

/// A simplified Form hook that manages form state and validation.
/// In a real gpux component, this would wrap `use_state`.
pub struct FormHelper<T: Clone> {
    state: FormState<T>,
    set_state: Rc<dyn Fn(FormState<T>)>,
    validate: Rc<dyn Fn(&T) -> HashMap<String, String>>,
}

impl<T: Clone + 'static> FormHelper<T> {
    pub fn new(
        state: FormState<T>,
        set_state: Rc<dyn Fn(FormState<T>)>,
        validate: Rc<dyn Fn(&T) -> HashMap<String, String>>,
    ) -> Self {
        Self {
            state,
            set_state,
            validate,
        }
    }

    pub fn values(&self) -> &T {
        &self.state.values
    }

    pub fn errors(&self) -> &HashMap<String, String> {
        &self.state.errors
    }

    pub fn is_submitting(&self) -> bool {
        self.state.is_submitting
    }

    pub fn set_value<F: FnOnce(&mut T)>(&self, updater: F) {
        let mut new_state = self.state.clone();
        updater(&mut new_state.values);
        
        // Validate on change
        let validation_errors = (self.validate)(&new_state.values);
        new_state.errors = validation_errors;
        
        (self.set_state)(new_state);
    }
    
    pub fn set_submitting(&self, is_submitting: bool) {
        let mut new_state = self.state.clone();
        new_state.is_submitting = is_submitting;
        (self.set_state)(new_state);
    }
}
