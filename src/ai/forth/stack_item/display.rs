use std::fmt;

use crate::sandbox::new_ai::forth::StackItem;

impl fmt::Display for StackItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StackItem::Int(i) => write!(f, "{}", i),
            StackItem::True => write!(f, "True"),
            StackItem::False => write!(f, "False"),
            StackItem::Coord { x, y } => write!(f, "Coord[{x}:{y}]"),
            StackItem::EntityId(eid) => write!(f, "{}", eid),
            StackItem::Option(stack_item) => write!(f, "Some({stack_item})"),
            StackItem::String(s) => write!(f, "“{}”", s),
            StackItem::Table(table_rc) => {
                // Borrow the table's map.
                let inner_map = &table_rc.map;

                let mut table_iter = inner_map.iter();
                write!(f, "{{",)?;
                if let Some(first) = table_iter.next() {
                    write!(f, "{}: {}", first.0, first.1)?;
                    for (key, value) in table_iter {
                        write!(f, ", {}: {}", key, value)?;
                    }
                }
                // Close the object.
                write!(f, "}}")
            }
        }
    }
}
