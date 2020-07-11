use ::warp::Filter;
use ::std::net::SocketAddr;

#[tokio::main]
async fn main() {
  let filter = ::warp::any().map(|| {
    return "Hello World";
  });
  let addr: SocketAddr = "[::]:5000".parse().unwrap();
  ::warp::serve(filter).run(addr).await;
}
