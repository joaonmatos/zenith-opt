// Copyright 2020 João Nuno Matos
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

pub trait LocalOptimizationProblem: Sized {
    type Score;
    fn successors(&self) -> Vec<Self>;
    fn evaluate(&self) -> Self::Score;
}

pub fn hill_climbing_search<T: LocalOptimizationProblem<Score = impl Ord>>(
    max_iterations: usize,
    initial_state: T,
) -> T {
    let mut current = initial_state;
    for _ in 0..max_iterations {
        let best = current
            .successors()
            .into_iter()
            .max_by(|s1, s2| s1.evaluate().cmp(&s2.evaluate()));
        match best {
            None => break,
            Some(succ) => {
                if succ.evaluate() > current.evaluate() {
                    current = succ;
                } else {
                    break;
                }
            }
        }
    }
    current
}

#[cfg(test)]
mod tests {
    use super::*;
    #[derive(Debug, PartialEq, Eq)]
    struct Quadratic(i32);

    impl LocalOptimizationProblem for Quadratic {
        type Score = i32;
        fn successors(&self) -> Vec<Self> {
            vec![Quadratic(self.0 - 1), Quadratic(self.0 + 1)]
        }
        fn evaluate(&self) -> Self::Score {
            -(self.0) * (self.0 - 2) + 1
        }
    }

    #[test]
    fn test_quadratic() {
        let initial = Quadratic(-8);
        let expected = Quadratic(1);
        let actual = hill_climbing_search(30, initial);
        assert_eq!(expected, actual);
    }
}
