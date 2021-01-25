let addon = require('./dist');
const assert = require('assert');

let obj = new addon.MyObject(10);
assert.equal(obj.value, 10, "verify value works");
assert.equal(obj.plusOne(), 11);