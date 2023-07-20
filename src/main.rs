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

struct TreeIter<T> {
    item: Option<T>,
    current_child: Option<Box<TreeIter<T>>>,
    children: std::iter::Peekable<std::vec::IntoIter<Tree<T>>>
}

// depth-first items, and whether the branch is the last branch of its parent
impl<T> std::iter::Iterator for TreeIter<T> {
    type Item = (Vec<bool>, T);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.item.take() { // start a branch
            Some((vec![], item))
        } else if let Some(current_child) = self.current_child.as_mut() {
            if let Some((mut depth, item)) = current_child.next() {
                depth.push(self.children.peek().is_none());
                Some((depth, item))
            } else {
                self.current_child = None;
                self.next()
            }
        } else {
            let next_child = self.children.next();
            if let Some(next_child) = next_child {
                self.current_child = Some(Box::new(next_child.into_iter()));
                self.next()
            } else {
                None
            }
        }
    }
}

impl<T> IntoIterator for Tree<T> {
    type IntoIter = TreeIter<T>;
    type Item = <TreeIter<T> as Iterator>::Item;

    fn into_iter(self) -> Self::IntoIter {
        let child_iter = self.children.into_iter().peekable();
        TreeIter {
            item: Some(self.value),
            children: child_iter,
            current_child: None,
        }
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