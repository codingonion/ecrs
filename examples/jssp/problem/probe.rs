use ecrs::ga::{individual::IndividualTrait, Probe};
use itertools::Itertools;
use log::info;

use super::individual::JsspIndividual;

pub(crate) struct JsspProbe {}

impl JsspProbe {
    pub(crate) fn new() -> Self {
        Self {}
    }

    // TODO: This has either been not working as expected or the solver runs so bad.
    // TODO: Verify whether the diversity is better on other problems
    fn estimate_pop_diversity(population: &[JsspIndividual]) -> f64 {
        population
            .iter()
            .map(|idv| (idv.chromosome().iter().product::<f64>() * 100_000f64) as usize)
            .unique()
            .count() as f64
            / population.len() as f64
    }
}

impl Probe<JsspIndividual> for JsspProbe {
    // CSV OUTLINE:
    // diversity,<generation>,<total_duration>,<population_size>,<diversity>
    // newbest,<generation>,<total_duration>,<fitness>
    // bestingen,<generation>,<total_duration>,<fitness>
    // popgentime,<time>
    // iterinfo,<generation>,<eval_time>,<sel_time>,<cross_time>,<mut_time>,<repl_time>,<iter_time>

    #[inline]
    fn on_start(&mut self, _metadata: &ecrs::ga::GAMetadata) {
        // Writing csv header to each file
        info!(target: "diversity", "event_name,generation,total_duration,population_size,diversity");
        info!(target: "popgentime", "event_name,time");
        info!(target: "newbest", "event_name,generation,total_duration,fitness");
        info!(target: "bestingen", "event_name,generation,total_duration,fitness");
        info!(target: "iterinfo", "event_name,eval_time,sel_time,cross_time,mut_time,repl_time,iter_time");
    }

    fn on_initial_population_created(
        &mut self,
        metadata: &ecrs::ga::GAMetadata,
        population: &[JsspIndividual],
    ) {
        // TODO: As this metric is useless right now I'm disabling it temporarily
        // let diversity = JsspProbe::estimate_pop_diversity(population);
        let diversity = 0.0;
        info!(target: "diversity", "diversity,0,0,{},{diversity}", population.len());
        info!(target: "popgentime", "popgentime,{}", metadata.pop_gen_dur.unwrap().as_millis());
    }

    fn on_new_best(&mut self, metadata: &ecrs::ga::GAMetadata, individual: &JsspIndividual) {
        info!(
            target: "newbest",
            "newbest,{},{},{}",
            metadata.generation,
            metadata.total_dur.unwrap().as_millis(),
            individual.fitness
        );
    }

    fn on_new_generation(&mut self, metadata: &ecrs::ga::GAMetadata, generation: &[JsspIndividual]) {
        // TODO: As this metric is useless right now I'm disabling it temporarily
        // let diversity = JsspProbe::estimate_pop_diversity(generation);
        let diversity = 0.0;
        info!(
            target: "diversity",
            "diversity,{},{},{},{diversity}",
            metadata.generation,
            metadata.total_dur.unwrap().as_millis(),
            generation.len()
        );
    }

    fn on_best_fit_in_generation(&mut self, metadata: &ecrs::ga::GAMetadata, individual: &JsspIndividual) {
        info!(
            target: "bestingen",
            "bestingen,{},{},{}",
            metadata.generation,
            metadata.total_dur.unwrap().as_millis(),
            individual.fitness
        );
    }

    #[inline]
    fn on_iteration_start(&mut self, _metadata: &ecrs::ga::GAMetadata) { /* defaults to noop */
    }

    #[inline]
    fn on_iteration_end(&mut self, metadata: &ecrs::ga::GAMetadata) {
        info!(target: "iterinfo", "iterinfo,{},{},{},{},{},{},{}",
            metadata.generation,
            metadata.pop_eval_dur.unwrap().as_millis(),
            metadata.selection_dur.unwrap().as_millis(),
            metadata.crossover_dur.unwrap().as_millis(),
            metadata.mutation_dur.unwrap().as_millis(),
            metadata.replacement_dur.unwrap().as_millis(),
            metadata.iteration_dur.unwrap().as_millis()
        );
    }

    #[inline]
    fn on_end(
        &mut self,
        _metadata: &ecrs::ga::GAMetadata,
        _population: &[JsspIndividual],
        _best_individual: &JsspIndividual,
    ) { /* defaults to noop */
    }
}
