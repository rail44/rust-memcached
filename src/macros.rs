#![macro_escape]

#[macro_export]
macro_rules! memcached_client(
  ($($server: expr),+) => {
    {
      let client = Client::new();
      $(
        let server_split: Vec<&str> = $server.splitn(':', 1).collect();
        let host = *server_split.get(0);
        let port: uint = from_str(*server_split.get(1)).unwrap();
        let server = Server::new(host, port);
        client.add_server(&server);
      )+
      client
    }
  }
)
