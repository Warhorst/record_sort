use std::convert::TryFrom;

use crate::benchmark::benchmark::Benchmark;
use crate::benchmark::measurement::Measurement;
use crate::benchmark::real_data_benchmark::RealDataBenchmark;
use crate::benchmark::simple_benchmark::SimpleBenchmark;
use crate::cli::action::Action;
use crate::cli::application_error::ApplicationError;
use crate::sort::algorithm::Algorithm;
use crate::sort::sort_factory::SortFactory;
use crate::util::random_number_generator::RandomNumberGenerator;

pub struct ConsoleApplication {
    sort_factory: SortFactory,
}

impl ConsoleApplication {
    const ACTION_INDEX: usize = 1;
    const ALGORITHM_INDEX: usize = 2;

    pub fn new(sort_factory: SortFactory) -> Self {
        ConsoleApplication {
            sort_factory,
        }
    }

    pub fn run_record_sort(&self, args: Vec<String>) {
        let result = self.run(args);

        match result {
            Err(error) => println!("An error occurred during execution: {}", error),
            Ok(_) => ()
        }
    }

    fn run(&self, args: Vec<String>) -> Result<(), ApplicationError> {
        let action = Action::try_from(args.get(ConsoleApplication::ACTION_INDEX))?;

        match action {
            Action::Simple => self.simple(args),
            Action::Real => self.real(args),
            Action::Fake => unimplemented!(),
            Action::Generate => unimplemented!()
        }
    }

    fn simple(&self, args: Vec<String>) -> Result<(), ApplicationError> {
        let algorithm = Algorithm::try_from(args.get(ConsoleApplication::ALGORITHM_INDEX))?;

        Ok(
            self.print_measurements(
                SimpleBenchmark::new(RandomNumberGenerator).execute(self.sort_factory.create(algorithm))?
            )
        )
    }

    fn real(&self, args: Vec<String>) -> Result<(), ApplicationError> {
        let algorithm = Algorithm::try_from(args.get(ConsoleApplication::ALGORITHM_INDEX))?;

        Ok(
            self.print_measurements(
                RealDataBenchmark::default().execute(self.sort_factory.create(algorithm))?
            )
        )
    }

    fn print_measurements(&self, measurements: Vec<Measurement>) {
        println!("Results: ");

        for result in measurements {
            println!("Lines: {}, Duration: {} seconds", result.get_sorted_elements(), result.get_duration())
        }
    }
}