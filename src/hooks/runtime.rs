    use std::any::Any;

    pub struct Hooks {
        states: Vec<Box<dyn Any>>,
        current_idx: usize,
        effect_deps: Vec<Option<Box<dyn Any>>>,
        effect_idx: usize,
        memos: Vec<(Option<Box<dyn Any>>, Box<dyn Any>)>,
        memo_idx: usize,
    }

    impl Default for Hooks {
        fn default() -> Self {
            Self::new()
        }
    }

    impl Hooks {
        pub fn new() -> Self {
            Self {
                states: Vec::new(),
                current_idx: 0,
                effect_deps: Vec::new(),
                effect_idx: 0,
                memos: Vec::new(),
                memo_idx: 0,
            }
        }

        pub fn reset(&mut self) {
            self.current_idx = 0;
            self.effect_idx = 0;
            self.memo_idx = 0;
        }

        pub fn check_effect<D: PartialEq + Clone + 'static>(&mut self, deps: Option<D>) -> bool {
            let idx = self.effect_idx;
            self.effect_idx += 1;

            if idx >= self.effect_deps.len() {
                self.effect_deps
                    .push(deps.map(|d| Box::new(d) as Box<dyn Any>));
                true
            } else {
                let mut changed = false;
                if let Some(new_deps) = &deps {
                    if let Some(old_deps) = &self.effect_deps[idx] {
                        if let Some(old_deps) = old_deps.downcast_ref::<D>() {
                            changed = old_deps != new_deps;
                            if changed { println!("check_effect: deps changed ({} != {})", "old", "new"); }
                        } else {
                            println!("check_effect: downcast failed!");
                            changed = true;
                        }
                    } else {
                        println!("check_effect: old_deps is None!");
                        changed = true;
                    }
                } else {
                    changed = true;
                }

                if changed {
                    self.effect_deps[idx] = deps.map(|d| Box::new(d) as Box<dyn Any>);
                }
                changed
            }
        }

        pub fn check_memo<D: PartialEq + Clone + 'static, T: Clone + 'static, F: FnOnce() -> T>(
            &mut self,
            deps: Option<D>,
            calc: F,
        ) -> T {
            let idx = self.memo_idx;
            self.memo_idx += 1;

            if idx >= self.memos.len() {
                let val = calc();
                self.memos.push((
                    deps.map(|d| Box::new(d) as Box<dyn Any>),
                    Box::new(val.clone()),
                ));
                val
            } else {
                let mut changed = false;
                if let Some(new_deps) = &deps {
                    if let Some(old_deps) = &self.memos[idx].0 {
                        if let Some(old_deps) = old_deps.downcast_ref::<D>() {
                            changed = old_deps != new_deps;
                        } else {
                            changed = true;
                        }
                    } else {
                        changed = true;
                    }
                } else {
                    changed = true;
                }

                if changed {
                    let val = calc();
                    self.memos[idx] = (
                        deps.map(|d| Box::new(d) as Box<dyn Any>),
                        Box::new(val.clone()),
                    );
                    val
                } else {
                    self.memos[idx].1.downcast_ref::<T>().unwrap().clone()
                }
            }
        }

        pub fn use_state<T: Clone + 'static>(&mut self, init: T) -> (T, usize) {
            if self.current_idx >= self.states.len() {
                self.states.push(Box::new(init.clone()));
            }

            let state_ref = self.states[self.current_idx].downcast_ref::<T>().unwrap();
            let val = state_ref.clone();

            let idx = self.current_idx;
            self.current_idx += 1;

            (val, idx)
        }

        pub fn set_state<T: 'static>(&mut self, idx: usize, new_val: T) {
            if let Some(state) = self.states[idx].downcast_mut::<T>() {
                *state = new_val;
            }
        }
    }
