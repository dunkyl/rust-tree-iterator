struct Tree<T> {
    value: T,
    children: Vec<Tree<T>>,
}

macro_rules! tr {
    ($value:expr, [$($children:expr),* $(,)?]) => {
        Tree { value: $value, children: vec![$($children),*] }
    };
    ($value:expr) => { tr!($value, []) };
}

type ChildIter<T> = std::iter::Peekable<std::vec::IntoIter<Tree<T>>>;

enum TreeIter<T> {
    Exhausted,
    Value(T, ChildIter<T>),
    Children(Box<TreeIter<T>>, ChildIter<T>)
}

impl<T> TreeIter<T> {
    fn from_children(mut children: ChildIter<T>) -> Self {
        if let Some(next_child) = children.next() {
            Self::Children(Box::new(next_child.into_iter()), children)
        } else {
            Self::Exhausted
        }
    }
}

// depth-first items, and whether the branch is the last branch of its parent
impl<T> std::iter::Iterator for TreeIter<T> {
    type Item = (Vec<bool>, T);

    fn next(&mut self) -> Option<Self::Item> {
        match std::mem::replace(self, Self::Exhausted) {
            Self::Value(value, children) => {
                *self = Self::from_children(children);
                Some((vec![], value))
            }
            // pattern if-let guard when stable
            Self::Children(mut next_child, mut children) => {
                if let Some((mut depth, item)) = next_child.next() {
                    depth.push(children.peek().is_none());
                    *self = Self::Children(next_child, children);
                    Some((depth, item))
                } else {
                    *self = Self::from_children(children);
                    self.next()
                }
            }
            Self::Exhausted => None,
        }
    }
}

impl<T> IntoIterator for Tree<T> {
    type IntoIter = TreeIter<T>;
    type Item = <TreeIter<T> as Iterator>::Item;

    fn into_iter(self) -> Self::IntoIter {
        let child_iter = self.children.into_iter().peekable();
        TreeIter::Value(self.value, child_iter)
    }
}

fn main() {

    let tree = tr!("top", [ tr!("1", []),
                            tr!("2", [ tr!("A", [ tr!("()"), ]),
                                       tr!("B", []),
                                       tr!("C", [ tr!("i"),
                                                  tr!("ii"), ]), ]),
                            tr!("3", [ tr!("x", [ tr!("α"),
                                                  tr!("β"),  ]), ]), ]);

    for (depth, item) in tree {
        if let Some((last_branch, parent_branches)) = depth.split_first() {
            let prefix = parent_branches.iter().rev()
                // indent and extend to match parents' branches
                .fold(String::new(), |p, n| { 
                    p + if *n { "   " } else { "│  " }
                });
            let last_prefix = if *last_branch { "└─ " } else { "├─ " };
            println!("{}{}{}", prefix, last_prefix, item);
        } else {
            // Top of tree (no parents)
            println!("{}", item);
        }
    }
}