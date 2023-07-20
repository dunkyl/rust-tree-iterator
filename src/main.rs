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
    children: Box<[TreeIter<T>]>,
}

// depth-first items, and whether the branch is the last branch of its parent
impl<T> std::iter::Iterator for TreeIter<T> {
    type Item = (Vec<bool>, T);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.item.take() { // start a branch
            Some((vec![], item))
        } else {
            let mut iter = self.children.iter_mut().peekable();
            // peek used to check if child is a last child
            while let Some(child) = iter.next() {
                if let Some((mut depth, item)) = child.next() {
                    depth.push(iter.peek().is_none());
                    return Some((depth, item))
                }
            }
            // All children exhausted
            None
        }
    }
}

impl<T> IntoIterator for Tree<T> {
    type IntoIter = TreeIter<T>;
    type Item = <TreeIter<T> as Iterator>::Item;

    fn into_iter(self) -> Self::IntoIter {
        let child_iter = self.children.into_iter().map(Self::into_iter).collect();
        TreeIter {
            item: Some(self.value),
            children: child_iter,
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