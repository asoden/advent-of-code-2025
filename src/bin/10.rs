advent_of_code::solution!(10);

use rand::prelude::*;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

const POPULATION_SIZE: usize = 1000;
const RANDOM_RANGE_PART1: u64 = 2;
const RANDOM_RANGE_PART2: u64 = 280;
const WRONG_FACTOR: u64 = 10000;
const GENERATIONS: usize = 1000;

#[derive(Debug, Clone)]
struct Organism {
    fitness: u64,
    genome: Vec<u64>,
}
impl Organism {
    #[inline]
    fn calculate_fitness(&mut self, operator: impl Fn(&[u64]) -> u64) {
        self.fitness = operator(&self.genome);
    }

    fn reproduce(&self, spouse: &Organism, max_range: u64) -> Organism {
        let mut rng = rand::rng();
        let mut new_genome = Vec::with_capacity(self.genome.len());
        for (parent1, parent2) in self.genome.iter().zip(spouse.genome.iter()) {
            let p: u32 = rng.random_range(0..100);
            if p < 40 {
                new_genome.push(*parent1);
            } else if p < 80 {
                new_genome.push(*parent2);
            } else {
                new_genome.push(rng.random_range(0..max_range));
            }
        }

        Organism {
            fitness: u64::MAX,
            genome: new_genome,
        }
    }
}

#[derive(Debug, Clone)]
struct Schematic {
    goals: Vec<u64>,
    wirings: Vec<Vec<u64>>,
}

#[inline]
fn create_genome(genome_size: usize, max_range: u64) -> Vec<u64> {
    let mut genome = Vec::with_capacity(genome_size);
    let mut rng = rand::rng();

    for _ in 0..genome_size {
        // genome = vec![0, 0, 1, 0, 0, 0];
        genome.push(rng.random_range(0..max_range));
    }
    genome
}

fn create_generation(schematic: &Schematic, max_range: u64) -> Vec<Organism> {
    let mut population = Vec::with_capacity(POPULATION_SIZE);

    for _ in 0..POPULATION_SIZE {
        let genome = create_genome(schematic.wirings.len(), max_range);
        population.push(Organism {
            fitness: u64::MAX,
            genome,
        });
    }
    population
}

fn calculate_part1_fitness(schematic: &Schematic, organism: &mut Organism) {
    let genome_total = organism.genome.iter().sum::<u64>();
    organism.calculate_fitness(|genome| {
        let off_factor = schematic
            .goals
            .iter()
            .enumerate()
            .map(|(i, &goal)| {
                let mut sub_total = genome
                    .iter()
                    .zip(schematic.wirings.iter())
                    .filter(|(_, wires)| wires.contains(&(i as u64)))
                    .map(|(gene, wire)| {
                        // dbg!(wire);
                        // dbg!(gene);
                        gene
                    })
                    .sum::<u64>();

                // dbg!(sub_total);
                // if sub_total == 0 {
                //     return 1_000_000_000;
                // }

                // let on_count = genome.iter().filter(|g| **g == 1).count() as u64;

                // dbg!(genome);
                if sub_total % 2 == goal {
                    0
                } else {
                    WRONG_FACTOR
                }
            })
            .sum::<u64>();

        let fitness = (off_factor * WRONG_FACTOR) + (genome_total);
        if fitness == 0 { u64::MAX } else { fitness }
    });
}

fn calculate_part2_fitness(schematic: &Schematic, organism: &mut Organism) {
    let genome_total = organism.genome.iter().sum::<u64>();
    organism.calculate_fitness(|genome| {
        let off_factor = schematic
            .goals
            .iter()
            .enumerate()
            .map(|(i, &goal)| {
                let mut sub_total = genome
                    .iter()
                    .zip(schematic.wirings.iter())
                    .filter(|(_, wires)| wires.contains(&(i as u64)))
                    .map(|(gene, wire)| {
                        // dbg!(wire);
                        // dbg!(gene);
                        gene
                    })
                    .sum::<u64>();

                // dbg!(sub_total);
                // if sub_total == 0 {
                //     return 1_000_000_000;
                // }

                // let on_count = genome.iter().filter(|g| **g == 1).count() as u64;

                // dbg!(genome);
                if sub_total == goal {
                    0
                } else {
                    WRONG_FACTOR
                }
            })
            .sum::<u64>();

        let fitness = (off_factor * WRONG_FACTOR) + (genome_total);
        if fitness == 0 { u64::MAX } else { fitness }
    });
}

