let addon = require('./dist');
const assert = require('assert');

async function run() {
  let obj = new addon.Session(10);
  obj.start();
  await new Promise(r => setTimeout(r, 500));
  obj.doRequest();
}
run();
// assert.equal(obj.value, 10, "verify value works");
// assert.equal(obj.plusOne(), 11);