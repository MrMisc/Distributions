use rand::thread_rng;
use rand::distributions::{Distribution, Uniform, Standard};
use statrs::distribution::{Gamma,Normal, Beta, Cauchy,Chi, ChiSquared, Dirac, Dirichlet, Erlang, Exp, FisherSnedecor,Geometric, Hypergeometric,InverseGamma,Laplace,LogNormal,Multinomial,MultivariateNormal,NegativeBinomial, Pareto, Poisson, StudentsT, Weibull};

fn main() {
    let mut rng = thread_rng();
    let v: Vec<f64> = Gamma::new(1.0,3.0).unwrap().sample_iter(&mut rng).take(160).collect();
    let n: Vec<f64> = Normal::new(1.0,3.0).unwrap().sample_iter(&mut rng).take(160).collect();
    // println!("Let us try, {}", distr::dgam(&1.0,&4.0,&0.1));
    dbg!(v);
    println!("Now let us have a look at a normal distribution");
    dbg!(n);

    println!("Now what about some others?");
    let mut rng = thread_rng();
    let v: Vec<f64> = Weibull::new(1.0,3.0).unwrap().sample_iter(&mut rng).take(160).collect();
    dbg!(v);
}
