use rec_rsys::models::ItemResult;
use stefn::AppError;

struct MoviesDataset;

pub async fn get_movies(
    prod_id: Option<String>,
    target_id: Option<String>,
    quantity: u8,
    dataset: MoviesDataset
) -> Result<Vec<ItemResult<f64>>, AppError>{
    // the dataset with all movies with their vectors.
    // if we have a target and/or product we could filter accros
    // this dataset. for example if a vendor does't have some movies
    // availables

    Ok(vec![])
}
