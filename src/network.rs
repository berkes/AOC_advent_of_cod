struct Network {
    instructions: String,
    nodes: Vec<Node>,
}
impl Network {
    const FIRST_ID: &str = "AAA";
    const LAST_ID: &str = "ZZZ";

    fn steps(&self, instructions: &str) -> Vec<String> {
        let mut steps = vec![];
        let mut current_node = self.nodes.iter().find(|n| n.id == Self::FIRST_ID).unwrap();
        steps.push(Self::FIRST_ID.to_string());

        let mut iter = instructions.chars().cycle();

        loop {
            if let Some(dir) = iter.next() {
                let current_id = match dir {
                    'R' => current_node.right.clone(),
                    'L' => current_node.left.clone(),
                    _ => panic!("Invalid instruction"),
                }
                .unwrap();

                steps.push(current_id.clone());
                // stop if we have the ID of the last node
                if current_id == Self::LAST_ID {
                    break;
                }

                current_node = self.nodes.iter().find(|n| n.id == current_id).unwrap();
            }
        }

        steps
    }
}

impl From<&str> for Network {
    fn from(input: &str) -> Self {
        let instructions = input.lines().next().unwrap().to_string();
        let nodes = input
            .lines()
            .skip(2)
            .map(|line| {
                let mut parts = line.split(" = ");
                let id = parts.next().unwrap().trim().to_string();

                let connections = parts.next().unwrap().trim_matches(|c| c == '(' || c == ')');

                let mut connections = connections.split(", ");
                let left = connections.next().unwrap().to_string();
                let right = connections.next().unwrap().to_string();

                Node {
                    id,
                    left: Some(left),
                    right: Some(right),
                }
            })
            .collect();
        Network {
            instructions,
            nodes,
        }
    }
}

#[derive(Debug)]
struct Node {
    id: String,
    left: Option<String>,
    right: Option<String>,
}

pub fn count_steps(input: &str) -> usize {
    let network = Network::from(input);
    let steps = network.steps(&network.instructions);

    steps.len() - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn network_instructions_test() {
        let input = r#"RL

            AAA = (BBB, CCC)"#;
        assert_eq!(Network::from(input).instructions, "RL");
    }

    #[test]
    fn network_nodes_test() {
        let input = r#"RL

            AAA = (BBB, CCC)
            BBB = (BBB, ZZZ)"#;
        assert_eq!(Network::from(input).nodes.len(), 2);
    }

    #[test]
    fn network_steps_test() {
        let input = r#"RL

            AAA = (BBB, CCC)
            BBB = (CCC, DDD)
            CCC = (ZZZ, BBB)"#;
        assert_eq!(Network::from(input).steps("RL"), vec!["AAA", "CCC", "ZZZ"]);
    }

    #[test]
    fn network_repeats_test() {
        let input = r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"#;

        assert_eq!(
            Network::from(input).steps("LLR"),
            vec!["AAA", "BBB", "AAA", "BBB", "AAA", "BBB", "ZZZ"]
        );
    }
}
