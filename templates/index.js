const js = import("./rust_handson");

js.then(js => {
  js.greet("World!");
  js.to_hash_digest("World!");
});
