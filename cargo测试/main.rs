use polars::frame::DataFrame;
use polars::prelude::{NamedFrom,Series};

fn main(){
    let s1:Series=Series::new("from vec".into(),vec![1,2,3]);
    let s2:Series=Series::new("from slice".into(),&[true,false,true]);
    let s3:Series=Series::new("from array".into(),["Mathematica","typescript","python"]);

    let data:polars::prelude::PolarsResult<DataFrame>=DataFrame::new(vec![s1.into(),s2.into(),s3.into()]);

    println!("{:?}",&data.unwrap());
}