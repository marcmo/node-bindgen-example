let addon = require('./dist');
const assert = require('assert');

async function run() {
  let obj = new addon.Session(10);
  obj.start((event) => {
    // This is call back in thread
    console.log(event);
  });
  // await new Promise(r => setTimeout(r, 500));
  obj.doRequestOne();
  obj.doRequestTwo();
  setTimeout(() => {
    console.log(`JS is finished ${obj}`);
    obj.doShutdown();
  }, 5000);
}
run();
console.log("after run");
// assert.equal(obj.value, 10, "verify value works");
// assert.equal(obj.plusOne(), 11);