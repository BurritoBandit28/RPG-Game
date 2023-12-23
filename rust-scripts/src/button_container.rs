use std::str::FromStr;
use godot::bind::godot_api;
use godot::builtin::{NodePath, StringName, VariantArray};
use godot::engine::{INode, Input, Node, NodeExt, Timer};
use godot::log::godot_print;
use godot::prelude::{Array, Base, Gd, GodotClass};
use godot::private::You_forgot_the_attribute__godot_api;
use crate::button::UIButton;

// the purpose of this class is to manage how a controller interacts with UI buttons.
#[derive(GodotClass)]
#[class(base=Node)]
pub struct ButtonContainer{
    #[base]
    base : Base<Node>,
    #[export]
    buttons : Array<VariantArray>,
    row : usize,
    column : usize

}

#[godot_api]
impl INode for ButtonContainer {

    // init function defining defaults for this class
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            base,
            buttons : Array::new(),
            row : 0,
            column : 0
        }
    }

    // what happens each frame
    fn process(&mut self, delta: f64) {
        if self.get_interact_timer().is_stopped() {
            let input = Input::singleton();

            // save previous hovered button
            let prevr = self.row;
            let prevc = self.column;

            // hover over button above current
            if input.is_action_pressed(StringName::from_str("w").unwrap()) {
                // if already at the top, move to button on the bottom
                if self.row == 0 {
                    self.row = self.buttons.get(self.column).len() - 1;
                }
                // otherwise just move up by one index
                else {
                    self.row -= 1;
                }
            }

            //hover over button bellow current
            if input.is_action_pressed(StringName::from_str("s").unwrap()) {
                // if already at the bottom, move to the top
                if self.row == self.buttons.get(self.column).len() - 1 {
                    self.row = 0
                }
                // otherwise just move down by one index
                else {
                    self.row += 1;
                }
            }

            // hover over button of the same row one column to the left
            if input.is_action_pressed(StringName::from_str("a").unwrap()) {
                // if already on far left, move to far right
                if self.column == 0 {
                    self.column = self.buttons.len() - 1;
                }
                // otherwise just move left by one index
                else {
                    self.column -= 1;
                }

                // explained later
                self.fix_row()
            }

            // hover over button of the same row one column to the right
            if input.is_action_pressed(StringName::from_str("d").unwrap()) {
                // if already on far right, move to far left
                if self.column == self.buttons.len() - 1 {
                    self.column = 0
                }
                // otherwise just move right by one index
                else {
                    self.column += 1;
                }

                // explained later
                self.fix_row()
            }

            // get nodes
            // previous
            let mut prev: Gd<UIButton> = self.base.get_node_as::<UIButton>(self.buttons.get(prevc).get(prevr).try_to::<NodePath>().unwrap().to_string());
            // current
            let mut new: Gd<UIButton> = self.base.get_node_as::<UIButton>(self.buttons.get(self.column).get(self.row).try_to::<NodePath>().unwrap().to_string());

            // if they are the same do nothing
            if &prev == &new {
                return
            }

            // set the previous to no longer be selected, and the new one to be selected
            prev.bind_mut().set_hovered(false);
            new.bind_mut().set_hovered(true);

            // begin input timer as  to make sure one click happens and not more than the user intended
            self.get_interact_timer().start();
        }
    }
}

#[godot_api]
impl ButtonContainer {

    // this exists to make sure rows are correctly selected when moving selection left or right
    pub fn fix_row(&mut self) {
        let mut ind : usize;

        // if the index of the current row is out of the range of the current column, then assign it to the last index of that column
        if self.row > {ind = self.buttons.get(self.column).len()-1; ind} {
            self.row = ind;
        }
    }
    pub fn get_interact_timer(&mut self) -> Gd<Timer> {
        return self.base.get_node_as::<Timer>("InteractTimer");
    }
}