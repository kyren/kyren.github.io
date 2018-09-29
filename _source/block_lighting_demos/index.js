// For more comments about what's going on here, check out the `hello_world`
// example.
import('./block_lighting_demos')
  .then(block_lighting_demos => block_lighting_demos.draw())
  .catch(console.error);