fn parse(input: &str, part1: bool) -> Vec<Schematic> {
    input
        .lines()
        .map(|line| {
            let (state, rest) = line.split_once(' ').unwrap();
            let mut wirings_str = rest.split_ascii_whitespace().collect::<Vec<&str>>();
            let joltages = wirings_str.remove(wirings_str.len() - 1);

            let lights = state[1..state.len() - 1]
                .bytes()
                .map(|b| match b {
                    b'#' => 1,
                    _ => 0,
                })
                .collect();

            let wirings = wirings_str
                .into_iter()
                .map(|wiring| {
                    wiring[1..wiring.len() - 1]
                        .split(',')
                        .map(|val| val.parse().unwrap())
                        .collect()
                })
                .collect();

            let joltages = joltages[1..joltages.len() -1].split(',').map(|jolt| {
                jolt.parse().unwrap()
            }).collect();

            Schematic {
                goals: if part1 {lights} else {joltages},
                wirings,
            }
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let schematics = parse(input, true);

    // let schematics = [schematics[0].clone()];

    let total = schematics
        .par_iter()
        .map(|schematic| {
            let mut rng = rand::rng();
            let mut generation_count = 0;
            let mut generation = create_generation(schematic, RANDOM_RANGE_PART1);

            // dbg!(&generation);

            for organism in generation.iter_mut() {
                calculate_part2_fitness(&schematics[0], organism);
            }

            generation.sort_by_key(|organism| organism.fitness);
            for _ in 0..GENERATIONS {
                let mut new_generation = Vec::with_capacity(POPULATION_SIZE);

                // THE CREME RISES TO THE TOP! YEAH!!!!
                // take the top 10% unaltered
                let cream_of_the_crop = (10 * POPULATION_SIZE) / 100;
                generation
                    .iter()
                    .take(cream_of_the_crop)
                    .for_each(|organism| {
                        new_generation.push(organism.clone());
                    });

                // from the fittest half of the population reproduce
                let rest = (90 * POPULATION_SIZE) / 100;
                for _ in 0..rest {
                    let idx = rng.random_range(0..POPULATION_SIZE / 2);
                    let parent1 = &generation[idx];
                    let idx = rng.random_range(0..POPULATION_SIZE / 2);
                    let parent2 = &generation[idx];
                    let offspring = parent1.reproduce(parent2, RANDOM_RANGE_PART1);
                    new_generation.push(offspring);
                }

                // calculate fitness
                for organism in new_generation.iter_mut() {
                    calculate_part1_fitness(schematic, organism);
                }
                generation = new_generation;

                // sort by fitness
                generation.sort_by_key(|organism| organism.fitness);

                // println!(
                //     "Generation: {} \tGenome: {:?}\tFitness: {}",
                //     generation_count, generation[0].genome, generation[0].fitness
                // );

                generation_count += 1;
            }

            // dbg!(&generation);

            generation[0].genome.iter().filter(|&&i| i != 0).count() as u64
        })
        .sum();

    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let schematics = parse(input, false);
    // let schematics = [schematics[0].clone()];

    let total = schematics
        .par_iter()
        .map(|schematic| {
            let mut rng = rand::rng();
            let mut generation_count = 0;
            let mut generation = create_generation(schematic, RANDOM_RANGE_PART2);

            // dbg!(&generation);

            for organism in generation.iter_mut() {
                calculate_part1_fitness(&schematics[0], organism);
            }

            generation.sort_by_key(|organism| organism.fitness);
            for _ in 0..GENERATIONS {
                let mut new_generation = Vec::with_capacity(generation.len());

                // THE CREME RISES TO THE TOP! YEAH!!!!
                // take the top 10% unaltered
                let cream_of_the_crop = (10 * POPULATION_SIZE) / 100;
                generation
                    .iter()
                    .take(cream_of_the_crop)
                    .for_each(|organism| {
                        new_generation.push(organism.clone());
                    });

                // from the fittest half of the population reproduce
                let rest = (90 * POPULATION_SIZE) / 100;
                for _ in 0..rest {
                    let idx = rng.random_range(0..POPULATION_SIZE / 2);
                    let parent1 = &generation[idx];
                    let idx = rng.random_range(0..POPULATION_SIZE / 2);
                    let parent2 = &generation[idx];
                    let offspring = parent1.reproduce(parent2, RANDOM_RANGE_PART2);
                    new_generation.push(offspring);
                }

                // calculate fitness
                for organism in new_generation.iter_mut() {
                    calculate_part2_fitness(schematic, organism);
                }
                generation = new_generation;

                // sort by fitness
                generation.sort_by_key(|organism| organism.fitness);

                // println!(
                //     "Generation: {} \tGenome: {:?}\tFitness: {}",
                //     generation_count, generation[0].genome, generation[0].fitness
                // );

                generation_count += 1;
            }

            // dbg!(&generation);

            generation[0].genome.iter().sum::<u64>()
        })
        .sum();

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(33));
    }
}
