use std::{collections::HashMap, fmt::Debug};

#[derive(Debug)]
struct Node {
    identifier: String,
    children: Vec<Node>,
}

fn main() {
    // Start at start
    // Pick a path that generates a prefix that doesn't exist

    let input = std::fs::read_to_string("src/input.txt").unwrap();
    let mut connections: HashMap<&str, Vec<String>> = HashMap::new();
    for line in input.split('\n') {
        let mut split = line.split('-');
        let a = split.next().unwrap();
        let b = split.next().unwrap();
        if connections.contains_key(a) {
            connections.get_mut(a).unwrap().push(b.to_owned());
        } else {
            connections.insert(a, vec![b.to_owned()]);
        }

        if b != "end" {
            if connections.contains_key(b) {
                connections.get_mut(b).unwrap().push(a.to_owned());
            } else {
                connections.insert(b, vec![a.to_owned()]);
            }
        }
    }

    let mut path_count = 0;

    fn str_to_node(
        identifier: String,
        path: Vec<String>,
        connections: &HashMap<&str, Vec<String>>,
        path_count: &mut i32,
    ) -> Option<Node> {
        let there_are_duplicates = path.iter().any(|v| {
            // v is lowercase and v is more than once in path
            &v.to_lowercase() == v && path.iter().filter(|v2| v2 == &v).count() > 1
        });

        if there_are_duplicates
            && identifier.to_lowercase() == identifier
            && path.contains(&identifier)
        {
            return None;
        }

        // println!("id {:?}", identifier);
        let default = Vec::new();
        let children = connections.get(&identifier[..]).unwrap_or(&default);
        let children = if identifier == "end" {
            Vec::new()
        } else {
            children
                .iter()
                .filter(|id| {
                    if id.to_uppercase() == **id {
                        true
                    } else if **id == "end" {
                        true
                    } else if **id == "start" {
                        false
                    } else {
                        let does_not_exist_in_path = !path.iter().any(|s| s == *id);
                        let there_are_no_duplicates = !path.iter().any(|v| {
                            // v is lowercase and v is more than once in path
                            &v.to_lowercase() == v && path.iter().filter(|v2| v2 == &v).count() > 1
                        });

                        // println!("🚧");
                        // println!("{:?}", path.join("-"));
                        // println!("does_not_exist_in_path {:?}", does_not_exist_in_path);
                        // println!("there_are_no_duplicates {:?}", there_are_no_duplicates);

                        does_not_exist_in_path || there_are_no_duplicates
                    }
                })
                .map(|id| {
                    let mut path = path.clone();
                    path.push(identifier.clone());
                    str_to_node(id.clone(), path, connections, path_count)
                })
                .filter(|node| !node.is_none())
                .map(|node| node.unwrap())
                .collect()
        };

        // println!("children {:?}", children);

        if identifier == "end" {
            *path_count += 1;

            // println!("{:?}", path.join("-"));
            // println!("{:?}", path_count)
        }

        Some(Node {
            identifier: identifier.to_owned(),
            children, // connections filter, if uppercase -> true, if end -> true, if lowercase, traverse parent to make sure it is not there
        })
    }

    let tree = str_to_node(
        "start".to_owned(),
        Vec::new(),
        &connections,
        &mut path_count,
    );

    // let root_node = Node {
    //     identifier: "start".to_owned(),
    //     children: connections.get("start").map(|str| str_to_node(str)),
    // };

    // Generate all possible paths

    //       start
    //          A                        | b
    //      c | end | b               d    |   A |       end
    //    A             d | end              c   |   end
    // end | b
    //      d | end | A
    //                 end

    println!("Hello, world! {:?}", path_count);
}
