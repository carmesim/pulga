mod exofetch;

fn main() 
{
    let user_data = exofetch::get_user_data();
    println!("{:?}", user_data);
}